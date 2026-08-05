// The board — the galaxy's third view.
//
// The galaxy answers *where* a thought sits, the reader *what it says*, and the
// board *what is still owed*. It renders the task board `docs/board.md`
// describes: one board per initiative, a card's column is the `status:<key>`
// tag on the task itself, columns come from the initiative's registry.
//
// Read-only, deliberately. `/graph.json` carries the `status:*` tags but not
// `properties`, so the column registry can't reach the browser yet and the
// built-in vocabulary is what we render. Moving a card is `set_status`, a
// write the viz endpoint doesn't have — so the board shows the command instead
// of performing it.
//
// Within a column the order is *rot*, not recency: overdue first, then age,
// then isolation. A task that has been open two months with nothing linked to
// it is not "todo", it is the thing this room exists to surface.

// The built-in vocabulary `write_task` stamps, in registry order (board.md).
// Once the export carries `properties`, this is what the Board node replaces.
import { loadFullBodies } from './bodies.js'

const COLUMNS = [
  { key: 'open', label: 'Open' },
  { key: 'in-progress', label: 'In Progress' },
  { key: 'done', label: 'Done' },
]
const DAY = 86400

const $ = (id) => document.getElementById(id)
const esc = (s) => (s || '').replace(/[&<>"']/g, (m) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[m]))
const deslug = (s) => (s || '').replace(/-20\d\d-\d\d-\d\d$/, '').replace(/-[0-9a-f]{6}$/, '').replace(/-/g, ' ')
const tagged = (n, p) => (n.tags || []).find((t) => t.startsWith(p))
const days = (n) => (n === 1 ? '1 day' : `${n} days`)

export function createBoard(data, { onOpenNode }) {
  const N = {}, LINKS = {}
  data.nodes.forEach((n) => { N[n.id] = n })
  for (const e of data.edges) {
    ;(LINKS[e.src] = LINKS[e.src] || []).push(e.dst)
    ;(LINKS[e.dst] = LINKS[e.dst] || []).push(e.src)
  }

  const today = new Date().toISOString().slice(0, 10)
  const now = Date.now() / 1000
  // Every task, decorated with what the board needs to sort and explain it.
  const CARDS = data.nodes.filter((n) => n.type === 'task').map((n) => {
    const age = n.created_secs ? Math.floor((now - n.created_secs) / DAY) : null
    const due = (tagged(n, 'due:') || '').slice(4) || null
    const overdue = !!due && due < today
    const near = (LINKS[n.id] || []).filter((id) => N[id])
    // provenance: what the task fell out of — an episode when there is one
    const src = near.find((id) => N[id].type === 'episode') || near[0] || null
    // A status the registry doesn't know falls into the first column rather
    // than disappearing — same rule the `board` verb applies (board.md).
    const raw = (tagged(n, 'status:') || '').slice(7)
    const key = COLUMNS.some((c) => c.key === raw) ? raw : COLUMNS[0].key
    const text = (n.body || '').trim() || deslug(n.name)
    return {
      id: n.id, name: n.name, key, age, due, overdue, src,
      island: near.length === 0,
      init: (n.initiatives || [])[0] || null,
      text,
      hay: (text + ' ' + deslug(n.name)).toLowerCase(),
      rot: (age || 0) + (near.length === 0 ? 45 : 0) + (overdue ? 400 : 0),
    }
  })

  // initiatives that actually own a task, busiest first
  const OWNERS = [...CARDS.reduce((m, c) => m.set(c.init, (m.get(c.init) || 0) + (c.key === 'done' ? 0 : 1)), new Map())]
    .filter(([k]) => k).sort((a, b) => b[1] - a[1])

  const ALL = '*'          // every initiative at once
  let init = ALL
  let picked = null
  let copyTimer = null
  // Filters stack: scope AND text AND every active sieve.
  let query = ''
  const sieveOn = new Set()

  // `status:` and `due:` are the only tags a person sets, and both are already
  // on the board as columns and badges. `topic:` is derived word-frequency —
  // its busiest values here are "блок", "новый", "документ" — so a topic picker
  // would mostly offer noise that free text finds better anyway.
  const SIEVES = [
    { key: 'overdue', label: 'past due', f: (c) => c.overdue },
    { key: 'island', label: 'no provenance', f: (c) => c.island },
    { key: 'old', label: 'over a month', f: (c) => c.age > 30 },
    { key: 'owed', label: 'not done', f: (c) => c.key !== 'done' },
  ]
  const inScope = () => (init === ALL ? CARDS : CARDS.filter((c) => c.init === init))
  const passes = (c) =>
    (!query || c.hay.includes(query)) &&
    [...sieveOn].every((k) => SIEVES.find((s) => s.key === k).f(c))
  const filtering = () => !!(query || sieveOn.size)

  // ── the filter bar ────────────────────────────────────────────────────────
  // Built once and updated in place: rebuilding the controls on every
  // keystroke would throw away focus mid-typing.
  function buildSieves() {
    $('bsieves').innerHTML = SIEVES.map((s) =>
      `<button type="button" class="sieve" data-sieve="${s.key}" aria-pressed="false">
         ${s.label} <span class="c"></span></button>`).join('')
  }
  function drawFilterBar(scoped) {
    SIEVES.forEach((s) => {
      const b = $('bsieves').querySelector(`[data-sieve="${s.key}"]`)
      if (!b) return
      const on = sieveOn.has(s.key)
      b.setAttribute('aria-pressed', String(on))
      b.classList.toggle('on', on)
      b.querySelector('.c').textContent = scoped.filter(s.f).length
    })
    $('bClear').hidden = !filtering()
  }
  function describeFilters() {
    const bits = []
    if (query) bits.push(`“${query}”`)
    sieveOn.forEach((k) => bits.push(SIEVES.find((s) => s.key === k).label))
    return bits.join(' + ')
  }
  function clearFilters() {
    query = ''; sieveOn.clear()
    $('bFind').value = ''
    drawColumns(); drawDetail()
  }

  // ── the columns ───────────────────────────────────────────────────────────
  const first = (t) => {
    const s = t.replace(/\s+/g, ' ').trim()
    const cut = s.search(/[.!?](\s|$)/)
    return cut > 24 && cut < 150 ? s.slice(0, cut + 1) : s
  }
  function card(c) {
    const flags = [
      c.overdue ? `<span class="due">due ${esc(c.due)}</span>` : c.due ? `<span class="soft">due ${esc(c.due)}</span>` : '',
      c.island ? `<span class="isle">no provenance</span>` : '',
    ].filter(Boolean).join('<span class="sep">·</span>')
    return `<button type="button" class="card${c.overdue ? ' overdue' : ''}${picked === c.id ? ' picked' : ''}"
        data-card="${esc(c.id)}" aria-pressed="${picked === c.id}">
      <span class="what">${esc(first(c.text))}</span>
      <span class="meta">${init === ALL && c.init ? `<span class="owner">${esc(c.init)}</span>` : ''}<span class="age">${c.age == null ? '—' : days(c.age)}</span>${flags ? `<span class="sep">·</span>${flags}` : ''}</span>
    </button>`
  }
  // A column nothing has ever reached is not "empty", it is unused — and since
  // this room cannot move a card, saying so is more use than saying nothing.
  const everUsed = new Set(CARDS.map((c) => c.key))
  function emptyCol(col) {
    if (everUsed.has(col.key) || col.key === COLUMNS[0].key) {
      return `<p class="none">${col.label} is empty.</p>`
    }
    return `<p class="none">Nothing has ever been in ${col.label}.
      A card gets here with <code>set_status &lt;task&gt; ${esc(col.key)}</code> —
      the board reads, the vault writes.</p>`
  }

  function drawColumns() {
    const scoped = inScope()
    const mine = scoped.filter(passes)
    $('bcols').innerHTML = COLUMNS.map((col) => {
      const rows = mine.filter((c) => c.key === col.key)
        // rot orders what is still owed; a finished card is just newest-first
        .sort((a, b) => (col.key === 'done' ? (b.age == null) - (a.age == null) || a.age - b.age : b.rot - a.rot))
      return `<section class="col" aria-labelledby="col-${col.key}">
        <header class="colhead"><h2 id="col-${col.key}">${col.label}</h2><span class="n">${rows.length}</span></header>
        <div class="stack">${rows.length ? rows.map(card).join('') : emptyCol(col)}</div>
      </section>`
    }).join('')
    if (!mine.length && filtering()) {
      $('bcols').innerHTML = `<p class="nohits">No cards match ${esc(describeFilters())}.
        <button type="button" class="btn" id="bClear2">Clear filters</button></p>`
    }
    const owed = mine.filter((c) => c.key !== 'done').length
    const over = mine.filter((c) => c.overdue && c.key !== 'done').length
    const shown = filtering() ? `${mine.length} of ${scoped.length} cards · ` : ''
    $('bcount').textContent = mine.length
      ? `${shown}${owed} owed${over ? `, ${over} past due` : ''} · sorted by rot — deadline, then age, then isolation`
      : ''
    drawFilterBar(scoped)
    $('bname').textContent = init === ALL ? 'all initiatives' : init
    $('bsay').textContent = `${init === ALL ? 'all initiatives' : init}: ${owed} open, ${mine.length} cards total`
  }

  // ── the card drawer ───────────────────────────────────────────────────────
  // Over the board, never squeezing it — the same move Jira makes. One drawer
  // rather than controls on every card: per-card buttons would have put 200+
  // stops between the lanes and anything below them.
  let returnFocusTo = null
  function drawDetail() {
    const c = CARDS.find((x) => x.id === picked)
    const strip = $('bdetail')
    strip.hidden = !c
    if (!c) return
    const col = COLUMNS.find((x) => x.key === c.key)
    const from = c.island
      ? `<span class="isle">nothing links to this card yet</span>`
      : `<button type="button" class="nodelink" data-open="${esc(c.src)}">${esc(deslug(N[c.src] ? N[c.src].name : ''))}</button>`
    strip.tabIndex = -1
    strip.innerHTML = `
      <header id="bdhead">
        <span class="k">card</span><span class="colname">${col ? col.label : c.key}</span>
        <button type="button" class="btn x" data-close="1" aria-label="Close card">✕</button>
      </header>
      <div id="bdbody">
        <p class="dbody">${esc(c.text)}</p>
        <dl>
          <dt>initiative</dt><dd>${esc(c.init || '—')}</dd>
          <dt>open for</dt><dd>${c.age == null ? '—' : days(c.age)}</dd>
          ${c.due ? `<dt>due</dt><dd>${c.overdue
            ? `<span class="due">${esc(c.due)} · past due</span>`
            : `<span class="soft">${esc(c.due)}</span>`}</dd>` : ''}
          <dt>came out of</dt><dd>${from}</dd>
        </dl>
      </div>
      <div id="bdacts">
        <button type="button" class="btn" data-open="${esc(c.id)}">Open in reader</button>
        <button type="button" class="btn cp" data-copy="set_status ${esc(c.name)} in-progress">Copy “set_status”</button>
        <button type="button" class="btn cp" data-copy="done ${esc(c.name)}">Copy “done”</button>
      </div>`
  }
  function closeDetail() {
    picked = null
    drawColumns(); drawDetail()
    if (returnFocusTo && document.contains(returnFocusTo)) returnFocusTo.focus()
    returnFocusTo = null
  }

  function pick(id, el) {
    if (picked === id) { closeDetail(); return }
    picked = id; returnFocusTo = el || null
    drawColumns(); drawDetail()
    $('bdetail').focus()
  }

  // ── wiring ────────────────────────────────────────────────────────────────
  $('bcols').addEventListener('click', (e) => {
    const b = e.target.closest('[data-card]'); if (b) pick(b.dataset.card, b)
  })
  $('bdetail').addEventListener('click', (e) => {
    const b = e.target.closest('button'); if (!b) return
    if (b.dataset.close) { closeDetail(); return }
    if (b.dataset.open) { onOpenNode(b.dataset.open); return }
    if (!b.dataset.copy) return
    navigator.clipboard?.writeText(b.dataset.copy)
    // Keep the label out of the state: two quick clicks used to leave "Copied"
    // as the button's own text forever.
    clearTimeout(copyTimer)
    $('bdetail').querySelectorAll('.cp').forEach((x) => x.classList.remove('ok'))
    b.classList.add('ok')
    $('bsay').textContent = `Copied: ${b.dataset.copy}`
    copyTimer = setTimeout(() => b.classList.remove('ok'), 1400)
  })
  // Escape closes the drawer first; the room's own handler only gets it once
  // there is no drawer left to close.
  addEventListener('keydown', (e) => {
    if (e.key !== 'Escape' || $('board').hidden || !picked) return
    e.stopImmediatePropagation(); e.preventDefault()
    closeDetail()
  })

  $('bsieves').addEventListener('click', (e) => {
    const b = e.target.closest('[data-sieve]'); if (!b) return
    const k = b.dataset.sieve
    if (sieveOn.has(k)) sieveOn.delete(k); else sieveOn.add(k)
    drawColumns(); drawDetail()
  })
  $('bFind').addEventListener('input', (e) => {
    query = e.target.value.trim().toLowerCase()
    drawColumns(); drawDetail()
  })
  $('bFind').addEventListener('keydown', (e) => {
    if (e.key !== 'Escape' || !e.target.value) return
    e.stopPropagation(); e.target.value = ''; query = ''; drawColumns(); drawDetail()
  })
  $('bClear').addEventListener('click', clearFilters)
  $('bcols').addEventListener('click', (e) => {
    if (e.target.id === 'bClear2') clearFilters()
  })

  const pickInit = $('bInit')
  buildSieves()
  const owedAll = CARDS.filter((c) => c.key !== 'done').length
  pickInit.innerHTML = `<option value="${ALL}">all initiatives (${owedAll} open)</option>` +
    OWNERS.map(([k, n]) => `<option value="${esc(k)}">${esc(k)} (${n} open)</option>`).join('')
  pickInit.addEventListener('change', () => {
    init = pickInit.value; picked = null
    drawColumns(); drawDetail()
  })

  return {
    /** Open the board, optionally on a named initiative. */
    show(name) {
      if (name === ALL || (name && OWNERS.some(([k]) => k === name))) init = name
      if (!init) return false
      pickInit.value = init
      if (pickInit._sync) pickInit._sync()
      picked = null
      drawColumns(); drawDetail()
      // The cards read fine on excerpts; the detail strip is where the whole
      // text belongs, so fetch it in the background and repaint if it lands.
      loadFullBodies().then((full) => {
        let grew = false
        for (const c of CARDS) if (full[c.id] && full[c.id] !== c.text) { c.text = full[c.id]; grew = true }
        if (grew) { drawColumns(); drawDetail() }
      })
      return true
    },
    initiative: () => init,
  }
}
