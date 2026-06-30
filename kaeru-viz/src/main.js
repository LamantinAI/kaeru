import * as THREE from 'three'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'

// ── helpers ───────────────────────────────────────────────────────────────
const esc = (s) => String(s ?? '').replace(/[&<>]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;' }[c]))
const fmtDate = (secs) => new Date(secs * 1000).toISOString().slice(0, 10)
const fmtDateTime = (secs) => {
  const d = new Date(secs * 1000)
  return d.toISOString().slice(0, 10).replace(/-/g, '·') + ' ' + d.toISOString().slice(11, 16)
}
const $ = (id) => document.getElementById(id)
function hash(str) { let h = 2166136261; for (let i = 0; i < str.length; i++) { h ^= str.charCodeAt(i); h = Math.imul(h, 16777619) } return h >>> 0 }
function rng(seed) { let s = seed >>> 0; return () => { s = (s * 1664525 + 1013904223) >>> 0; return s / 4294967296 } }

// ── palettes ──────────────────────────────────────────────────────────────
const TIER_COLOR = { operational: '#ff6ad5', archival: '#56d0ff' }
const LAYER_COLOR = { core: '#ffce4d', hot: '#ff7a45', warm: '#6f95ff', cold: '#5566a0', frozen: '#46506f' }
const LAYER_BAR = { core: '#caa24a', hot: '#c8402e', warm: '#7e96cf', cold: '#6f7da0', frozen: '#566' }
const EDGE_COLOR = {
  derived_from: [0.45, 0.78, 0.85], refers_to: [0.55, 0.6, 0.82], causal: [0.88, 0.66, 0.36],
  supersedes: [0.85, 0.45, 0.45], verifies: [0.42, 0.78, 0.6], falsifies: [0.9, 0.45, 0.72],
  contradicts: [0.9, 0.35, 0.35], part_of: [0.66, 0.82, 0.45], temporal: [0.6, 0.64, 0.78],
  consolidated_to: [0.95, 0.78, 0.4], blocks: [0.95, 0.58, 0.32], targets: [0.4, 0.78, 0.95],
}
const EDGE_DEFAULT = [0.4, 0.45, 0.55]
// per-layer point geometry: spread from cluster centre, base px size, white-mix, alpha
const LAYER = {
  hub:    { spread: 0,   size: 24, mix: 0.12, a: 1.0 },
  core:   { spread: 75,  size: 22, mix: 0.12, a: 1.0 },
  hot:    { spread: 150, size: 14, mix: 0.06, a: 1.0 },
  warm:   { spread: 240, size: 10, mix: 0.02, a: 0.96 },
  cold:   { spread: 300, size: 7,  mix: 0.0,  a: 0.72 },
  frozen: { spread: 345, size: 5,  mix: 0.0,  a: 0.52 },
}
const WHITE = new THREE.Color(0xfff3df)
const BG = 0x2a2a2f   // greyish backdrop, not pure black

// ── data ──────────────────────────────────────────────────────────────────
async function loadGraph() {
  for (const url of ['/graph.json', './graph.json']) {
    try { const r = await fetch(url); if (r.ok) return await r.json() } catch (_) {}
  }
  return null
}
function fail(msg) {
  const g = document.getElementById('graph')
  if (g) g.insertAdjacentHTML('beforeend',
    `<div style="position:absolute;inset:0;display:grid;place-items:center;color:#8b887e;font-family:'IBM Plex Mono',monospace;font-size:13px;letter-spacing:.04em;text-align:center;padding:24px;">${msg}</div>`)
  const s = document.getElementById('stats'); if (s) s.textContent = 'unavailable'
  throw new Error('kaeru-viz: ' + msg)  // halt module init — the message above already shows
}
const data = await loadGraph()
if (!data || !Array.isArray(data.nodes) || data.nodes.length === 0) {
  fail('graph data unavailable — is the kaeru daemon running, or a snapshot baked?')
}
// tolerate a partial/malformed snapshot: default the optional collections
data.initiatives ??= []
data.edges ??= []
data.chains ??= []
data.project_links ??= []
data.meta ??= {}
const rawNodes = data.nodes.filter((n) => n.type !== 'chain')
const primInit = (n) => (n.isHub ? n.hubInit : (n.initiatives && n.initiatives[0]) || '∅')

const initNames = data.initiatives.map((i) => i.name)
const initColor = {}, initCenter = {}
const R = 1050
initNames.forEach((name, i) => {
  initColor[name] = new THREE.Color().setHSL(((i * 137.508) % 360) / 360, 0.95, 0.6)
  const y = initNames.length > 1 ? 1 - (i / (initNames.length - 1)) * 2 : 0
  const rad = Math.sqrt(Math.max(0, 1 - y * y)), theta = i * 2.39996
  initCenter[name] = new THREE.Vector3(Math.cos(theta) * rad * R, y * R * 0.82, Math.sin(theta) * rad * R)
})
const fallbackCenter = new THREE.Vector3(0, 0, 0)
const centerOf = (name) => initCenter[name] || fallbackCenter

// scale each cluster's spread by ∛(count) so big clusters (e.g. 1c-agent, 184
// nodes) don't pack tight — keeps node density roughly even across projects.
const avgCount = rawNodes.length / Math.max(1, initNames.length)
const clusterScale = {}
for (const i of data.initiatives) clusterScale[i.name] = Math.min(2.0, Math.max(0.7, Math.cbrt((i.node_count || 1) / avgCount)))

// place every node deterministically around its cluster centre
const nodes = []
const byId = new Map()
for (const n of rawNodes) {
  const init = primInit(n)
  const c = centerOf(init)
  const L = LAYER[n.layer] || LAYER.warm
  const sc = clusterScale[init] || 1
  const r = rng(hash(n.id))
  const g = () => (r() + r() + r() - 1.5) * 0.9
  const s = L.spread * sc
  const node = {
    ...n, init, layer: n.layer || 'warm', isHub: false,
    pos: new THREE.Vector3(c.x + g() * s, c.y + g() * s, c.z + g() * s),
  }
  nodes.push(node); byId.set(node.id, node)
}
// project-core hubs at each cluster centre
const countByInit = Object.fromEntries(data.initiatives.map((i) => [i.name, i.node_count]))
for (const name of initNames) {
  const hub = { id: '__hub__' + name, name, isHub: true, hubInit: name, layer: 'hub',
    type: 'project core', tier: '—', initiatives: [name], count: countByInit[name] || 1,
    pos: centerOf(name).clone() }
  nodes.push(hub); byId.set(hub.id, hub)
}
const NN = nodes.length
const idx = new Map(nodes.map((n, i) => [n.id, i]))

// edges: real graph edges + membership spokes + cross-project bridges
const edges = []
for (const e of data.edges) {
  if (idx.has(e.src) && idx.has(e.dst)) edges.push({ a: idx.get(e.src), b: idx.get(e.dst), type: e.type, kind: 'real' })
}
for (const n of nodes) {
  if (n.isHub) continue
  const h = '__hub__' + n.init
  if (idx.has(h)) edges.push({ a: idx.get(n.id), b: idx.get(h), kind: 'spoke' })
}
const bridges = []
const _pl = [...(data.project_links || [])].sort((x, y) => (y.weight || 0) - (x.weight || 0)).slice(0, 20)
for (const p of _pl) {
  const ha = '__hub__' + p.a, hb = '__hub__' + p.b
  if (idx.has(ha) && idx.has(hb)) bridges.push({ a: idx.get(ha), b: idx.get(hb), w: p.weight })
}
// adjacency (real + spoke) for hover neighbourhoods
const adj = nodes.map(() => new Set())
edges.forEach((e, i) => { if (e.kind !== 'bridge') { adj[e.a].add(i); adj[e.b].add(i) } })

// ── scene ─────────────────────────────────────────────────────────────────
const host = $('graph')
const scene = new THREE.Scene()
scene.fog = new THREE.FogExp2(BG, 0.00018)
const camera = new THREE.PerspectiveCamera(55, innerWidth / innerHeight, 1, 8000)
const renderer = new THREE.WebGLRenderer({ antialias: true })
renderer.setPixelRatio(Math.min(2, devicePixelRatio))
renderer.setSize(innerWidth, innerHeight)
renderer.setClearColor(BG, 1)
host.appendChild(renderer.domElement)
const controls = new OrbitControls(camera, renderer.domElement)
controls.enableDamping = true; controls.dampingFactor = 0.08; controls.enablePan = false
controls.autoRotate = true; controls.autoRotateSpeed = 0.4; controls.minDistance = 200; controls.maxDistance = 4000

function discTex() {
  const c = document.createElement('canvas'); c.width = c.height = 64; const x = c.getContext('2d')
  const g = x.createRadialGradient(32, 32, 0, 32, 32, 32)
  g.addColorStop(0, 'rgba(255,255,255,1)'); g.addColorStop(0.55, 'rgba(255,255,255,1)')
  g.addColorStop(0.82, 'rgba(255,255,255,0.5)'); g.addColorStop(1, 'rgba(255,255,255,0)')
  x.fillStyle = g; x.fillRect(0, 0, 64, 64); return new THREE.CanvasTexture(c)
}
function ringTex() {
  const c = document.createElement('canvas'); c.width = c.height = 128; const x = c.getContext('2d')
  x.strokeStyle = 'rgba(236,232,223,0.95)'; x.lineWidth = 4; x.beginPath(); x.arc(64, 64, 52, 0, 7); x.stroke()
  for (let k = 0; k < 4; k++) { const a = k * Math.PI / 2; x.beginPath(); x.moveTo(64 + Math.cos(a) * 52, 64 + Math.sin(a) * 52); x.lineTo(64 + Math.cos(a) * 62, 64 + Math.sin(a) * 62); x.stroke() }
  return new THREE.CanvasTexture(c)
}
const TEX = discTex()

// node cloud — one Points with per-node colour/size/alpha attributes
const pos = new Float32Array(NN * 3), col = new Float32Array(NN * 3), siz = new Float32Array(NN), alp = new Float32Array(NN)
nodes.forEach((n, i) => { pos[i * 3] = n.pos.x; pos[i * 3 + 1] = n.pos.y; pos[i * 3 + 2] = n.pos.z })
const ng = new THREE.BufferGeometry()
ng.setAttribute('position', new THREE.BufferAttribute(pos, 3))
ng.setAttribute('aColor', new THREE.BufferAttribute(col, 3))
ng.setAttribute('aSize', new THREE.BufferAttribute(siz, 1))
ng.setAttribute('aAlpha', new THREE.BufferAttribute(alp, 1))
const nmat = new THREE.ShaderMaterial({
  uniforms: { map: { value: TEX } }, transparent: true, depthWrite: false,
  vertexShader: `attribute vec3 aColor;attribute float aSize;attribute float aAlpha;
    varying vec3 vC;varying float vA;void main(){vC=aColor;vA=aAlpha;
    vec4 mv=modelViewMatrix*vec4(position,1.0);
    gl_PointSize=aSize*(1100.0/ -mv.z);gl_Position=projectionMatrix*mv;}`,
  fragmentShader: `uniform sampler2D map;varying vec3 vC;varying float vA;
    void main(){if(vA<0.01)discard;vec4 t=texture2D(map,gl_PointCoord);if(t.a<0.04)discard;gl_FragColor=vec4(vC,t.a*vA);}`,
})
scene.add(new THREE.Points(ng, nmat))

// edges — one LineSegments, per-vertex colour carries visibility/intensity
const epos = new Float32Array(edges.length * 6), ecol = new Float32Array(edges.length * 6)
edges.forEach((e, i) => { const A = nodes[e.a].pos, B = nodes[e.b].pos; epos.set([A.x, A.y, A.z, B.x, B.y, B.z], i * 6) })
const eg = new THREE.BufferGeometry()
eg.setAttribute('position', new THREE.BufferAttribute(epos, 3))
eg.setAttribute('color', new THREE.BufferAttribute(ecol, 3))
const emat = new THREE.LineBasicMaterial({ vertexColors: true, transparent: true, opacity: 0.1, depthWrite: false })
scene.add(new THREE.LineSegments(eg, emat))

// cross-project bridges — their own brighter ochre layer (the 0.1 edge layer
// would otherwise bury them)
const bgeo = new THREE.BufferGeometry()
const bpos2 = new Float32Array(bridges.length * 6)
bridges.forEach((e, i) => { const A = nodes[e.a].pos, B = nodes[e.b].pos; bpos2.set([A.x, A.y, A.z, B.x, B.y, B.z], i * 6) })
bgeo.setAttribute('position', new THREE.BufferAttribute(bpos2, 3))
const bmat = new THREE.LineBasicMaterial({ color: 0xcf8b2e, transparent: true, opacity: 0.6, depthWrite: false })
scene.add(new THREE.LineSegments(bgeo, bmat))

// distant starfield (depth)
const sf = []; const sr = rng(99); for (let i = 0; i < 900; i++) sf.push(new THREE.Vector3(sr() * 2 - 1, sr() * 2 - 1, sr() * 2 - 1).normalize().multiplyScalar(2200 + sr() * 1600))
const sg = new THREE.BufferGeometry(); const sp = new Float32Array(sf.length * 3); sf.forEach((v, i) => sp.set([v.x, v.y, v.z], i * 3))
sg.setAttribute('position', new THREE.BufferAttribute(sp, 3))
scene.add(new THREE.Points(sg, new THREE.PointsMaterial({ size: 3, map: TEX, color: 0x9a968c, transparent: true, opacity: 0.4, blending: THREE.AdditiveBlending, depthWrite: false, sizeAttenuation: true })))

// bridge particles
const bp = bridges.flatMap(() => [0.2, 0.55, 0.85])
const bpGeo = new THREE.BufferGeometry(); const bpPos = new Float32Array(bp.length * 3)
bpGeo.setAttribute('position', new THREE.BufferAttribute(bpPos, 3))
const bpPoints = new THREE.Points(bpGeo, new THREE.PointsMaterial({ size: 5, map: TEX, color: 0xe6a83c, transparent: true, opacity: 0.8, blending: THREE.AdditiveBlending, depthWrite: false, sizeAttenuation: true }))
scene.add(bpPoints)

// selection ring
const ring = new THREE.Sprite(new THREE.SpriteMaterial({ map: ringTex(), transparent: true, depthTest: false, depthWrite: false }))
ring.visible = false; ring.renderOrder = 10; scene.add(ring)

// ── state ─────────────────────────────────────────────────────────────────
let colorMode = 'initiative', glow = true, focusInit = null
const T0 = data.meta.earliest_secs ?? 0, T1 = data.meta.latest_secs ?? 0
let timeFilter = Infinity
const chain = new Set()
let hovered = -1, pinned = -1

const baseColor = (n) => {
  if (colorMode === 'tier') return new THREE.Color(TIER_COLOR[n.tier] || '#888')
  if (colorMode === 'layer') return new THREE.Color(LAYER_COLOR[n.layer] || '#888')
  const hue = initColor[n.init] || initColor[n.hubInit] || new THREE.Color('#9af')
  const L = LAYER[n.layer] || LAYER.warm
  return hue.clone().lerp(WHITE, L.mix)
}
const visible = (n) => n.isHub ? true : (n.created_secs ?? 0) <= timeFilter

const _c = new THREE.Color()
function applyVisuals() {
  const anyFocus = focusInit && !chain.size
  const act = hovered >= 0 ? hovered : pinned       // hover wins; pinned persists
  const actNb = hovered >= 0 ? hoverNb : pinnedNb
  for (let i = 0; i < NN; i++) {
    const n = nodes[i]
    const L = LAYER[n.layer] || LAYER.warm
    let size = L.size
    if (glow && (n.layer === 'core' || n.layer === 'hot')) size *= 1.45
    let a = L.a, dim = false, hot = false
    if (!visible(n)) { a = 0 }
    else if (chain.size) {
      if (chain.has(n.id)) { hot = true; if (n.__cur) hot = 'cur'; size = Math.max(size, 16) } else dim = true
    } else if (act >= 0) {
      if (i === act) { hot = true; size *= 1.6 } else if (actNb.has(i)) { hot = true } else dim = true
    } else if (anyFocus) {
      if (n.init !== focusInit && n.hubInit !== focusInit) dim = true
    }
    _c.copy(baseColor(n))
    if (hot === 'cur') _c.set('#ffffff')
    else if (hot && chain.size) _c.copy(n.__visited ? new THREE.Color('#ffd86a') : baseColor(n)).lerp(WHITE, 0.2)
    else if (dim) _c.multiplyScalar(0.2)
    col[i * 3] = _c.r; col[i * 3 + 1] = _c.g; col[i * 3 + 2] = _c.b
    siz[i] = size; alp[i] = a
  }
  ng.attributes.aColor.needsUpdate = ng.attributes.aSize.needsUpdate = ng.attributes.aAlpha.needsUpdate = true
  // bridges glow ochre at rest; recede while a node/chain/focus is in the spotlight
  bmat.opacity = (chain.size || hovered >= 0 || pinned >= 0 || focusInit) ? 0.14 : 0.6
  applyEdges()
}
function edgeBaseColor(e) {
  if (e.kind === 'spoke') return [0.15, 0.16, 0.21]
  if (e.kind === 'bridge') return [0.72, 0.54, 0.28]
  return EDGE_COLOR[e.type] || EDGE_DEFAULT
}
function applyEdges() {
  const anyFocus = focusInit && !chain.size
  const act = hovered >= 0 ? hovered : pinned
  for (let i = 0; i < edges.length; i++) {
    const e = edges[i], na = nodes[e.a], nb = nodes[e.b]
    let c = edgeBaseColor(e), k = 1
    const vis = visible(na) && visible(nb)
    if (!vis) k = 0
    else if (chain.size) { k = (chain.has(na.id) && chain.has(nb.id) && e.kind !== 'spoke') ? 1.4 : 0.05; if (k > 1) c = [1.0, 0.83, 0.4] }
    else if (act >= 0) {
      const inc = adj[act].has(i)
      if (inc) { k = 1.5; c = [0.95, 0.88, 0.66] } else k = 0.06
    } else if (anyFocus) {
      const inF = (na.init === focusInit || na.hubInit === focusInit) && (nb.init === focusInit || nb.hubInit === focusInit)
      k = inF ? 1 : 0.05
    } else if (e.kind === 'bridge') k = 0.9
    else if (e.kind === 'spoke') { const ml = nodes[e.a].layer; k = (ml === 'cold' || ml === 'frozen') ? 0 : 0.85 }
    for (let q = 0; q < 6; q += 3) { ecol[i * 6 + q] = c[0] * k; ecol[i * 6 + q + 1] = c[1] * k; ecol[i * 6 + q + 2] = c[2] * k }
  }
  eg.attributes.color.needsUpdate = true
}

// ── readout card ──────────────────────────────────────────────────────────
const readout = $('readout')
function showReadout(n, step, total) {
  const kicker = step ? `step <b>${step}</b> / ${total} · knowledge chain` : `node · ${esc(n.type)}`
  const bar = LAYER_BAR[n.layer] || '#caa24a'
  const asserted = n.created_secs ? fmtDateTime(n.created_secs) : '—'
  const meta = n.isHub
    ? `<span class="tag k">${n.count} insights</span><span class="tag">project core</span>`
    : `<span class="tag k">${esc(n.tier)}</span><span class="tag">${esc(n.layer)}</span>${(n.initiatives || []).map((i) => `<span class="tag">${esc(i)}</span>`).join('')}`
  readout.innerHTML = `
    <div class="bar" style="background:${bar}"></div>
    <div class="body">
      <div class="kicker">${kicker}</div>
      <h2>${esc(n.name)}</h2>
      <div class="tags">${meta}</div>
      <p>${esc(n.body || (n.redacted ? '⟨body redacted⟩' : (n.isHub ? 'A project cluster — hover its nodes to explore.' : '—')))}</p>
      ${n.isHub ? '' : `<div class="ts">asserted <b>${asserted}</b></div>`}
    </div>`
  readout.classList.add('show')
}
const hideReadout = () => readout.classList.remove('show')

// ── hover (speed-gated, forgiving pick) ─────────────────────────────────────
let hoverNb = new Set(), pinnedNb = new Set()
const chip = (() => { const d = document.createElement('div'); d.id = 'nodechip'; document.body.appendChild(d); return d })()
chip.style.cssText = 'position:fixed;z-index:11;pointer-events:none;transform:translate(-50%,-145%);background:rgba(18,18,21,.94);border:1px solid rgba(236,232,223,.2);border-radius:4px;padding:5px 9px;font-family:"Zen Old Mincho",Georgia,serif;font-size:13px;white-space:nowrap;display:none;box-shadow:0 4px 18px rgba(0,0,0,.5)'

let mouse = null, lastMove = 0, lastP = null, speed = 0, dragging = false, downAt = null
let hoverCand = -1, candSince = 0
const SPEED = 1.1, HOVER_DELAY = 120   // ms a cursor must dwell on a node before it highlights
addEventListener('pointermove', (e) => {
  const t = performance.now()
  if (lastP) { const dt = Math.max(1, t - lastMove); speed = Math.hypot(e.clientX - lastP.x, e.clientY - lastP.y) / dt }
  lastP = { x: e.clientX, y: e.clientY }; lastMove = t
  // pick only when over the canvas and not dragging — panels (console, readout,
  // tour) shield the nodes beneath them, and orbiting the camera shouldn't focus.
  mouse = (!dragging && e.target === renderer.domElement) ? { x: e.clientX, y: e.clientY } : null
})
addEventListener('pointerleave', () => { mouse = null })
addEventListener('pointerdown', (e) => { dragging = true; downAt = { x: e.clientX, y: e.clientY, onCanvas: e.target === renderer.domElement } })
addEventListener('pointerup', (e) => {
  dragging = false
  // a click (no drag) on the canvas pins the node + its links; on empty space, clears
  if (downAt && downAt.onCanvas && Math.hypot(e.clientX - downAt.x, e.clientY - downAt.y) < 5) {
    mouse = { x: e.clientX, y: e.clientY }; pinNode(pick())
  }
  downAt = null
})

const _v = new THREE.Vector3()
function pick() {
  if (!mouse) return -1
  let best = -1, bd = 22 * 22   // forgiving catch radius; the dwell delay tames flicker
  for (let i = 0; i < NN; i++) {
    if (alp[i] < 0.05) continue
    _v.set(pos[i * 3], pos[i * 3 + 1], pos[i * 3 + 2]).project(camera)
    if (_v.z > 1) continue
    const sx = (_v.x * 0.5 + 0.5) * innerWidth, sy = (-_v.y * 0.5 + 0.5) * innerHeight
    const dx = sx - mouse.x, dy = sy - mouse.y, w = nodes[i].isHub ? 2 : 1
    const d = (dx * dx + dy * dy) / (w * w)
    if (d < bd) { bd = d; best = i }
  }
  return best
}
function nbOf(i) { const s = new Set([i]); for (const ei of adj[i]) { const e = edges[ei]; s.add(e.a); s.add(e.b) } return s }
// the active node = hovered if hovering, else the pinned (clicked) one
function refreshActive() {
  const a = hovered >= 0 ? hovered : pinned
  if (!chain.size) { if (a >= 0) showReadout(nodes[a]); else hideReadout() }
  if (a >= 0) { const n = nodes[a]; ring.position.copy(n.pos); const sc = (LAYER[n.layer] || LAYER.warm).size * 2.6; ring.scale.set(sc, sc, 1); ring.visible = true } else ring.visible = false
  applyVisuals()
}
function setHover(i) {
  if (i === hovered) return
  hovered = i
  if (i < 0) { hoverNb = new Set(); chip.style.display = 'none'; host.style.cursor = ''; refreshActive(); return }
  hoverNb = nbOf(i)
  const n = nodes[i]
  _v.copy(n.pos).project(camera)
  chip.style.display = 'block'
  chip.style.left = ((_v.x * 0.5 + 0.5) * innerWidth) + 'px'
  chip.style.top = ((-_v.y * 0.5 + 0.5) * innerHeight) + 'px'
  chip.innerHTML = `${esc(n.name)}<div style="font-family:var(--mono,monospace);font-size:9px;letter-spacing:.12em;text-transform:uppercase;color:#8b887e;margin-top:2px">${esc(n.type)} · ${hoverNb.size - 1} neighbours</div>`
  host.style.cursor = 'pointer'
  refreshActive()
}
function pinNode(i) { pinned = i; pinnedNb = i >= 0 ? nbOf(i) : new Set(); refreshActive() }

// ── chain replay ────────────────────────────────────────────────────────────
let replayTimer = null
function flyTo(p, ms = 1200) {
  const d = p.length() || 1, ratio = 1 + 320 / d
  controls.autoRotate = false
  controls.target.lerp(p, 0.6)
  camTween({ x: p.x * ratio, y: p.y * ratio, z: p.z * ratio }, p, ms)
}
let tween = null
function camTween(to, target, ms) {
  const from = camera.position.clone(), tgt0 = controls.target.clone(), t0 = performance.now()
  tween = () => {
    const k = Math.min(1, (performance.now() - t0) / ms), e = 1 - Math.pow(1 - k, 3)
    camera.position.set(from.x + (to.x - from.x) * e, from.y + (to.y - from.y) * e, from.z + (to.z - from.z) * e)
    controls.target.set(tgt0.x + (target.x - tgt0.x) * e, tgt0.y + (target.y - tgt0.y) * e, tgt0.z + (target.z - tgt0.z) * e)
    if (k >= 1) tween = null
  }
}
function startReplay(c) {
  stopReplay(); const members = (c.members || []).filter((id) => byId.has(id))
  if (members.length < 2) return
  pinned = -1; pinnedNb = new Set()
  chain.clear(); members.forEach((id) => chain.add(id))
  nodes.forEach((n) => { n.__visited = false; n.__cur = false })
  resetTime(); let i = 0
  const step = () => {
    if (i > 0) { const p = byId.get(members[i - 1]); p.__cur = false; p.__visited = true }
    if (i >= members.length) { const last = byId.get(members[members.length - 1]); showReadout(last, members.length, members.length); applyVisuals(); return }
    const cur = byId.get(members[i]); cur.__cur = true; cur.__visited = true
    showReadout(cur, i + 1, members.length); flyTo(cur.pos); applyVisuals()
    i += 1; replayTimer = setTimeout(step, 2400)
  }
  step()
}
function stopReplay() { if (replayTimer) { clearTimeout(replayTimer); replayTimer = null } }
function resetChain() { stopReplay(); chain.clear(); pinned = -1; pinnedNb = new Set(); nodes.forEach((n) => { n.__visited = false; n.__cur = false }); applyVisuals(); hideReadout(); controls.autoRotate = true; frame(900) }

// ── time-lapse ──────────────────────────────────────────────────────────────
const timeEl = $('time'); const timeLabel = (s) => ($('timeLabel').textContent = s)
function applyTime(pct) {
  if (pct >= 100) { timeFilter = Infinity; timeLabel('— full graph —') }
  else { timeFilter = T0 + (pct / 100) * (T1 - T0); timeLabel(`${fmtDate(T0)} → ${fmtDate(timeFilter)}`) }
  applyVisuals()
}
timeEl.addEventListener('input', (e) => applyTime(+e.target.value))
let timeAnim = null
const stopTimeLapse = () => { if (timeAnim) { clearInterval(timeAnim); timeAnim = null } }
function startTimeLapse() { stopTimeLapse(); let v = 0; timeEl.value = 0; applyTime(0); timeAnim = setInterval(() => { v += 1.5; if (v >= 100) { v = 100; stopTimeLapse() } timeEl.value = v; applyTime(v) }, 60) }
function resetTime() { stopTimeLapse(); timeEl.value = 100; applyTime(100) }
$('timePlay').addEventListener('click', () => (timeAnim ? stopTimeLapse() : startTimeLapse()))

// ── focus ───────────────────────────────────────────────────────────────────
function setFocus(name) {
  focusInit = name || null; $('focus').value = name || ''; $('focus')._sync && $('focus')._sync()
  pinned = -1; pinnedNb = new Set()
  applyVisuals()
  if (focusInit) frameCluster(focusInit, 1400); else frame(1200)
}

// ── colour / glow ────────────────────────────────────────────────────────────
function setColorMode(m) { colorMode = m; $('colorMode').value = m; $('colorMode')._sync && $('colorMode')._sync(); buildLegend(); applyVisuals() }
function setGlow(b) { glow = b; $('glow').checked = b; applyVisuals() }
$('colorMode').addEventListener('change', (e) => setColorMode(e.target.value))
$('glow').addEventListener('change', (e) => setGlow(e.target.checked))
const focusEl = $('focus')
data.initiatives.forEach((i) => { const o = document.createElement('option'); o.value = i.name; o.textContent = `${i.name} (${i.node_count})`; focusEl.appendChild(o) })
focusEl.addEventListener('change', (e) => setFocus(e.target.value))
const chainPick = $('chainPick')
data.chains.forEach((c, i) => { const o = document.createElement('option'); o.value = i; o.textContent = `${c.name} (${c.members.length})`; chainPick.appendChild(o) })
$('chainPlay').addEventListener('click', () => { const c = data.chains[+chainPick.value || 0]; if (c) startReplay(c) })
$('chainReset').addEventListener('click', resetChain)

// custom archive-styled dropdowns over the (hidden) native selects
function makeDropdown(sel) {
  sel.style.display = 'none'
  const dd = document.createElement('div'); dd.className = 'dd'
  const trig = document.createElement('div'); trig.className = 'dd-trigger'; trig.tabIndex = 0
  const lab = document.createElement('span'); lab.className = 'dd-label'
  const car = document.createElement('span'); car.className = 'dd-caret'
  trig.append(lab, car)
  const menu = document.createElement('div'); menu.className = 'dd-menu'; menu.hidden = true
  dd.append(trig, menu); sel.after(dd)
  const close = () => { menu.hidden = true; trig.classList.remove('open'); document.removeEventListener('pointerdown', outside, true) }
  const outside = (e) => { if (!dd.contains(e.target)) close() }
  function sync() {
    const o = sel.selectedOptions[0]; lab.textContent = o ? o.textContent : ''
    ;[...menu.children].forEach((c) => c.classList.toggle('sel', c.dataset.value === sel.value))
  }
  function rebuild() {
    menu.innerHTML = ''
    for (const o of sel.options) {
      const it = document.createElement('div'); it.className = 'dd-opt'; it.dataset.value = o.value
      const m = o.textContent.match(/^(.*?)\s*\(([^)]+)\)\s*$/)   // split trailing "(count)" → dim, right-aligned
      const l = document.createElement('span'); l.className = 'dd-optlabel'; l.textContent = m ? m[1] : o.textContent
      it.append(l)
      if (m) { const meta = document.createElement('span'); meta.className = 'dd-meta'; meta.textContent = m[2]; it.append(meta) }
      it.addEventListener('click', () => { sel.value = o.value; sel.dispatchEvent(new Event('change')); sync(); close() })
      menu.append(it)
    }
    sync()
  }
  trig.addEventListener('click', () => { if (menu.hidden) { rebuild(); menu.hidden = false; trig.classList.add('open'); document.addEventListener('pointerdown', outside, true) } else close() })
  sel._sync = sync; rebuild()
  return sync
}
;['chainPick', 'colorMode', 'focus'].forEach((id) => makeDropdown($(id)))

