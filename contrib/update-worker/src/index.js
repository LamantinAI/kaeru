// kaeru's update-check endpoint (#99): the same release listing the daemon
// used to fetch from GitHub directly, served through here so that running
// installs can be counted per day without any identifier ever leaving them.
//
// What is stored: per UTC day, how many distinct installs asked, split by the
// version / OS / install channel / environment the daemon names in its
// User-Agent. Distinctness within a day comes from a salted SHA-256 of
// IP + User-Agent; the salt is random per day and deleted the next day, as
// are the hashes. No IP, no hash and no salt survives the day.

const GITHUB = "https://api.github.com/repos/LamantinAI/kaeru/releases?per_page=20";
const CACHE_SECONDS = 3600;

// kaeru/0.7.6 (linux-x86_64; installer; host)
const UA = /^kaeru\/([0-9A-Za-z.+-]{1,32}) \(([a-z0-9_-]{1,32}); ([a-z]{1,16}); ([a-z]{1,16})\)$/;

export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    if (request.method !== "GET" || url.pathname !== "/v1/releases") {
      return new Response("not found\n", { status: 404 });
    }
    // Counting must never cost the caller anything: it runs after the
    // response, and any failure in it is swallowed.
    ctx.waitUntil(record(request, env).catch(() => {}));
    return releases(env, ctx);
  },

  async scheduled(_event, env) {
    const today = utcDay(new Date());
    await env.DB.batch([
      env.DB.prepare("DELETE FROM salt WHERE day < ?").bind(today),
      env.DB.prepare("DELETE FROM seen WHERE day < ?").bind(today),
    ]);
  },
};

async function releases(env, ctx) {
  const cache = caches.default;
  const key = new Request(GITHUB);
  const hit = await cache.match(key);
  if (hit) return hit;

  const headers = { "User-Agent": "kaeru-updates", Accept: "application/vnd.github+json" };
  if (env.GITHUB_TOKEN) headers.Authorization = `Bearer ${env.GITHUB_TOKEN}`;
  const upstream = await fetch(GITHUB, { headers });
  if (!upstream.ok) {
    // The daemon treats any failure as "no answer" and falls back to GitHub
    // itself, so a bad upstream is passed on rather than papered over.
    return new Response("upstream unavailable\n", { status: 502 });
  }
  const body = await upstream.text();
  const response = new Response(body, {
    headers: {
      "Content-Type": "application/json",
      "Cache-Control": `public, max-age=${CACHE_SECONDS}`,
    },
  });
  ctx.waitUntil(cache.put(key, response.clone()));
  return response;
}

async function record(request, env) {
  const ua = request.headers.get("User-Agent") || "";
  const m = UA.exec(ua);
  if (!m) return; // not a kaeru daemon — served, not counted
  const [, version, os, channel, environment] = m;

  const day = utcDay(new Date());
  const salt = await saltFor(env, day);
  const ip = request.headers.get("CF-Connecting-IP") || "";
  const hash = await sha256(`${salt}|${ip}|${ua}`);

  const first = await env.DB.prepare("INSERT OR IGNORE INTO seen (day, hash) VALUES (?, ?)")
    .bind(day, hash)
    .run();
  if (first.meta.changes !== 1) return; // already counted today

  await env.DB.prepare(
    `INSERT INTO daily (day, version, os, channel, env, installs) VALUES (?, ?, ?, ?, ?, 1)
     ON CONFLICT (day, version, os, channel, env) DO UPDATE SET installs = installs + 1`,
  )
    .bind(day, version, os, channel, environment)
    .run();
}

async function saltFor(env, day) {
  const fresh = toHex(crypto.getRandomValues(new Uint8Array(16)));
  // First writer of the day wins; everyone else reads what it wrote.
  await env.DB.prepare("INSERT OR IGNORE INTO salt (day, value) VALUES (?, ?)").bind(day, fresh).run();
  const row = await env.DB.prepare("SELECT value FROM salt WHERE day = ?").bind(day).first();
  return row.value;
}

async function sha256(text) {
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
  return toHex(new Uint8Array(digest));
}

function toHex(bytes) {
  return [...bytes].map((b) => b.toString(16).padStart(2, "0")).join("");
}

function utcDay(date) {
  return date.toISOString().slice(0, 10);
}
