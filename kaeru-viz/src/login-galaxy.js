// The sign-in page's illustration: one project from the visualizer, seen up
// close. Nodes sit in shells around the project's hub by memory layer (core
// nearest, frozen farthest), coloured by layer as in the galaxy's "colour by
// layer" mode, but in the brand's own colours. One reasoning chain runs from
// the outer layers to the core, and a light travels along it — the thread the
// headline asks to keep. Drawn on a 2D canvas: the page does not load
// three.js, and a frame is one paint rather than thousands of DOM writes (an
// SVG draft made ~3,500 attribute writes a frame at 1920x1080). The data is
// invented and seeded; nothing about the vault shows before sign-in.

// [layer, share of nodes, shell radius, node radius] — the galaxy's LAYER table, scaled
const LAYERS = [
  ['core', 0.05, 0.14, 3.1], ['hot', 0.17, 0.34, 2.3], ['warm', 0.36, 0.58, 1.7],
  ['cold', 0.28, 0.8, 1.25], ['frozen', 0.14, 0.95, 0.95],
]
const COUNT = 150

function build() {
  let seed = 0x6b61
  const rnd = () => ((seed = (seed * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff)
  const gauss = () => (rnd() + rnd() + rnd() - 1.5) / 1.5
  const nodes = [{ x: 0, y: 0, z: 0, r: 4.2, layer: 'hub' }], links = []
  LAYERS.forEach(([layer, share, shell, r]) => {
    const n = Math.round(COUNT * share)
    for (let i = 0; i < n; i++) {
      // a point on the shell, roughened so the shells blur into a cloud
      const u = rnd() * 2 - 1, t = rnd() * Math.PI * 2, q = Math.sqrt(1 - u * u)
      const d = shell * (1 + gauss() * 0.22)
      nodes.push({ x: Math.cos(t) * q * d, y: u * d * 0.8, z: Math.sin(t) * q * d, r: r * (0.8 + rnd() * 0.4), layer })
    }
  })
  // a web: each node to the nearest of a few nodes nearer the hub than itself
  const radius = (n) => Math.hypot(n.x, n.y / 0.8, n.z)
  nodes.forEach((n, i) => {
    if (!i) return
    let best = 0, bd = Infinity
    for (let k = 0; k < 14; k++) {
      const j = Math.floor(rnd() * nodes.length), m = nodes[j]
      if (j === i || radius(m) >= radius(n)) continue
      const dist = (m.x - n.x) ** 2 + (m.y - n.y) ** 2 + (m.z - n.z) ** 2
      if (dist < bd) { bd = dist; best = j }
    }
    links.push([best, i])
    if (rnd() < 0.18) links.push([i, 1 + Math.floor(rnd() * (nodes.length - 1))])
  })
  // the chain: from a frozen node inward, each step to the nearest node one
  // layer closer, ending at the hub — observation to decision. It starts at
  // the frozen node farthest out to one side, so it sweeps across the cloud
  // instead of hopping within one patch of it.
  let at = 0, far = -Infinity
  nodes.forEach((n, j) => { if (n.layer === 'frozen' && n.x - Math.abs(n.z) * 0.5 > far) { far = n.x - Math.abs(n.z) * 0.5; at = j } })
  const chain = [at]
  for (const layer of ['cold', 'warm', 'hot', 'core']) {
    let best = -1, bd = Infinity
    nodes.forEach((m, j) => {
      if (m.layer !== layer) return
      const a = nodes[at], dist = (m.x - a.x) ** 2 + (m.y - a.y) ** 2 + (m.z - a.z) ** 2
      if (dist < bd) { bd = dist; best = j }
    })
    chain.push(best); at = best
  }
  chain.push(0)
  return { nodes, links, chain }
}

// Colours come from the theme tokens, read once and again whenever the theme
// flips, so the canvas follows the light and dark palettes as the CSS does.
const TOKENS = ['--fg', '--fg-2', '--dim', '--faint', '--gold', '--kind', '--seal']
const palette = {}
function readPalette() {
  const css = getComputedStyle(document.documentElement)
  TOKENS.forEach((t) => { palette[t] = css.getPropertyValue(t).trim() })
  palette.light = document.documentElement.dataset.theme === 'light'
}
function onThemeChange(fn) {
  new MutationObserver(() => { readPalette(); fn() })
    .observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] })
}