// ── HUD + legend ─────────────────────────────────────────────────────────────
const m = data.meta
$('stats').innerHTML =
  `<div class="cell"><div class="n">${m.node_count ?? data.nodes.length}</div><div class="l">insights</div></div>` +
  `<div class="cell"><div class="n">${m.edge_count ?? data.edges.length}</div><div class="l">links</div></div>` +
  `<div class="cell"><div class="n">${m.initiative_count ?? data.initiatives.length}</div><div class="l">projects</div></div>` +
  `<div class="cell"><div class="n">${m.chain_count ?? data.chains.length}</div><div class="l">chains</div></div>` +
  `<div class="cell span">${fmtDate(T0)} — ${fmtDate(T1)}<span class="sub">one agent · ${Math.round((T1 - T0) / 86400)} days</span></div>`
function hex(c) { return '#' + c.getHexString() }
function buildLegend() {
  const el = $('legend')
  if (colorMode === 'initiative') {
    el.innerHTML = data.initiatives.slice(0, 8).map((i) => `${esc(i.name)} <span class="sw" style="background:${hex(initColor[i.name])}"></span>`).join('<br>') +
      `<br><span style="opacity:.6">+ ${Math.max(0, data.initiatives.length - 8)} more</span>`
  } else if (colorMode === 'tier') {
    el.innerHTML = `operational (hippocampus) <span class="sw" style="background:${TIER_COLOR.operational}"></span><br>archival (cortex) <span class="sw" style="background:${TIER_COLOR.archival}"></span>`
  } else {
    el.innerHTML = ['core', 'hot', 'warm', 'cold', 'frozen'].map((l) => `${l} <span class="sw" style="background:${LAYER_COLOR[l]}"></span>`).join('<br>')
  }
}
buildLegend()

