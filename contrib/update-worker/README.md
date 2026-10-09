# update-worker — the endpoint behind kaeru's daily update check

`kaeru-mcp` asks once a day whether a newer release exists (#99). It asks
here first; this Worker answers with the same public GitHub release listing
and, on the way, counts how many distinct installs asked that day. If it does
not answer, the daemon asks GitHub directly, so nothing depends on it being up.

## What is kept, and what is not

| | kept |
|---|---|
| installs per day by version, OS, channel (installer / bundle / source), environment (host / ci / container) | yes, in `daily` |
| IP address | never stored |
| per-install identifier | none exists — the daemon sends none |
| salted hash used to count an install once per day | swept the next day, with the salt |

The daemon's User-Agent is the whole disclosure:
`kaeru/0.7.6 (linux-x86_64; installer; host)`. `KAERU_MCP_UPDATE_CHECK=0`
turns the check off entirely.

## Deploy (once)

Needs a Cloudflare account; the free plan covers this many times over.

```sh
cd contrib/update-worker
npx wrangler login
npx wrangler d1 create kaeru-updates        # paste the printed id into wrangler.toml
npx wrangler d1 execute kaeru-updates --remote --file=schema.sql
npx wrangler deploy
npx wrangler secret put GITHUB_TOKEN         # optional: a read-only token avoids GitHub's
                                             # 60/hour anonymous limit on shared egress
```

Then point `updates.lamantin-ai.com` at the Worker (Workers → Settings →
Domains & Routes). If the domain's DNS is not on Cloudflare, use the
`*.workers.dev` URL instead and change `UPDATES_URL` in
`kaeru-mcp/src/update.rs` to match.

## Reading the numbers

```sh
npx wrangler d1 execute kaeru-updates --remote --command \
  "SELECT day, SUM(installs) AS installs FROM daily WHERE env = 'host'
   GROUP BY day ORDER BY day DESC LIMIT 14"

npx wrangler d1 execute kaeru-updates --remote --command \
  "SELECT version, SUM(installs) FROM daily
   WHERE env = 'host' AND day = date('now') GROUP BY version ORDER BY 2 DESC"
```

A day's count is a lower bound: installs behind one office IP with the same
version and OS count once, and offline machines and those with the check off
are not seen at all.

## Local check

```sh
npx wrangler d1 execute kaeru-updates --local --file=schema.sql
npx wrangler dev --local --test-scheduled
curl -A "kaeru/0.7.5 (macos-aarch64; installer; host)" http://127.0.0.1:8787/v1/releases
curl "http://127.0.0.1:8787/__scheduled?cron=17+0+*+*+*"   # runs the daily sweep
```
