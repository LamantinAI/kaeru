-- One random salt per UTC day. Deleted the day after, so a hash from one day
-- cannot be recomputed or matched against another day's.
CREATE TABLE IF NOT EXISTS salt (
  day   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

-- Who was already counted today: a salted hash, nothing else. Swept daily.
CREATE TABLE IF NOT EXISTS seen (
  day  TEXT NOT NULL,
  hash TEXT NOT NULL,
  PRIMARY KEY (day, hash)
);

-- The only thing kept: how many distinct installs asked on a day, by what
-- they said about themselves in the User-Agent.
CREATE TABLE IF NOT EXISTS daily (
  day      TEXT NOT NULL,
  version  TEXT NOT NULL,
  os       TEXT NOT NULL,
  channel  TEXT NOT NULL,
  env      TEXT NOT NULL,
  installs INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (day, version, os, channel, env)
);