// ── camera framing ───────────────────────────────────────────────────────────
function boundsOf(filter) {
  // geometric (bounding-box) centre so the densest cluster doesn't skew the
  // pivot — the galaxy then sits centred on screen and rotates about its middle.
  let mnx = Infinity, mny = Infinity, mnz = Infinity, mxx = -Infinity, mxy = -Infinity, mxz = -Infinity, any = false
  for (let i = 0; i < NN; i++) {
    const n = nodes[i]; if (n.isHub || !visible(n)) continue; if (filter && !filter(n)) continue
    const p = n.pos; any = true
    if (p.x < mnx) mnx = p.x; if (p.x > mxx) mxx = p.x
    if (p.y < mny) mny = p.y; if (p.y > mxy) mxy = p.y
    if (p.z < mnz) mnz = p.z; if (p.z > mxz) mxz = p.z
  }
  if (!any) return { c: new THREE.Vector3(), maxR: R }
  const c = new THREE.Vector3((mnx + mxx) / 2, (mny + mxy) / 2, (mnz + mxz) / 2)
  const maxR = Math.max(mxx - mnx, mxy - mny, mxz - mnz) / 2
  return { c, maxR }
}
function frame(ms = 1200) {
  const { c, maxR } = boundsOf(null)
  const D = maxR / Math.tan((camera.fov * Math.PI / 180) / 2) * 1.0
  camTween({ x: c.x, y: c.y, z: c.z + D }, c, ms)
}
function frameCluster(name, ms = 1400) {
  const { c, maxR } = boundsOf((n) => n.init === name)
  const D = Math.max(260, maxR / Math.tan((camera.fov * Math.PI / 180) / 2) * 1.8)
  controls.autoRotate = false
  camTween({ x: c.x, y: c.y, z: c.z + D }, c, ms)
}

