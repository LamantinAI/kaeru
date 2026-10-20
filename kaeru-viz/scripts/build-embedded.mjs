// Build a snapshot-free, reproducible bundle that cargo can embed without Node.
import { createHash } from 'node:crypto'
import { readdir, readFile, mkdir, rm, mkdtemp, writeFile, cp } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { dirname, join, relative, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { build } from 'vite'

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const target = resolve(root, '../kaeru-mcp/ui-dist')
const temporary = await mkdtemp(join(tmpdir(), 'kaeru-ui-build-'))
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex')
async function files(directory) {
  const out = []
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name)
    if (entry.isDirectory()) out.push(...await files(path)); else if (entry.isFile()) out.push(path)
  }
  return out.sort()
}
async function fingerprints(directory) {
  return Object.fromEntries(await Promise.all((await files(directory)).map(async (path) => [relative(directory, path).replaceAll('\\', '/'), hash(await readFile(path))])))
}
try {
  const inputs = [...await files(join(root, 'src')), ...await files(join(root, 'public/fonts')), ...['index.html', 'package.json', 'package-lock.json', 'vite.config.js', 'scripts/build-embedded.mjs'].map((path) => join(root, path))].sort()
  const digest = createHash('sha256')
  for (const path of inputs) { digest.update(relative(root, path).replaceAll('\\', '/')); digest.update(await readFile(path)) }
  await build({ root, configFile: join(root, 'vite.config.js'), base: '/ui/', publicDir: false,
    build: { outDir: temporary, emptyOutDir: true, sourcemap: false } })
  const assets = await fingerprints(temporary)
  for (const name of Object.keys(assets)) {
    if (!/\.(html|js|css|svg|woff2|png|ico)$/.test(name) || name.endsWith('graph.json')) throw new Error(`Unexpected embedded asset: ${name}`)
  }
  const manifest = { source_sha256: digest.digest('hex'), assets }
  await writeFile(join(temporary, 'manifest.json'), JSON.stringify(manifest, null, 2) + '\n')
  if (process.argv.includes('--check')) {
    const actual = await fingerprints(target)
    const expected = await fingerprints(temporary)
    if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error('Embedded UI is stale; run npm run build:embedded')
    console.log('Embedded UI matches its sources and lockfile.')
  } else {
    await rm(target, { recursive: true, force: true })
    await mkdir(target, { recursive: true })
    await cp(temporary, target, { recursive: true })
    console.log(`Embedded ${Object.keys(assets).length} assets; no graph snapshot.`)
  }
} finally { await rm(temporary, { recursive: true, force: true }) }
