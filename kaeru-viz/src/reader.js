// The reader — the galaxy's second view.
//
// The galaxy answers *where* a thought sits; the reader answers *what it says*.
// It runs on the same feed, the same tokens and the same theme control, and it
// is driven by the same chain picker, so the two views are one instrument.
//
// Layout grammar: **X is how far the reasoning was carried** (depth of
// derivation) and **Y is which line of inquiry** (lane). A plain chain is just
// a one-lane DAG, so a linear trail and a branching deep-research share one
// layout instead of two.

// same layer ramp the galaxy's readout bar uses
const LAYER_ACCENT = { core: '#caa24a', hot: '#c8402e', warm: '#7e96cf', cold: '#6f7da0', frozen: '#8a8578' }
const REL = {
  derived_from: ['derived from', 'grounds this'], refers_to: ['refers to', 'referenced here'],
  supersedes: ['supersedes', 'superseded by'], verifies: ['verifies', 'verified in'],
  causal: ['led to', 'follows from'], part_of: ['part of', 'contains'],
  temporal: ['before', 'after'], contradicts: ['contradicts', 'contradicts'],
  blocks: ['blocks', 'blocked by'], targets: ['targets', 'targeted by'],
}
const STEP_DX = 470, LANE_DY = 300, SPINE_Y = 300, X0 = 120