// ── guided tour ──────────────────────────────────────────────────────────────
const SCENES = [
  { tag: 'a knowledge galaxy', title: 'One mind. A knowledge galaxy.',
    narr: "Months of one agent's work across many projects, in one memory. Same rule shapes it as a galaxy: what belongs together pulls together. Each cluster a project, each point a thought.",
    apply() { resetChain(); resetTime(); setFocus(null); setGlow(true); setColorMode('initiative'); frame(1500) } },
  { tag: 'reasoning chains', title: 'How — not just what.',
    narr: 'A knowledge chain is the load-bearing path between insights. Watch how one conclusion was reached — node by node, in order.',
    apply() { resetTime(); setFocus(null); const c = data.chains.reduce((b, x) => (x.members.length > (b ? b.members.length : 0) ? x : b), null); if (c) { chainPick.value = data.chains.indexOf(c); chainPick._sync && chainPick._sync(); startReplay(c) } } },
  { tag: 'one project, up close', title: 'Each cluster is a real project.',
    narr: 'Zoom into a single project and the structure appears: keystone facts in Core, standing rules in Hot, working notes in Warm — scoped and prioritized.',
    apply() { resetChain(); resetTime(); setGlow(true); setColorMode('layer'); setFocus((data.initiatives[0] || {}).name || null) } },
  { tag: 'memory layers', title: 'Important glows first.',
    narr: 'Memory has priority. Core and Hot glow largest and load on every re-entry; Warm is the working set; Cold and Frozen wait until asked.',
    apply() { resetChain(); resetTime(); setFocus(null); setGlow(true); setColorMode('layer'); frame(1200) } },
  { tag: 'two tiers', title: 'Hippocampus & cortex.',
    narr: 'Two tiers, like the brain. Operational is fast, messy working thought; archival is settled, durable knowledge. Operational decays and gets revisited; archival is what survives.',
    apply() { resetChain(); resetTime(); setFocus(null); setColorMode('tier'); frame(1200) } },
]
let scriptIdx = -1
function renderDots(i) { $('scriptDots').innerHTML = SCENES.map((_, k) => `<span class="dot${k === i ? ' on' : ''}"></span>`).join('') }
function gotoScene(i) {
  if (i < 0 || i >= SCENES.length) return
  scriptIdx = i; const s = SCENES[i]
  $('scriptTag').textContent = s.tag; $('scriptNum').textContent = `${i + 1} / ${SCENES.length}`
  $('scriptTitle').textContent = s.title; $('scriptNarr').textContent = s.narr
  $('scriptPrev').disabled = i === 0; $('scriptNext').disabled = i === SCENES.length - 1
  renderDots(i); s.apply()
}
function enterScript() { $('panel').hidden = true; $('script').hidden = false; controls.autoRotate = false; gotoScene(0) }
function exitScript() { $('script').hidden = true; $('panel').hidden = false; scriptIdx = -1; resetChain(); resetTime(); setFocus(null); setGlow(true); setColorMode('initiative') }
const nextScene = () => gotoScene(Math.min(scriptIdx + 1, SCENES.length - 1))
const prevScene = () => gotoScene(Math.max(scriptIdx - 1, 0))
$('talkBtn').addEventListener('click', enterScript)
$('scriptNext').addEventListener('click', nextScene)
$('scriptPrev').addEventListener('click', prevScene)
$('scriptExit').addEventListener('click', exitScript)
addEventListener('keydown', (e) => { if ($('script').hidden) return; if (e.key === 'ArrowRight' || e.key === ' ') { e.preventDefault(); nextScene() } else if (e.key === 'ArrowLeft') { e.preventDefault(); prevScene() } else if (e.key === 'Escape') exitScript() })

