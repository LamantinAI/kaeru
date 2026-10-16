import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";
import test from "node:test";

import worker from "../src/index.js";

// The D1 adapter executes the worker's real SQL against SQLite. batch() uses
// the transactional, sequential behavior promised by D1; there is no network.
class LocalD1 {
  constructor() {
    this.db = new DatabaseSync(":memory:");
    this.db.exec(readFileSync(new URL("../schema.sql", import.meta.url), "utf8"));
  }

  prepare(sql) {
    const statement = (...args) => ({
      bind: (...values) => statement(...values),
      run: async () => ({ meta: { changes: Number(this.db.prepare(sql).run(...args).changes) } }),
      first: async () => this.db.prepare(sql).get(...args) ?? null,
      execute: () => ({ meta: { changes: Number(this.db.prepare(sql).run(...args).changes) } }),
    });
    return statement();
  }

  async batch(statements) {
    this.db.exec("BEGIN");
    try {
      const results = statements.map((statement) => statement.execute());
      this.db.exec("COMMIT");
      return results;
    } catch (error) {
      this.db.exec("ROLLBACK");
      throw error;
    }
  }
}

globalThis.caches = { default: { match: async () => new Response("[]") } };

function fixture(t) {
  const DB = new LocalD1();
  t.after(() => DB.db.close());
  const count = (table) => DB.db.prepare(`SELECT COUNT(*) AS n FROM ${table}`).get().n;
  const total = () => DB.db.prepare("SELECT COALESCE(SUM(installs), 0) AS n FROM daily").get().n;
  const request = async (ip = "192.0.2.1", ua = "kaeru/0.7.6 (linux-x86_64; installer; host)") => {
    const pending = [];
    const response = await worker.fetch(new Request("https://updates.example/v1/releases", {
      headers: { "User-Agent": ua, "CF-Connecting-IP": ip },
    }), { DB }, { waitUntil: (promise) => pending.push(promise) });
    await Promise.all(pending);
    assert.equal(response.status, 200, "counting errors do not break release checks");
  };
  return { DB, count, total, request };
}

test("a failed counter write leaves no dedup marker and a retry counts once", async (t) => {
  const { DB, count, total, request } = fixture(t);
  DB.db.exec(`CREATE TRIGGER fail_daily BEFORE INSERT ON daily
    BEGIN SELECT RAISE(ABORT, 'temporary counter failure'); END`);
  await request();
  assert.equal(count("seen"), 0);
  assert.equal(total(), 0);
  DB.db.exec("DROP TRIGGER fail_daily");
  await request();
  await request();
  assert.equal(count("seen"), 1);
  assert.equal(total(), 1);
});

test("a failed dedup write also rolls back the counter", async (t) => {
  const { DB, count, total, request } = fixture(t);
  DB.db.exec(`CREATE TRIGGER fail_seen BEFORE INSERT ON seen
    BEGIN SELECT RAISE(ABORT, 'temporary dedup failure'); END`);
  await request();
  assert.equal(count("seen"), 0);
  assert.equal(total(), 0);
  DB.db.exec("DROP TRIGGER fail_seen");
  await request();
  assert.equal(count("seen"), 1);
  assert.equal(total(), 1);
});

test("concurrent duplicate requests count a fingerprint only once", async (t) => {
  const { count, total, request } = fixture(t);
  await Promise.all(Array.from({ length: 12 }, () => request()));
  assert.equal(count("seen"), 1);
  assert.equal(total(), 1);
});

test("concurrent distinct fingerprints increment the same bucket without loss", async (t) => {
  const { count, total, request } = fixture(t);
  await Promise.all(Array.from({ length: 12 }, (_, i) => request(`192.0.2.${i + 1}`)));
  assert.equal(count("seen"), 12);
  assert.equal(count("daily"), 1);
  assert.equal(total(), 12);
});

test("different User-Agents remain separate buckets and non-kaeru clients are ignored", async (t) => {
  const { count, total, request } = fixture(t);
  await request();
  await request("192.0.2.1", "kaeru/0.7.5 (linux-x86_64; installer; host)");
  await request("192.0.2.1", "other-client");
  assert.equal(count("seen"), 2);
  assert.equal(count("daily"), 2);
  assert.equal(total(), 2);
});