const $ = (id) => document.getElementById(id)
const esc = (s) => (s || '').replace(/[&<>]/g, (m) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;' }[m]))
const deslug = (n) => (n || '').replace(/-20\d\d-\d\d-\d\d$/, '').replace(/-[0-9a-f]{6}$/, '').replace(/-/g, ' ')
const clamp = (v, a, b) => Math.min(b, Math.max(a, v))

export function createReader(data, { fmtDateTime }) {
  const world = $('rworld'), track = $('rtrack'), canvas = $('rcanvas'), sheet = $('manuscript')
  const N = {}, ADJ = {}
  data.nodes.filter((n) => n.type !== 'chain').forEach((n) => { N[n.id] = { ...n }; ADJ[n.id] = [] })
  for (const e of data.edges) {
    if (!N[e.src] || !N[e.dst]) continue
    ADJ[e.src].push({ o: e.dst, t: e.type, dir: 'out' })
    ADJ[e.dst].push({ o: e.src, t: e.type, dir: 'in' })
  }

  let chain = null, steps = [], POS = {}, lanes = 0, shared = new Map(), research = new Set()
  let hist = []                 // trail of what was read before this
  let tx = 60, ty = 40, scale = 0.62, view = 'trail'
  let bodiesLoaded = false

  // ── text ────────────────────────────────────────────────────────────────
  const inline = (t) => esc(t)
    .replace(/\[\[([^\]]+)\]\]/g, (m, x) => `<span class="rmark">${deslug(x)}</span>`)
    .replace(/`([^`]+)`/g, '<code>$1</code>')
  const paras = (b) => (b || '').trim().split(/\n{2,}/)
    .map((p) => `<p>${inline(p.trim()).replace(/\n/g, '<br>')}</p>`).join('')
  const clip = (b, n) => {
    const t = (b || '').replace(/\s+/g, ' ').trim()
    return t.length > n ? inline(t.slice(0, n).replace(/\s\S*$/, '')) + '…' : inline(t)
  }
  const when = (n) => (n.created_secs ? fmtDateTime(n.created_secs) : '—')
  const h = (s) => { const d = document.createElement('div'); d.innerHTML = s.trim(); return d.firstChild }

  // The galaxy only needs excerpts; a reader needs the whole body. Pay for the
  // heavier payload once, the first time the reader is actually opened.
  async function loadBodies() {
    if (bodiesLoaded) return
    bodiesLoaded = true
    for (const url of ['/graph.json?bodies=true', './graph.json?bodies=true']) {
      try {
        const r = await fetch(url)
        if (!r.ok) continue
        const g = await r.json()
        for (const n of g.nodes) if (N[n.id] && n.body) N[n.id].body = n.body
        return
      } catch (_) { /* fall through to the baked snapshot */ }
    }
  }

  // ── layout: derivation DAG, lanes by path decomposition ─────────────────
  const derivEdges = () => {
    const out = []
    for (const id in ADJ) for (const l of ADJ[id]) if (l.dir === 'out' && l.t === 'derived_from') out.push({ from: l.o, to: id })
    return out
  }
  function layout() {
    POS = {}
    const E = derivEdges(), parents = {}
    research = new Set(steps.map((s) => s.id))
    // pull in whatever the chain's own steps derive from or feed into
    for (let hop = 0; hop < 2; hop++) {
      for (const e of E) {
        if (research.has(e.from)) research.add(e.to)
        if (research.has(e.to)) research.add(e.from)
      }
    }
    for (const e of E) if (research.has(e.from) && research.has(e.to)) (parents[e.to] = parents[e.to] || []).push(e.from)

    const depth = {}
    const visit = (id, seen) => {
      if (depth[id] != null) return depth[id]
      if (seen.has(id)) return 0
      seen.add(id)
      const ps = (parents[id] || []).filter((p) => research.has(p))
      depth[id] = ps.length ? Math.max(...ps.map((p) => visit(p, seen))) + 1 : 0
      return depth[id]
    }
    ;[...research].forEach((id) => visit(id, new Set()))

    // A lane must be a REAL chain of parent→child. Slotting by depth instead
    // drops unrelated threads onto one rail, and the rail then draws a
    // continuity that does not exist in the graph.
    const byDepth = {}
    ;[...research].forEach((id) => { (byDepth[depth[id]] = byDepth[depth[id]] || []).push(id) })
    const lane = {}, passedOn = new Set()
    let next = 0
    Object.keys(byDepth).map(Number).sort((a, b) => a - b).forEach((d) => {
      byDepth[d].sort((a, b) => (N[a]?.created_secs || 0) - (N[b]?.created_secs || 0))
      byDepth[d].forEach((id) => {
        const p = (parents[id] || []).find((pp) => lane[pp] != null && !passedOn.has(pp))
        if (p != null) { lane[id] = lane[p]; passedOn.add(p) } else lane[id] = next++
      })
    })
    const onChain = new Set(steps.map((s) => s.id))
    const chainLanes = new Set([...onChain].map((id) => lane[id]).filter((v) => v != null))
    const order = [...new Set(Object.values(lane))]
      .sort((a, b) => (chainLanes.has(b) ? 1 : 0) - (chainLanes.has(a) ? 1 : 0) || a - b)
    const rank = {}; order.forEach((l, i) => { rank[l] = i })
    lanes = order.length
    ;[...research].forEach((id) => {
      POS[id] = { x: X0 + depth[id] * STEP_DX, y: SPINE_Y - 70 + rank[lane[id]] * LANE_DY, d: depth[id], lane: rank[lane[id]] }
    })
    // anything several lines lean on is stated once, as the trail's bedrock
    const touch = new Map()
    ;[...research].forEach((id) => {
      for (const l of ADJ[id] || []) {
        if (research.has(l.o)) continue
        if (!touch.has(l.o)) touch.set(l.o, { n: 0, steps: [] })
        const t = touch.get(l.o); t.n++; t.steps.push(id)
      }
    })
    shared = new Map([...touch].filter(([, v]) => v.n > 1))
  }

  // ── build ───────────────────────────────────────────────────────────────
  function buildTrail() {
    world.querySelectorAll('.step,.sat,.lanetag,.bedlabel').forEach((e) => e.remove())
    if (!steps.length) return
    layout()
    const onChain = new Set(steps.map((s) => s.id))
    ;[...research].sort((a, b) => POS[a].d - POS[b].d).forEach((id) => {
      const n = N[id]; if (!n || !POS[id]) return
      const { x, y } = POS[id], acc = LAYER_ACCENT[n.layer] || 'var(--dim)'
      const main = onChain.has(id), idx = steps.findIndex((s) => s.id === id)
      // You opened the reader to read: the body starts OPEN. Folding is the
      // opt-in, for when a long trail needs to be scanned rather than read.
      const el = h(`<button type="button" class="step${main ? '' : ' aside'}" data-id="${id}"
          style="left:${x}px;top:${y}px;--acc:${acc}"
          aria-label="Read ${esc(deslug(n.name))}">
        <span class="meta"><span class="num">${main ? String(idx + 1).padStart(2, '0') : '—'}</span>
          <span class="s">·</span><span class="t">${esc(n.type)}</span>
          <span class="s">·</span><span>${esc(n.layer)}</span></span>
        <span class="h2-like"></span>
        <h2>${esc(deslug(n.name))}</h2>
        <p class="gist">${clip(n.body, 150)}</p>
        <span class="when">${when(n)}</span>
      </button>`)
      world.appendChild(el)
      el.onclick = () => readAt(id)
    })

    // A lane's height is whatever its tallest card turned out to be. Spacing
    // lanes by a fixed constant worked only while bodies were clipped; with
    // them open a long card runs straight through the lane below it.
    relayout()
    // Open at a readable size on the head of the trail rather than fitting the
    // whole research — a deep one fits only by shrinking the text to nothing.
    requestAnimationFrame(frameStart)
  }
  /** Re-lay the trail around whatever height the cards currently have.
   *  Everything that hangs off a card — its satellite, its lane tag, the
   *  shared ground below — is discarded and re-placed, because a card that
   *  grew leaves all of them pointing at where it used to be. */
  function relayout() {
    world.querySelectorAll('.sat,.lanetag,.bedlabel').forEach((e) => e.remove())
    const byLane = {}
    Object.entries(POS).forEach(([id, p]) => {
      const el = world.querySelector(`.step[data-id="${id}"]`); if (!el) return
      ;(byLane[p.lane] = byLane[p.lane] || []).push({ id, p, el })
    })
    const GAP = 64, SAT = 150       // between lanes, and room for a satellite above
    let top = SPINE_Y - 70
    Object.keys(byLane).map(Number).sort((a, b) => a - b).forEach((ln) => {
      const row = byLane[ln]
      if (row.some(({ id }) => satOf(id).length)) top += SAT
      row.forEach(({ p, el }) => { p.y = top; el.style.top = `${top}px` })
      top += Math.max(...row.map(({ el }) => el.offsetHeight)) + GAP
    })
    Object.entries(POS).forEach(([id, p]) => {
      const list = satOf(id); if (!list.length) return
      placeSat(list.slice(0, 1), p.x, p.y - 130, id)
    })
    labelLanes(new Set(steps.map((x) => x.id)))
    placeBedrock()
    drawTrack()
  }
  const satOf = (id) => (ADJ[id] || []).filter((l) =>
    !research.has(l.o) && !shared.has(l.o) && l.dir === 'in' && l.t === 'refers_to')

  function placeSat(list, sx, y, ofId) {
    list.forEach((l, j) => {
      const n = N[l.o]; if (!n) return
      const acc = LAYER_ACCENT[n.layer] || 'var(--dim)'
      const label = (REL[l.t] || [l.t, l.t])[l.dir === 'out' ? 0 : 1]
      const el = h(`<button type="button" class="sat" data-id="${n.id}" data-of="${ofId}"
          style="left:${sx + 8 + j * 310}px;top:${y}px;--acc:${acc}"
          aria-label="Read ${esc(deslug(n.name))} — ${esc(label)}">
        <span class="rel">${esc(label)}</span><span class="h3">${esc(deslug(n.name))}</span>
        <span class="ex">${clip(n.body, 110)}</span>
        <span class="tag">${esc(n.type)} · ${esc(n.tier)}</span></button>`)
      el.onclick = () => openNode(n.id)
      world.appendChild(el)
    })
  }
  function labelLanes(onChain) {
    const heads = {}
    Object.entries(POS).forEach(([id, p]) => { if (!heads[p.lane] || p.d < POS[heads[p.lane]].d) heads[p.lane] = id })
    // exactly one lane is the authored trail: the one carrying its first step
    const mainLane = steps.length && POS[steps[0].id] ? POS[steps[0].id].lane : 0
    Object.entries(heads).forEach(([ln, id]) => {
      const p = POS[id], main = +ln === mainLane
      // above its own first card, never beside it — to the left is where the
      // title page sits, and the tag used to collide with it
      // clear of the card it names: the tag is 18px tall, so sit it above that
      world.appendChild(h(`<div class="lanetag${main ? ' main' : ''}" style="left:${p.x}px;top:${p.y - 30}px">
        <span class="ln">${String.fromCharCode(65 + +ln)}</span> · ${main ? 'main trail' : 'side line'}</div>`))
    })
  }
  function placeBedrock() {
    if (!shared.size) return
    const xs = Object.values(POS).map((p) => p.x)
    let lowest = SPINE_Y
    Object.entries(POS).forEach(([id, p]) => {
      const el = world.querySelector(`.step[data-id="${id}"]`)
      lowest = Math.max(lowest, p.y + (el ? el.offsetHeight : 300))
    })
    const y = lowest + 90, total = shared.size
    const mid = (Math.min(...xs) + Math.max(...xs) + 540) / 2, left = mid - (total * 310) / 2
    world.appendChild(h(`<div class="bedlabel" style="left:${left}px;top:${y - 30}px">
      shared ground — more than one line rests on this</div>`))
    ;[...shared].forEach(([id, v], i) => {
      const n = N[id]; if (!n) return
      const acc = LAYER_ACCENT[n.layer] || 'var(--dim)'
      const el = h(`<button type="button" class="sat bed" data-id="${id}" data-steps="${v.steps.join(',')}"
          style="left:${left + i * 310}px;top:${y}px;--acc:${acc}"
          aria-label="Read ${esc(deslug(n.name))} — shared ground for ${v.n} steps">
        <span class="rel">shared ground · ${v.n}×</span><span class="h3">${esc(deslug(n.name))}</span>
        <span class="ex">${clip(n.body, 130)}</span>
        <span class="tag">${esc(n.type)} · ${esc(n.tier)}</span></button>`)
      el.onclick = () => openNode(id)
      world.appendChild(el)
    })
  }

  // ── the track follows real edges, so a fork looks like a fork ────────────
  function drawTrack() {
    if (!steps.length) { track.innerHTML = ''; return }
    const cs = getComputedStyle(document.documentElement)
    const acc = cs.getPropertyValue('--seal').trim()
    const quiet = cs.getPropertyValue('--line-2').trim()
    const faint = cs.getPropertyValue('--line').trim()
    const box = (id) => {
      const el = world.querySelector(`.step[data-id="${id}"]`); if (!el) return null
      const x = parseFloat(el.style.left), y = parseFloat(el.style.top)
      return { x, y, w: el.offsetWidth, h: el.offsetHeight, cy: y + 34 }
    }
    let d = ''
    // each lane's rail runs only as far as that line actually goes
    const rail = {}
    Object.entries(POS).forEach(([id, p]) => {
      const el = world.querySelector(`.step[data-id="${id}"]`); if (!el) return
      const L = rail[p.lane] = rail[p.lane] || { y: p.y + 34, min: Infinity, max: -Infinity }
      L.min = Math.min(L.min, p.x); L.max = Math.max(L.max, p.x + el.offsetWidth)
    })
    Object.values(rail).forEach((L) => {
      if (!isFinite(L.min)) return
      d += `<line x1="${L.min - 60}" y1="${L.y}" x2="${L.max + 60}" y2="${L.y}" stroke="${quiet}" stroke-width="1" opacity=".5"/>`
    })
    derivEdges().forEach((e) => {
      const a = box(e.from), b = box(e.to); if (!a || !b) return
      const x1 = a.x + a.w, y1 = a.cy, x2 = b.x - 10, y2 = b.cy
      const dx = Math.max(60, (x2 - x1) * 0.45)
      d += `<path d="M ${x1} ${y1} C ${x1 + dx} ${y1}, ${x2 - dx} ${y2}, ${x2} ${y2}" fill="none" stroke="${acc}" stroke-width="1.5" opacity=".6"/>`
      d += `<path d="M ${x2 - 6} ${y2 - 4} l 6 4 l -6 4" fill="none" stroke="${acc}" stroke-width="1.5" opacity=".6"/>`
      d += `<circle cx="${x1}" cy="${y1}" r="3.5" fill="${acc}" opacity=".75"/>`
    })
    world.querySelectorAll('.sat.bed').forEach((bd) => {
      const bx = parseFloat(bd.style.left) + 145, by = parseFloat(bd.style.top)
      ;(bd.dataset.steps || '').split(',').filter(Boolean).forEach((sid) => {
        const st = box(sid); if (!st) return
        const sx = st.x + st.w / 2, sy = st.y + st.h
        d += `<path d="M ${sx} ${sy} C ${sx} ${sy + 80}, ${bx} ${by - 80}, ${bx} ${by}" fill="none" stroke="${faint}" stroke-width="1" stroke-dasharray="3 5"/>`
      })
    })
    world.querySelectorAll('.sat:not(.bed)').forEach((s) => {
      const of = box(s.dataset.of); if (!of) return
      const sx = parseFloat(s.style.left) + 40, sy = parseFloat(s.style.top) + s.offsetHeight
      d += `<path d="M ${sx} ${sy} L ${sx} ${of.y}" stroke="${faint}" stroke-width="1" stroke-dasharray="3 4"/>`
    })
    track.innerHTML = d
  }

  // ── manuscript: the same trail as one continuous read ───────────────────
  /** The map is for shape; the words live in the reading view. */
  function readAt(id) {
    setView('manuscript')
    // Point at the step once now and once after the sheet has been given a box.
    // Not via requestAnimationFrame: it does not reliably fire when the view
    // that is about to paint was display:none a moment ago.
    const point = () => {
      const sec = sheet.querySelector(`section[data-id="${id}"]`)
      if (!sec) return
      sheet.querySelectorAll('section.here').forEach((e) => e.classList.remove('here'))
      sec.classList.add('here')
      const top = sec.getBoundingClientRect().top - sheet.getBoundingClientRect().top
      sheet.scrollTo({ top: sheet.scrollTop + top - 96, behavior: reduceMotion() ? 'auto' : 'smooth' })
    }
    point(); setTimeout(point, 60)
  }
  const reduceMotion = () => matchMedia('(prefers-reduced-motion: reduce)').matches

  function buildManuscript() {
    if (!steps.length) { sheet.innerHTML = ''; return }
    let html = `<div class="ms"><div class="kind">${chain.adhoc ? 'node' : 'reasoning chain'}</div>
      <h1>${esc(deslug(chain.name))}</h1>
      <p class="lede">${esc(chain.body || (chain.adhoc ? 'A single node, read in full.' : 'A worked trail of reasoning, read end to end.'))}</p>
      <div class="byline">${steps.length} ${steps.length === 1 ? 'step' : 'steps'} · ${when(steps[0])}${steps.length > 1 ? ' — ' + when(steps[steps.length - 1]) : ''}</div>`
    const onChain = new Set(steps.map((x) => x.id))
    const order = [...steps, ...[...research].filter((id) => !onChain.has(id)).map((id) => N[id])
      .filter(Boolean).sort((a, b) => (a.created_secs || 0) - (b.created_secs || 0))]
    order.forEach((n, i) => {
      const acc = LAYER_ACCENT[n.layer] || 'var(--dim)'
      const main = onChain.has(n.id)
      html += `<section data-id="${n.id}" style="--acc:${acc}">
        <div class="snum">${main ? `STEP ${String(i + 1).padStart(2, '0')}` : 'SIDE LINE'} · ${esc(n.type)} · ${esc(n.tier)}</div>
        <h2>${esc(deslug(n.name))}</h2>
        <div class="when">asserted <b>${when(n)}</b></div>
        <div class="prose">${paras(n.body || '—')}</div></section>`
      const links = (ADJ[n.id] || []).filter((l) => !onChain.has(l.o)).slice(0, 3)
      if (links.length) {
        html += `<aside class="marg" style="--acc:${acc}"><div class="mtitle">in the margin</div>`
        links.forEach((l) => {
          const t = N[l.o]; if (!t) return
          const label = (REL[l.t] || [l.t, l.t])[l.dir === 'out' ? 0 : 1]
          html += `<button type="button" class="mlink" data-to="${t.id}"
            aria-label="Read ${esc(deslug(t.name))} — ${esc(label)}">
            <span class="r">${l.dir === 'out' ? '↳ ' : '↰ '}${esc(label)}</span>
            <span class="n">${esc(deslug(t.name))}</span></button>`
        })
        html += '</aside>'
      }
    })
    sheet.innerHTML = html + '</div>'
    sheet.querySelectorAll('.mlink').forEach((b) => {
      b.onclick = () => { openNode(b.dataset.to); setView('manuscript') }
    })
  }

  // ── pan / zoom ──────────────────────────────────────────────────────────
  function applyTransform() {
    world.style.transform = `translate(${tx}px,${ty}px) scale(${scale})`
    const g = 28 * scale
    canvas.style.backgroundSize = `${g}px ${g}px`
    canvas.style.backgroundPosition = `${tx}px ${ty}px`
    const z = $('rzoom'); if (z) z.textContent = Math.round(scale * 100) + '%'
  }
  /** Frame the start of the trail at a size you can actually read. */
  function frameStart() {
    // the map is meant to be taken in, so start from a fit rather than a
    // fixed zoom — the stations are legible even well under 100%
    fit()
  }
  function fit() {
    const els = [...world.children].filter((e) => e.tagName !== 'svg' && e.offsetWidth)
    if (!els.length) return
    let mnx = 1e9, mny = 1e9, mxx = -1e9, mxy = -1e9
    els.forEach((e) => {
      const x = parseFloat(e.style.left), y = parseFloat(e.style.top)
      mnx = Math.min(mnx, x); mny = Math.min(mny, y)
      mxx = Math.max(mxx, x + e.offsetWidth); mxy = Math.max(mxy, y + e.offsetHeight)
    })
    const pad = 90, TOP = 76, BOTTOM = 60        // the reader's own bar, and the hint
    const availH = innerHeight - TOP - BOTTOM
    scale = clamp(Math.min(innerWidth / (mxx - mnx + pad * 2), availH / (mxy - mny + pad * 2)), 0.34, 1)
    tx = innerWidth / 2 - (mnx + (mxx - mnx) / 2) * scale
    ty = TOP + availH / 2 - (mny + (mxy - mny) / 2) * scale
    applyTransform()
  }
  let panning = false, sx = 0, sy = 0
  canvas.addEventListener('pointerdown', (e) => {
    if (e.target.closest('#rdock')) return
    panning = true; sx = e.clientX - tx; sy = e.clientY - ty; canvas.classList.add('grabbing')
  })
  addEventListener('pointermove', (e) => { if (!panning) return; tx = e.clientX - sx; ty = e.clientY - sy; applyTransform() })
  addEventListener('pointerup', () => { panning = false; canvas.classList.remove('grabbing') })
  canvas.addEventListener('wheel', (e) => {
    e.preventDefault()
    const ns = clamp(scale * Math.exp(-e.deltaY * 0.0012), 0.2, 1.8)
    const r = canvas.getBoundingClientRect(), mx = e.clientX - r.left, my = e.clientY - r.top
    tx = mx - (mx - tx) * (ns / scale); ty = my - (my - ty) * (ns / scale); scale = ns; applyTransform()
  }, { passive: false })
  const zoomBy = (f) => {
    const ns = clamp(scale * f, 0.2, 1.8), cx = innerWidth / 2, cy = innerHeight / 2
    tx = cx - (cx - tx) * (ns / scale); ty = cy - (cy - ty) * (ns / scale); scale = ns; applyTransform()
  }

  function setView(v) {
    const nm = $('rname'); if (nm && chain) nm.textContent = deslug(chain.name)
    const hint = $('rhint'); if (hint) hint.textContent =
      v === 'manuscript' ? 'reading the trail end to end' : 'click a step to read it'
    view = v
    sheet.classList.toggle('on', v === 'manuscript')
    canvas.style.display = v === 'manuscript' ? 'none' : ''
    $('rTrail').classList.toggle('on', v === 'trail')
    $('rMs').classList.toggle('on', v === 'manuscript')
  }
  const prevBtn = $('rPrev')
  if (prevBtn) prevBtn.onclick = goBack
  addEventListener('keydown', (e) => {
    if ($('reader').hidden) return
    const typing = /^(INPUT|TEXTAREA)$/.test(e.target.tagName)
    if (typing) return
    if (e.key === 'Backspace' || (e.altKey && e.key === 'ArrowLeft')) { e.preventDefault(); goBack() }
  })
  $('rTrail').onclick = () => setView('trail')
  $('rMs').onclick = () => setView('manuscript')

  /** Read a single node: its own chain if it has one, else the node plus what
   *  it was derived from. Used by the galaxy's readout and by every card the
   *  trail shows as a link. */
  /** A descriptor of what is on the desk, enough to come back to it. */
  const mark = () => (!chain ? null : chain.adhoc ? { node: steps[0] && steps[0].id } : { chain: chain.id })
  function restore(m) {
    if (!m) return false
    if (m.chain) {
      const c = data.chains.find((x) => x.id === m.chain)
      return c ? render(c, c.members) : false
    }
    return m.node && N[m.node] ? render({ name: N[m.node].name, adhoc: true }, [m.node]) : false
  }
  function openNode(id, remember = true) {
    if (!N[id]) return false
    const from = mark()
    const c = data.chains.find((x) => (x.members || []).includes(id))
    const ok = c ? render(c, c.members) : render({ name: N[id].name, adhoc: true }, [id])
    if (ok && remember && from && JSON.stringify(from) !== JSON.stringify(mark())) hist.push(from)
    syncBack()
    return ok
  }
  function goBack() {
    const m = hist.pop()
    if (!m) return false
    const ok = restore(m)
    syncBack()
    return ok
  }
  function syncBack() {
    const b = $('rPrev'); if (!b) return
    b.hidden = !hist.length
    const last = hist[hist.length - 1]
    const name = !last ? '' : last.chain
      ? (data.chains.find((x) => x.id === last.chain) || {}).name
      : (N[last.node] || {}).name
    b.title = name ? `Back to ${deslug(name)}` : ''
  }
  function render(c, members) {
    chain = c
    steps = members.map((id) => N[id]).filter(Boolean).sort((a, b) => (a.created_secs || 0) - (b.created_secs || 0))
    if (!steps.length) return false
    buildTrail(); buildManuscript(); setView(view)
    return true
  }

  return {
    /** Load a chain (by the picker's value) and render both views. */
    async show(chainId) {
      await loadBodies()
      const c = data.chains.find((x) => x.id === chainId) || data.chains.find((x) => x.name === chainId)
      hist = []; syncBack()
      return c ? render(c, c.members) : false
    },

    /** Open on a single node — the way in from the galaxy. If the node sits on
     *  a saved chain, read that trail; otherwise make an ad-hoc one from the
     *  node, which the layout then grows along its derivation links. */
    async showNode(id) {
      await loadBodies()
      hist = []; syncBack()
      return openNode(id, false)
    },

    /** Step back to whatever was being read before the last link. */
    back: goBack,
    canBack: () => hist.length > 0,

    /** Which chain is on the desk, if any (so the picker can follow along). */
    chainId: () => chain && chain.id,
    /** Re-tint the SVG track when the theme flips. */
    redraw: drawTrack,
    hasChain: () => !!chain,
  }
}