// ── loop ─────────────────────────────────────────────────────────────────────
addEventListener('resize', () => { camera.aspect = innerWidth / innerHeight; camera.updateProjectionMatrix(); renderer.setSize(innerWidth, innerHeight) })
let t = 0
function loop() {
  requestAnimationFrame(loop); t += 1
  if (performance.now() - lastMove > 80) speed = 0   // cursor settled
  const cand = (mouse && speed < SPEED) ? pick() : -1 // skip picking while sweeping fast / dragging
  if (cand !== hoverCand) { hoverCand = cand; candSince = performance.now() }
  let want = hovered
  if (cand < 0) want = -1
  else if (performance.now() - candSince >= HOVER_DELAY) want = cand  // dwell before highlight
  if (want !== hovered) setHover(want)
  else if (hovered >= 0) { _v.copy(nodes[hovered].pos).project(camera); chip.style.left = ((_v.x * 0.5 + 0.5) * innerWidth) + 'px'; chip.style.top = ((-_v.y * 0.5 + 0.5) * innerHeight) + 'px' }
  // bridge particles
  let k = 0
  for (const e of bridges) { const A = nodes[e.a].pos, B = nodes[e.b].pos; for (let j = 0; j < 3; j++) { const u = ((t * 0.0016) + j / 3) % 1; bpPos[k * 3] = A.x + (B.x - A.x) * u; bpPos[k * 3 + 1] = A.y + (B.y - A.y) * u + Math.sin(u * Math.PI) * 40; bpPos[k * 3 + 2] = A.z + (B.z - A.z) * u; k++ } }
  bpGeo.attributes.position.needsUpdate = true
  if (tween) tween()
  // auto-rotate only when idle — stop on hover, chain, focus, tour, camera tween
  controls.autoRotate = !(window.__rec && window.__rec.active) &&
    hovered < 0 && !chain.size && !focusInit && $('script').hidden && !tween
  controls.update(); renderer.render(scene, camera)
}

// recording hook — only present with ?rec, drives a deterministic seamless orbit
if (new URLSearchParams(location.search).has('rec')) {
  let base = null
  window.__rec = {
    active: false,
    orbit(t) {
      this.active = true
      const o = controls.target
      if (!base) { const off = camera.position.clone().sub(o); base = { r: Math.hypot(off.x, off.z), y: off.y } }
      const a = t * Math.PI * 2
      camera.position.set(o.x + Math.sin(a) * base.r, o.y + base.y, o.z + Math.cos(a) * base.r)
      camera.lookAt(o); renderer.render(scene, camera)
    },
  }
}

applyVisuals(); frame(0); loop()