// a canvas backing store at the device pixel ratio, drawn in CSS pixels
function hiDpi(canvas, w, h) {
  const dpr = Math.min(devicePixelRatio || 1, 2)
  canvas.width = Math.round(w * dpr); canvas.height = Math.round(h * dpr)
  const ctx = canvas.getContext('2d')
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)
  return ctx
}

// The visualizer's background: a faint field of stars behind the whole page,
// so the figure sits in a sky rather than on a blank ground. The stars stand on
// a wide cylinder around the project's vertical axis and turn with it, at a
// quarter of its speed — the parallax a camera orbit gives a distant sky.
// `rotate(turn)` is driven by the project; without one the field stays still.
const MAX_STARS = 420 // density follows the area, capped: 4K would ask for ~2,200
const PARALLAX = 0.25
export function mountStarfield(parent) {
  readPalette()
  const canvas = document.createElement('canvas')
  canvas.className = 'account-starfield'
  canvas.setAttribute('aria-hidden', 'true')
  parent.prepend(canvas)
  let ctx, stars = [], axis = 0, angle = 0
  function place() {
    const w = innerWidth, h = innerHeight
    ctx = hiDpi(canvas, w, h)
    const frame = document.querySelector('.memory-galaxy')?.getBoundingClientRect()
    axis = frame && frame.width ? frame.left + frame.width / 2 : w / 2
    let seed = 0x5741
    const rnd = () => ((seed = (seed * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff)
    const reach = Math.max(axis, w - axis) * 1.15
    stars = Array.from({ length: Math.min(MAX_STARS, Math.round((w * h) / 3800)) }, () => {
      const big = rnd() < 0.06
      return {
        r: reach * Math.sqrt(0.08 + rnd() * 0.92), t: rnd() * Math.PI * 2, y: rnd() * h,
        size: big ? 0.9 + rnd() * 0.6 : 0.35 + rnd() * 0.45, glow: big ? 0.55 + rnd() * 0.3 : 0.18 + rnd() * 0.4,
      }
    })
    rotate(angle)
  }
  function rotate(turn) {
    angle = turn
    const a = turn * PARALLAX
    ctx.clearRect(0, 0, innerWidth, innerHeight)
    ctx.fillStyle = palette.light ? palette['--faint'] : palette['--dim']
    for (const s of stars) {
      const depth = Math.cos(s.t + a) // 1 = nearest
      ctx.globalAlpha = s.glow * (0.55 + 0.45 * depth)
      ctx.beginPath()
      ctx.arc(axis + s.r * Math.sin(s.t + a), s.y, s.size * (0.8 + 0.2 * depth), 0, Math.PI * 2)
      ctx.fill()
    }
    ctx.globalAlpha = 1
  }
  place()
  let pending = 0
  addEventListener('resize', () => { cancelAnimationFrame(pending); pending = requestAnimationFrame(place) })
  onThemeChange(() => rotate(angle))
  return { rotate, place }
}

export function mountGalaxy(container, { onTurn } = {}) {
  readPalette()
  const W = 520, H = 340, R = 200, F = 3.4, TILT = 0.38
  const { nodes, links, chain } = build()
  const canvas = document.createElement('canvas')
  canvas.className = 'memory-galaxy'
  canvas.setAttribute('aria-hidden', 'true')
  container.prepend(canvas)
  // drawn in a 520x340 design space, scaled to whatever width CSS gives it
  let ctx, scale = 1
  function size() {
    const w = canvas.getBoundingClientRect().width || W
    scale = w / W
    ctx = hiDpi(canvas, w, (w * H) / W)
  }
  const fill = { hub: '--seal', core: '--gold', hot: '--kind', warm: '--fg-2', cold: '--dim', frozen: '--faint' }
  const chainPairs = chain.slice(1).map((j, k) => [chain[k], j])
  const onChain = new Set(chain)
  const ct = Math.cos(TILT), st = Math.sin(TILT)
  const placed = nodes.map(() => ({ x: 0, y: 0, s: 1, z: 0 }))
  const order = nodes.map((n, i) => i)
  let pulse = -1 // position along the chain in segments; negative = resting
  function draw(angle) {
    const ca = Math.cos(angle), sa = Math.sin(angle)
    nodes.forEach((n, i) => {
      const x = n.x * ca + n.z * sa, z0 = -n.x * sa + n.z * ca
      const y = n.y * ct - z0 * st, z = n.y * st + z0 * ct
      const s = F / (F + z), p = placed[i]
      p.x = W / 2 + x * R * s; p.y = H / 2 + y * R * s; p.s = s; p.z = z
    })
    ctx.save()
    ctx.clearRect(0, 0, W * scale, H * scale)
    ctx.scale(scale, scale)
    // the web, as one path
    ctx.strokeStyle = palette['--dim']; ctx.globalAlpha = 0.22; ctx.lineWidth = 0.5
    ctx.beginPath()
    for (const [i, j] of links) { ctx.moveTo(placed[i].x, placed[i].y); ctx.lineTo(placed[j].x, placed[j].y) }
    ctx.stroke()
    // nodes, back to front
    order.sort((a, b) => placed[b].z - placed[a].z)
    for (const i of order) {
      const p = placed[i], n = nodes[i]
      ctx.globalAlpha = Math.max(0.55, Math.min(1, 0.8 - p.z * 0.3))
      ctx.fillStyle = palette[fill[n.layer]]
      ctx.beginPath(); ctx.arc(p.x, p.y, n.r * p.s, 0, Math.PI * 2); ctx.fill()
      if (onChain.has(i)) { ctx.globalAlpha = 1; ctx.strokeStyle = palette['--gold']; ctx.lineWidth = 1.2; ctx.stroke() }
    }
    // the chain, on top
    ctx.globalAlpha = 1; ctx.strokeStyle = palette['--gold']; ctx.lineWidth = 1.3
    ctx.beginPath()
    chain.forEach((i, k) => (k ? ctx.lineTo(placed[i].x, placed[i].y) : ctx.moveTo(placed[i].x, placed[i].y)))
    ctx.stroke()
    if (pulse >= 0) {
      const k = Math.min(Math.floor(pulse), chainPairs.length - 1), t = pulse - k
      const a = placed[chainPairs[k][0]], b = placed[chainPairs[k][1]]
      ctx.fillStyle = palette['--fg']; ctx.shadowColor = palette['--gold']; ctx.shadowBlur = 8
      ctx.beginPath(); ctx.arc(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, 2.4, 0, Math.PI * 2); ctx.fill()
    }
    ctx.restore()
  }

  // a turn every 80 seconds, drawn about 30 times a second; the light crosses
  // the chain in about 4 s, then rests 3 s
  let angle = 0.9, clock = 0, last = 0, frame = 0, running = false
  const STEP = 650, REST = 3000, RUN = STEP * chainPairs.length
  const tick = (t) => {
    if (!running) return
    if (!last) last = t
    if (t - last > 33) {
      const dt = t - last; last = t
      angle += (dt / 80000) * Math.PI * 2
      clock = (clock + dt) % (RUN + REST)
      pulse = clock < RUN ? clock / STEP : -1
      draw(angle); onTurn?.(angle)
    }
    frame = requestAnimationFrame(tick)
  }
  size(); draw(angle); onTurn?.(angle)
  new ResizeObserver(() => { size(); draw(angle) }).observe(canvas)
  onThemeChange(() => draw(angle))
  return {
    play() { if (running) return; running = true; last = 0; frame = requestAnimationFrame(tick) },
    pause() { running = false; cancelAnimationFrame(frame); pulse = -1; draw(angle) },
    get playing() { return running },
  }
}
