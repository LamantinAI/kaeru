import ForceGraph3D from '3d-force-graph'

// ── palettes ────────────────────────────────────────────────────────────────
const EDGE_COLORS = {
  derived_from: '#74d0ff', refers_to: '#8aa0ff', causal: '#ffb35c', supersedes: '#ff8f8f',
  verifies: '#6cf0a0', falsifies: '#ff7ac0', contradicts: '#ff5555', part_of: '#a8d97a',
  temporal: '#9aa6c8', consolidated_to: '#ffd166', blocks: '#ff9a52', targets: '#67d3ff',
}
const LAYER_SIZE = { core: 9, hot: 5, warm: 2.2, cold: 1.4, frozen: 0.9 }
const LAYER_COLOR = { core: '#ffd34d', hot: '#ff7a45', warm: '#5b8cff', cold: '#3f4c78', frozen: '#2a3150' }
const TIER_COLOR = { operational: '#ff6ad5', archival: '#56d0ff' }
const DIM = 'rgba(86,98,140,0.10)'

const esc = (s) => String(s ?? '').replace(/[&<>]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;' }[c]))
const fmtDate = (secs) => new Date(secs * 1000).toISOString().slice(0, 10)

// ── data ────────────────────────────────────────────────────────────────────
async function loadGraph() {
  try { const r = await fetch('/graph.json'); if (r.ok) return await r.json() } catch (_) {}
  const r = await fetch('./graph.json'); return await r.json()  // baked snapshot fallback
}
const data = await loadGraph()
// `chain` nodes are meta-trails (a saved reasoning path), not insights — they
// drive the replay selector via data.chains[].members, but must not render as
// loose floating points in the galaxy. Keep only real insight nodes.
const nodes = data.nodes.filter((n) => n.type !== 'chain')
const byId = new Map(nodes.map((n) => [n.id, n]))
const links = data.edges.map((e) => ({ source: e.src, target: e.dst, type: e.type, weight: e.weight, created: e.created_secs }))
const primInit = (n) => (n.isHub ? n.hubInit : (n.initiatives && n.initiatives[0]) || '∅')

// initiative colors + cluster centers on a Fibonacci sphere
const initNames = data.initiatives.map((i) => i.name)
const initColor = {}, initCenter = {}
const R = 620
initNames.forEach((name, i) => {
  initColor[name] = `hsl(${(i * 137.508) % 360} 72% 62%)`
  const y = initNames.length > 1 ? 1 - (i / (initNames.length - 1)) * 2 : 0
  const rad = Math.sqrt(Math.max(0, 1 - y * y)), theta = i * 2.39996
  initCenter[name] = { x: Math.cos(theta) * rad * R, y: y * R, z: Math.sin(theta) * rad * R }
})

// ── recomputed cross-project affinity ────────────────────────────────────────
// kaeru-core's project_affinity uses an EN-only stop-list, so for a RU vault the
// author's name, dates, numbers and RU generic words leak in and dominate the
// bridges. Recompute here with a cleaner, multilingual filter (same algorithm:
// inverse-frequency over topics shared by 2..8 initiatives, top 70, normalized).
const TOPIC_STOP = new Set([
  'fix','test','build','first','after','phase','root','running','merged','master','earlier',
  'correction','bug','finding','the','name','local','new','final','initial','via','not','and','---',
  'post','done','wip','todo','update','plan','status','draft','note','notes','task','step','work',
  'иван','ивана','ивану','иваном','иване','григорий','григория',
  'решение','решения','решений','полный','полная','полное','полностью','два','две','три','все','всё','всех',
  'ветка','ветки','ветку','коммит','коммита','коммиты','отдельный','отдельная','отдельно','общий',
  'репозиторий','репозитория','проект','проекта','проекты','проекте','новый','новая','новые','новое',
  'факт','факты','фактов','документ','документы','документов','заказчик','заказчика','клиент',
  'версия','версии','текущий','рабочий','статус','план','планы','задача','задачи','вопрос','вопросы',
  'ответ','итог','итоги','дата','имя','правка','правки','правок','ошибка','сборка','первый','после','фаза','корень',
  'мерж','мастер','ранее','через','этот','это','для','при','что','как','уже','есть','один','день',
  'веток','тест','теста','тесты','тестов','работает','прогресс','апдейт','разбор','запись',
  'main','live','across','three','done','wip','feat',
])
const topicWord = (t) => (t.startsWith('topic:') ? t.slice(6) : null)
const goodTopic = (w) =>
  !!w && w.length >= 4 && !TOPIC_STOP.has(w) &&
  !/^\d+$/.test(w) && !/\d{4}/.test(w) && !/\d{1,2}[-.]\d{1,2}/.test(w)
function recomputeProjectLinks(ns) {
  const topicInits = new Map()
  for (const n of ns) {
    const init = n.initiatives && n.initiatives[0]; if (!init) continue
    for (const t of n.tags || []) {
      const w = topicWord(t); if (!goodTopic(w)) continue
      let s = topicInits.get(w); if (!s) { s = new Set(); topicInits.set(w, s) } s.add(init)
    }
  }
  const aff = new Map()
  for (const s of topicInits.values()) {
    const span = s.size; if (span < 2 || span > 8) continue
    const wgt = 1 / span, v = [...s].sort()
    for (let i = 0; i < v.length; i++) for (let j = i + 1; j < v.length; j++) {
      const k = v[i] + '|' + v[j]; aff.set(k, (aff.get(k) || 0) + wgt)
    }
  }
  let pairs = [...aff.entries()].map(([k, w]) => { const [a, b] = k.split('|'); return { a, b, weight: w } })
  pairs.sort((x, y) => y.weight - x.weight); pairs = pairs.slice(0, 70)
  const max = pairs.length ? pairs[0].weight : 1
  for (const p of pairs) p.weight /= max
  return pairs
}
data.project_links = recomputeProjectLinks(nodes)

// ── cross-project relatedness (from the export's project_links) ──────────────
// The projects aren't silos — they share subject matter that was never captured
// as explicit edges. The export derives inter-project affinity from shared
// topics (kaeru_core::project_affinity); we render it as a constellation between
// per-project hub nodes pinned at each cluster center.
const countByInit = Object.fromEntries(data.initiatives.map((i) => [i.name, i.node_count]))
const hubNodes = initNames.map((name) => {
  const c = initCenter[name]
  return { id: '__hub__' + name, isHub: true, hubInit: name, name, count: countByInit[name] || 1,
    x: c.x, y: c.y, z: c.z, fx: c.x, fy: c.y, fz: c.z }
})
const projLinks = (data.project_links || []).map((p) => ({ source: '__hub__' + p.a, target: '__hub__' + p.b, isProj: true, w: p.weight }))
nodes.push(...hubNodes)
hubNodes.forEach((h) => byId.set(h.id, h))
links.push(...projLinks)

// ── state ───────────────────────────────────────────────────────────────────
let colorMode = 'initiative', glow = true, focusInit = null, crossMode = false
const T0 = data.meta.earliest_secs ?? 0, T1 = data.meta.latest_secs ?? 0
let timeFilter = Infinity
const chain = new Set()                  // active replay member ids
let replayTimer = null

// ── accessors ───────────────────────────────────────────────────────────────
function nodeColor(n) {
  if (n.isHub) return (crossMode || (!chain.size && !focusInit)) ? (initColor[n.hubInit] || '#9af') : 'rgba(0,0,0,0)'
  if (crossMode) return DIM
  if (chain.size) {
    if (!chain.has(n.id)) return DIM
    if (n.__cur) return '#ffffff'
    return n.__visited ? '#ffd34d' : (initColor[primInit(n)] || '#9af')
  }
  if (focusInit && primInit(n) !== focusInit) return DIM
  if (colorMode === 'tier') return TIER_COLOR[n.tier] || '#999'
  if (colorMode === 'layer') return LAYER_COLOR[n.layer] || '#888'
  return initColor[primInit(n)] || '#888'
}
function nodeVal(n) {
  if (n.isHub) return crossMode ? 5 + Math.sqrt(n.count) * 1.4 : ((!chain.size && !focusInit) ? 3 + Math.sqrt(n.count) * 0.7 : 0)
  if (crossMode) return 0.5
  let s = LAYER_SIZE[n.layer] ?? 2
  if (glow && (n.layer === 'core' || n.layer === 'hot')) s *= 1.4
  if (chain.has(n.id)) s = Math.max(s, 6)
  return s
}
const visN = (n) => (n.isHub ? (crossMode || (!chain.size && !focusInit)) : (n.created_secs ?? 0) <= timeFilter)
function visL(l) {
  if (l.isProj) return crossMode || (!chain.size && !focusInit && l.w >= 0.12)
  if (crossMode) return false
  const s = typeof l.source === 'object' ? l.source : byId.get(l.source)
  const t = typeof l.target === 'object' ? l.target : byId.get(l.target)
  if (!s || !t || !visN(s) || !visN(t)) return false
  if (chain.size) return chain.has(s.id) && chain.has(t.id)
  if (focusInit) return primInit(s) === focusInit && primInit(t) === focusInit
  return true
}
const linkColor = (l) => {
  if (!l.isProj) return chain.size ? '#ffd34d' : (EDGE_COLORS[l.type] || '#566')
  const a = crossMode ? 0.55 + l.w * 0.4 : 0.42 + l.w * 0.5    // bright amber bridges that pop
  return `rgba(255,190,90,${a.toFixed(2)})`
}
const linkWidth = (l) => {
  if (!l.isProj) return chain.size ? 2.5 : 0.4 + (l.weight || 0.5) * 1.6
  return crossMode ? 1 + l.w * 7 : 1.2 + l.w * 5              // thick, weight-scaled bridges
}

// ── graph ───────────────────────────────────────────────────────────────────
// OrbitControls (not the default TrackballControls) — it's the one with
// `autoRotate`, which the galaxy spin relies on.
const Graph = ForceGraph3D({ controlType: 'orbit' })(document.getElementById('graph'))
  .backgroundColor('#05060d')
  .graphData({ nodes, links })
  .nodeId('id')
  .nodeLabel((n) => n.isHub
    ? `<b>${esc(n.name)}</b><br><span style="color:#9aa">${n.count} insights · project</span>`
    : `<b>${esc(n.name)}</b><br><span style="color:#9aa">${n.type} · ${n.tier} · ${n.layer} · ${esc(primInit(n))}</span>`)
  .nodeColor(nodeColor).nodeVal(nodeVal).nodeOpacity(0.92).nodeResolution(10)
  .nodeVisibility(visN).linkVisibility(visL)
  .linkColor(linkColor).linkWidth(linkWidth).linkOpacity(0.4)
  .linkCurvature((l) => (l.isProj ? 0.3 : 0))
  .linkDirectionalParticles((l) => (l.isProj && visL(l) ? Math.max(2, Math.round(l.w * 5)) : 0))
  .linkDirectionalParticleWidth((l) => (l.isProj ? 2.6 : 2)).linkDirectionalParticleSpeed(0.012)
  .linkDirectionalParticleColor((l) => (l.isProj ? 'rgba(255,214,140,0.95)' : '#ffd34d'))
  .onNodeClick((n) => showReadout(n))
  .warmupTicks(60).cooldownTicks(220)

// pull nodes toward their initiative cluster center, and on the same pass park
// each project core (hub) at the LIVE centroid of its cloud so it sits dead
// center — not at the fixed sphere anchor the cloud may have drifted from.
const cluster = (alpha) => {
  const acc = {}
  for (const n of nodes) {
    if (n.isHub || n.x == null) continue
    const init = primInit(n)
    const c = initCenter[init]; if (!c) continue
    const k = 0.05 * alpha
    n.vx += (c.x - n.x) * k; n.vy += (c.y - n.y) * k; n.vz += (c.z - n.z) * k
    if ((n.created_secs ?? 0) > timeFilter) continue   // only visible nodes count
    const a = acc[init] || (acc[init] = { x: 0, y: 0, z: 0, n: 0 })
    a.x += n.x; a.y += n.y; a.z += n.z; a.n++
  }
  for (const h of hubNodes) {
    const a = acc[h.hubInit]
    if (a && a.n) { h.fx = h.x = a.x / a.n; h.fy = h.y = a.y / a.n; h.fz = h.z = a.z / a.n }
  }
}
cluster.initialize = () => {}
Graph.d3Force('cluster', cluster)
Graph.d3Force('charge').strength((n) => (n.isHub ? 0 : -14))

const refresh = () => Graph
  .nodeColor(nodeColor).nodeVal(nodeVal)
  .linkColor(linkColor).linkWidth(linkWidth)
  .nodeVisibility(visN).linkVisibility(visL)
  .linkDirectionalParticles((l) => (l.isProj && visL(l) ? Math.max(2, Math.round(l.w * 5)) : (chain.size && visL(l) ? 4 : 0)))

// ── camera ──────────────────────────────────────────────────────────────────
function flyTo(p, lookAt, ms = 1400) {
  const d = Math.hypot(p.x, p.y, p.z) || 1
  const ratio = 1 + 160 / d
  Graph.cameraPosition({ x: p.x * ratio, y: p.y * ratio, z: p.z * ratio }, lookAt || p, ms)
}

// ── chain replay (hero) ─────────────────────────────────────────────────────
function startReplay(c) {
  stopReplay()
  const members = c.members.filter((id) => byId.has(id))
  if (members.length < 2) return
  chain.clear(); members.forEach((id) => chain.add(id))
  nodes.forEach((n) => { n.__visited = false; n.__cur = false })
  timeFilter = Infinity; document.getElementById('time').value = 100; timeLabel('— full graph —')
  refresh()
  let i = 0
  const step = () => {
    if (i > 0) { const p = byId.get(members[i - 1]); p.__cur = false; p.__visited = true }
    if (i >= members.length) { const last = byId.get(members[members.length - 1]); showReadout(last, members.length, members.length); refresh(); return }
    const cur = byId.get(members[i]); cur.__cur = true; cur.__visited = true
    showReadout(cur, i + 1, members.length)
    flyTo(cur, cur, 1200)
    refresh()
    i += 1
    replayTimer = setTimeout(step, 2400)
  }
  step()
}
function stopReplay() { if (replayTimer) { clearTimeout(replayTimer); replayTimer = null } }
function resetChain() {
  stopReplay(); chain.clear()
  nodes.forEach((n) => { n.__visited = false; n.__cur = false })
  refresh(); hideReadout()
  Graph.zoomToFit(900, 80)
}

// ── readout panel ───────────────────────────────────────────────────────────
const readout = document.getElementById('readout')
function showReadout(n, step, total) {
  const stepLine = step ? `step ${step} / ${total} — knowledge chain` : 'node'
  readout.innerHTML = `
    <div class="step">${esc(stepLine)}</div>
    <h2>${esc(n.name)}</h2>
    <div class="meta">
      <span class="pill">${esc(n.type)}</span><span class="pill">${esc(n.tier)}</span>
      <span class="pill">${esc(n.layer)}</span>${(n.initiatives || []).map((i) => `<span class="pill">${esc(i)}</span>`).join('')}
    </div>
    <p>${esc(n.body || (n.redacted ? '⟨body redacted⟩' : '—'))}</p>`
  readout.classList.add('show')
}
const hideReadout = () => readout.classList.remove('show')

// ── time-lapse ──────────────────────────────────────────────────────────────
const timeEl = document.getElementById('time')
const timeLabel = (s) => (document.getElementById('timeLabel').textContent = s)
function applyTime(pct) {
  if (pct >= 100) { timeFilter = Infinity; timeLabel('— full graph —') }
  else { timeFilter = T0 + (pct / 100) * (T1 - T0); timeLabel(`${fmtDate(T0)} → ${fmtDate(timeFilter)}`) }
  refresh()
}
timeEl.addEventListener('input', (e) => applyTime(+e.target.value))
let timeAnim = null
function stopTimeLapse() { if (timeAnim) { clearInterval(timeAnim); timeAnim = null } }
function startTimeLapse() {
  stopTimeLapse()
  let v = 0; timeEl.value = 0; applyTime(0)
  timeAnim = setInterval(() => { v += 1.5; if (v >= 100) { v = 100; stopTimeLapse() } timeEl.value = v; applyTime(v) }, 60)
}
function resetTime() { stopTimeLapse(); timeEl.value = 100; applyTime(100) }
document.getElementById('timePlay').addEventListener('click', () => (timeAnim ? stopTimeLapse() : startTimeLapse()))

// ── controls ────────────────────────────────────────────────────────────────
const chainPick = document.getElementById('chainPick')
data.chains.forEach((c, i) => { const o = document.createElement('option'); o.value = i; o.textContent = `${c.name} (${c.members.length})`; chainPick.appendChild(o) })
document.getElementById('chainPlay').addEventListener('click', () => { const c = data.chains[+chainPick.value || 0]; if (c) startReplay(c) })
document.getElementById('chainReset').addEventListener('click', resetChain)
document.getElementById('colorMode').addEventListener('change', (e) => { colorMode = e.target.value; refresh(); buildLegend() })
document.getElementById('glow').addEventListener('change', (e) => { glow = e.target.checked; refresh() })
const focusEl = document.getElementById('focus')
data.initiatives.forEach((i) => { const o = document.createElement('option'); o.value = i.name; o.textContent = `${i.name} (${i.node_count})`; focusEl.appendChild(o) })
focusEl.addEventListener('change', (e) => setFocus(e.target.value))

// ── HUD + legend ────────────────────────────────────────────────────────────
const m = data.meta
document.getElementById('stats').innerHTML =
  `<b>${m.node_count}</b> insights · <b>${m.edge_count}</b> links · <b>${m.initiative_count}</b> projects · <b>${m.chain_count}</b> chains<br>` +
  `${fmtDate(T0)} → ${fmtDate(T1)} · one agent, ${Math.round((T1 - T0) / 86400)} days`

function buildLegend() {
  const el = document.getElementById('legend')
  if (colorMode === 'initiative') {
    const top = data.initiatives.slice(0, 8)
    el.innerHTML = top.map((i) => `${esc(i.name)} <span class="sw" style="background:${initColor[i.name]}"></span>`).join('<br>') + '<br><span style="opacity:.6">+ ' + Math.max(0, data.initiatives.length - 8) + ' more</span>'
  } else if (colorMode === 'tier') {
    el.innerHTML = `operational (hippocampus) <span class="sw" style="background:${TIER_COLOR.operational}"></span><br>archival (cortex) <span class="sw" style="background:${TIER_COLOR.archival}"></span>`
  } else {
    el.innerHTML = ['core', 'hot', 'warm', 'cold', 'frozen'].map((l) => `${l} <span class="sw" style="background:${LAYER_COLOR[l]}"></span>`).join('<br>')
  }
}
buildLegend()

// gentle auto-orbit until the user interacts
Graph.controls().autoRotate = true
Graph.controls().autoRotateSpeed = 0.5
document.getElementById('graph').addEventListener('pointerdown', () => { Graph.controls().autoRotate = false }, { once: true })

// frame the galaxy nicely on first load — lifted above the bottom panel so the
// controls never occlude it (the app only framed inside Demo/Focus before).
let __framedOnce = false
Graph.onEngineStop(() => { if (!__framedOnce) { __framedOnce = true; frameGalaxyInitial(1200) } })

// re-centre on window resize so the galaxy never leaves dead space (skip during
// the guided demo / focus / chain replay, which drive their own camera).
let __rzT
addEventListener('resize', () => {
  clearTimeout(__rzT)
  __rzT = setTimeout(() => {
    if (document.getElementById('script').hidden && !focusInit && !chain.size) frameGalaxyInitial(400)
  }, 220)
})

// ── talk mode: a guided tour of the wow-moments ──────────────────────────────
// Each scene narrates one beat and drives the viz into the matching state.
const $ = (id) => document.getElementById(id)
function setColorMode(m) { colorMode = m; $('colorMode').value = m; buildLegend(); refresh() }
function setGlow(b) { glow = b; $('glow').checked = b; refresh() }
function setFocus(name) {
  focusInit = name || null
  $('focus').value = name || ''
  refresh()
  if (focusInit) Graph.zoomToFit(1400, 90, (n) => primInit(n) === focusInit && !n.isHub)
  else Graph.zoomToFit(1200, 90, (n) => !n.isHub)
}
function setCross(b) { crossMode = b; refresh() }
function setSpin(on, speed = 0.55) { const c = Graph.controls(); c.autoRotate = on; c.autoRotateSpeed = speed }
function clearFocus() { focusInit = null; $('focus').value = ''; refresh() }
// Centre of mass + robust radius of the visible cloud. Origin is NOT the centroid
// (14 clusters of different sizes on a sphere skew it), so framing on origin left
// dead space on one side. Uses a 90th-percentile radius so a few far/lonely
// clusters don't shrink the whole galaxy into the middle of a big screen.
function galaxyBounds() {
  const pts = []
  let cx = 0, cy = 0, cz = 0
  for (const n of nodes) {
    if (n.isHub || n.x == null || (n.created_secs ?? 0) > timeFilter) continue
    pts.push(n); cx += n.x; cy += n.y; cz += n.z
  }
  if (pts.length) { cx /= pts.length; cy /= pts.length; cz /= pts.length }
  const ds = pts.map((n) => Math.hypot(n.x - cx, n.y - cy, n.z - cz)).sort((a, b) => a - b)
  const r = ds.length ? ds[Math.floor(ds.length * 0.9)] : 1
  return { cx, cy, cz, maxR: Math.max(1, r) }
}
// Pivot the look-point on the centroid so the galaxy is always centred (auto-rotate
// orbits this point, so it stays centred while spinning). Fully adaptive: sizes +
// centres the galaxy into the CLEAR BAND between the fixed HUD (top) and panel /
// demo-card (bottom). The chrome is fixed px, so on small screens it eats a bigger
// fraction and the galaxy auto-shrinks to stay clear — works on any monitor
// (2K / ultrawide / laptop / portrait).
const NODE_SPREAD = 0.86  // visible node spread ≈ 86% of the framed bounding-sphere
function frameGalaxy(ms, fillBand, botR = 175) {
  const { cx, cy, cz, maxR } = galaxyBounds()
  const cam = Graph.camera()
  const vfov = (((cam && cam.fov) || 50) * Math.PI) / 180
  const tanV = Math.tan(vfov / 2)
  const W = window.innerWidth, H = window.innerHeight
  const aspect = (cam && cam.aspect) || (W / H)
  const topR = 150
  const usableH = Math.max(220, H - topR - botR)
  const Dh = (NODE_SPREAD * maxR * H) / (tanV * usableH * fillBand)   // fill the band by height
  const Dw = (NODE_SPREAD * maxR) / (tanV * aspect * 0.9)             // width cap (tall screens)
  const D = Math.max(Dh, Dw)
  const bandCenterFrac = (topR + usableH / 2) / H                     // centre in the clear band
  const up = D * tanV * (1 - 2 * bandCenterFrac)
  Graph.cameraPosition({ x: cx, y: cy - up, z: cz + D }, { x: cx, y: cy - up, z: cz }, ms)
}
function frameGalaxyInitial(ms = 1200) { frameGalaxy(ms, 0.96, 165) }
function frameGalaxyHigh(ms = 1400) { frameGalaxy(ms, 0.82, 230) }

const SCENES = [
  {
    tag: 'a knowledge galaxy',
    title: 'One mind. A knowledge galaxy.',
    narr: "Months of one AI agent's work across many open-source projects, all in one memory. It forms a galaxy because the same rule shapes both: what belongs together, pulls together. Each cluster a project, each point a thought — a small universe of one mind's work.",
    apply() { resetChain(); resetTime(); clearFocus(); setCross(false); setGlow(true); setColorMode('initiative'); frameGalaxyHigh(1600); setSpin(true, 0.55) },
  },
  {
    tag: 'bi-temporal substrate',
    title: 'Watch the knowledge grow.',
    narr: 'Every node and edge is bi-temporal — we record exactly when each insight was asserted. So we can rewind and replay months of thinking accumulating, project by project.',
    apply() { resetChain(); clearFocus(); setCross(false); setSpin(false); setColorMode('initiative'); frameGalaxyHigh(); startTimeLapse() },
  },
  {
    tag: 'reasoning chains',
    title: 'How — not just what.',
    narr: 'Reasoning is preserved, not just results. A knowledge chain is the load-bearing path between insights. Watch how one conclusion was actually reached — node by node, in order.',
    apply() {
      resetTime(); clearFocus(); setCross(false); setSpin(false)
      // Demo the richest trail: the longest chain (name-free, always best).
      const c = data.chains.reduce((best, x) => (x.members.length > (best ? best.members.length : 0) ? x : best), null)
      if (c) { chainPick.value = data.chains.indexOf(c); startReplay(c) }
    },
  },
  {
    tag: 'cross-project knowledge',
    title: 'The projects relate.',
    narr: "These aren't silos. Each line connects two projects that share subject matter — weighted toward the specific topics, so the real domain links surface: shared hardware families, shared protocol stacks, shared tooling. One agent's memory spanning a connected domain.",
    apply() { resetChain(); resetTime(); setFocus(null); setColorMode('initiative'); setCross(true); setSpin(true, 0.3); Graph.zoomToFit(1700, 130) },
  },
  {
    tag: 'one project, up close',
    title: 'Each cluster is a real project.',
    narr: "Zoom into a single project — the largest in this vault. Colour by layer and the structure appears: the keystone facts in Core (gold), the standing rules in Hot, the working notes in Warm. One project's whole memory, scoped and prioritized.",
    apply() { resetChain(); resetTime(); setCross(false); setSpin(false); setGlow(true); setColorMode('layer'); setFocus((data.initiatives[0] || {}).name || null) },
  },
  {
    tag: 'memory layers',
    title: 'Important glows first.',
    narr: 'Memory has priority. Core and Hot glow largest and load on every re-entry; Warm is the working set; Cold and Frozen wait until explicitly asked. The important stuff surfaces first.',
    apply() { resetChain(); resetTime(); clearFocus(); setCross(false); setSpin(false); setGlow(true); setColorMode('layer'); frameGalaxyHigh() },
  },
  {
    tag: 'two tiers',
    title: 'Hippocampus & cortex.',
    narr: 'Two tiers, like the brain. Operational (hippocampus) is fast, messy working thought. Archival (cortex) is settled, durable knowledge. Operational decays and gets revisited; archival is what survives.',
    apply() { resetChain(); resetTime(); clearFocus(); setCross(false); setSpin(false); setColorMode('tier'); frameGalaxyHigh() },
  },
]

let scriptIdx = -1
function renderDots(i) {
  $('scriptDots').innerHTML = SCENES.map((_, k) => `<span class="dot${k === i ? ' on' : ''}"></span>`).join('')
}
function gotoScene(i) {
  if (i < 0 || i >= SCENES.length) return
  scriptIdx = i
  const s = SCENES[i]
  $('scriptTag').textContent = s.tag
  $('scriptNum').textContent = `${i + 1} / ${SCENES.length}`
  $('scriptTitle').textContent = s.title
  $('scriptNarr').textContent = s.narr
  $('scriptPrev').disabled = i === 0
  $('scriptNext').disabled = i === SCENES.length - 1
  renderDots(i)
  s.apply()
}
function enterScript() {
  $('panel').hidden = true
  $('script').hidden = false
  Graph.controls().autoRotate = false
  gotoScene(0)
}
function exitScript() {
  $('script').hidden = true
  $('panel').hidden = false
  scriptIdx = -1
  resetChain(); resetTime(); setFocus(null); setCross(false); setSpin(false); setGlow(true); setColorMode('initiative')
}
const nextScene = () => gotoScene(Math.min(scriptIdx + 1, SCENES.length - 1))
const prevScene = () => gotoScene(Math.max(scriptIdx - 1, 0))

$('talkBtn').addEventListener('click', enterScript)
$('scriptNext').addEventListener('click', nextScene)
$('scriptPrev').addEventListener('click', prevScene)
$('scriptExit').addEventListener('click', exitScript)
window.addEventListener('keydown', (e) => {
  if ($('script').hidden) return
  if (e.key === 'ArrowRight' || e.key === ' ') { e.preventDefault(); nextScene() }
  else if (e.key === 'ArrowLeft') { e.preventDefault(); prevScene() }
  else if (e.key === 'Escape') { exitScript() }
})
