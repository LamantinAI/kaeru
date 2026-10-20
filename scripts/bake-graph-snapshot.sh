#!/usr/bin/env bash
# Bake a scoped offline graph from the named HTTP memory API. The output is
# git-ignored and can contain real content; the script does not commit it.
# KAERU_VIZ_URL sets the daemon base URL; KAERU_VIZ_TOKEN supplies auth locally.
# Optional first argument narrows to exact initiative names. KAERU_VIZ_DENY
# rejects substring matches before any output file is written.
set -euo pipefail
kaeru_script_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
node --input-type=module - "$kaeru_script_root" "${1:-}" <<'JS'
import { mkdir, writeFile, rename } from 'node:fs/promises'
import { pathToFileURL } from 'node:url'
const [root, csv] = process.argv.slice(2)
const { createClient } = await import(pathToFileURL(`${root}/kaeru-viz/src/api.js`))
const token = process.env.KAERU_VIZ_TOKEN
const client = createClient({ baseUrl: process.env.KAERU_VIZ_URL || 'http://127.0.0.1:9876',
  headers: token ? { authorization: `Bearer ${token}` } : {} })
const scopes = csv.split(',').map((s) => s.trim()).filter(Boolean)
const graph = await client.graph(scopes)
const deny = (process.env.KAERU_VIZ_DENY || '').split(',').map((s) => s.trim().toLowerCase()).filter(Boolean)
if (graph.initiatives.some((i) => deny.some((d) => i.name.toLowerCase().includes(d)))) throw new Error('Denied initiative in snapshot')
const secret = /sk-[A-Za-z0-9]{20}|ghp_[A-Za-z0-9]{20}|AKIA[A-Z0-9]{16}|-----BEGIN/
if (graph.nodes.some((n) => secret.test(`${n.name}\n${n.body || ''}`))) throw new Error('Secret in snapshot')
const directory = `${root}/kaeru-viz/public`
await mkdir(directory, { recursive: true })
const temporary = `${directory}/.graph-${process.pid}.json`
await writeFile(temporary, JSON.stringify(graph, null, 2))
await rename(temporary, `${directory}/graph.json`)
console.log(`${graph.meta.node_count} nodes / ${graph.meta.edge_count} edges / ${graph.meta.initiative_count} initiatives`)
JS
