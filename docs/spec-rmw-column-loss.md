# Спека: RMW-мутации теряют `visibility`/`layer`/`tags`/`properties` узла

**Дата:** 2026-07-08 · **Статус:** подтверждено на живой памяти, готово к реализации
**Серьёзность:** высокая — молчаливая потеря данных на каждом `revise`/`done`/`confirm`/…

---

## 1. Проблема

Все мутации, переписывающие СУЩЕСТВУЮЩИЙ узел через retract + re-assert (шаблон RMW),
пишут новую версию узла с неполным списком колонок. Не перечисленные в `:put` колонки
cozo молча заполняет **дефолтами схемы**, а часть колонок зануляется явно. В результате
узел теряет:

- `layer` → сбрасывается в `'warm'` (default) — **core/hot-узлы выпадают из инъекции при awake**;
- `visibility` → сбрасывается в `'local'` (default) — **shared-узлы выпадают из облака**
  (локально помечены local, в облаке остаётся устаревшая копия);
- `properties` → `null` (явно);
- `tags` → пересобираются заново из body (`build_body_tags`) — **ручные тэги теряются**.

### Реальный инцидент (2026-07-08, память kaeru-personal, инициатива 1c-agent)

`revise` core-хендоффа (rename + новый body) сбросил `layer: core → warm` и
`visibility: shared → local`. Итог: core-слой инициативы опустел (следующая сессия не
получила бы точку входа), узел выпал из облака. Тем же вечером обнаружилось, что
пер-тикетные эпизоды, ранее переименованные через `revise`, тоже молча стали
local/warm. Восстанавливали вручную (`layer` + `share` на ~11 узлов).

---

## 2. Первопричина

### Схема (kaeru-core/src/store.rs, `SCHEMA_STATEMENTS`)

```
:create node {
    id: String,
    validity: Validity ... =>
    type: String,
    tier: String,
    name: String,
    body: String?,
    tags: [String]?,
    initiatives: [String]?,
    properties: Json?,
    visibility: String default 'local',
    layer: String default 'warm',
}
```

### Сломанный шаблон (пример — `improve()`, kaeru-core/src/mutate/metabolism.rs:79)

Step 2 re-assert:

```
:put node {id, validity => type, tier, name, body, tags, initiatives, properties}
```

`visibility` и `layer` не перечислены → cozo подставляет дефолты. `initiatives` и
`properties` передаются как `null` явно. `tags` строятся заново из body.
Сохраняются только `type`/`tier` (читаются через `read_type_tier_now`).

Докстринг `improve` честно называет это «MVP scope» — осознанный долг:

> MVP scope: `tags`, `initiatives`, and `properties` are reset to null on the new
> revision. Callers who need to preserve those should write dedicated primitives…

### Почему инициатива при этом НЕ теряется

Членство узла в инициативе живёт в отдельной append-only таблице `node_initiative`
(prefix-scan), RMW-мутации её не трогают. Колонка `node.initiatives` фактически
рудиментарна — зануление её RMW-ом не наблюдаемо. Это единственная причина, почему
инцидент не был замечен раньше.

---

## 3. Аудит всех `:put node` (проверено 2026-07-08)

### ✅ Корректные — перечисляют `visibility, layer` (образец для фикса)

| Файл:строка | Что делает |
|---|---|
| `mutate/sharing.rs:83` | share — полный список колонок |
| `mutate/layer.rs:101` | set_layer — полный список |
| `mutate/ingest.rs:63` | import — полный список |

### 🟡 Создание нового узла — дефолты уместны, НЕ трогать (вне скоупа)

| Файл:строка | Verb |
|---|---|
| `mutate/episode.rs:59,100` | episode/jot (layer передаётся параметром; visibility=local при создании, share — отдельным шагом) |
| `mutate/cite.rs:58` | cite |
| `mutate/task.rs:70` | task (создание) |
| `mutate/hypothesis.rs:48` | claim (создание) |
| `mutate/synthesise.rs:61` | synthesise (новый узел) |
| `graph/audit.rs:52` | audit-записи |
| `store.rs:435,442,449`, `migrate.rs:446,587` | bootstrap/миграции |

### 🔴 RMW существующего узла — ЧИНИТЬ (теряют visibility+layer, часто tags/properties)

| Файл:строка | Verb (MCP) | Что теряет |
|---|---|---|
| `mutate/metabolism.rs:112` | **revise** (`improve`) | layer, visibility, properties, ручные tags |
| `mutate/task.rs:100,123` | **done / reopen** | layer, visibility, properties |
| `mutate/hypothesis.rs:87,151,186` | **test / confirm / refute** | layer, visibility, properties |
| `mutate/review.rs:81` | **flag/settle (review)** | layer, visibility, properties |
| `mutate/supersedes.rs:53,73` | **supersede** (обе стороны) | layer, visibility, properties |
| `mutate/consolidate.rs:108,128` | **consolidate** (retract источников) | layer, visibility |
| `lib.rs:700,708` | (проверить, что за путь) | layer, visibility |
| `mutate/metabolism.rs:59,91` | forget / improve-retract | ОК как есть — это retract-плейсхолдеры, читаться не будут |

Примечание: retract-шаги (валидность `false`, placeholder-строки) чинить не нужно —
они не резолвятся при чтении NOW. Чинить нужно только **re-assert живой версии**.

---

## 4. Дизайн фикса

### 4.1. Общий RMW-хелпер в kaeru-core

Единственная точка правды для «переписать узел, сохранив всё, кроме явно меняемого»:

```rust
/// Полный снапшот value-колонок узла at NOW.
struct NodeSnapshot {
    type_: String, tier: String, name: String, body: Option<String>,
    tags: Option<Vec<String>>, initiatives: Option<Vec<String>>,
    properties: Option<Json>, visibility: String, layer: String,
}

fn read_node_snapshot(store: &Store, id: &NodeId) -> Result<Option<NodeSnapshot>>;

/// Re-assert: снапшот + переопределения. `:put node {…}` со ВСЕМИ колонками схемы.
struct NodeOverrides {
    name: Option<String>, body: Option<String>,
    extra_tags: Vec<String>,          // добавить к существующим (merge, не replace)
    replace_tags: Option<Vec<String>>,// редкий случай полного replace
    properties: Option<Json>, layer: Option<Layer>, visibility: Option<String>,
    tier: Option<String>,             // для мутаций, двигающих tier (settle и т.п.)
}

fn reassert_node(store: &Store, id: &NodeId, snap: &NodeSnapshot, ov: NodeOverrides) -> Result<()>;
```

Ключевые правила:
- `:put` в `reassert_node` перечисляет **все** value-колонки схемы — ни одна не
  уходит в default молча. При добавлении колонки в схему компилятор/тест должен
  заставить обновить хелпер (см. 4.4 про тест-замок на схему).
- Тэги по умолчанию — **merge**: `snap.tags ∪ build_body_tags(kind/role, new_body)`,
  дедуп. Полный replace — только явным `replace_tags`.
- Снапшот читается ДО retract (иначе NOW уже не резолвится).

### 4.2. Перевести на хелпер все 🔴-места из таблицы

- `improve()` (revise): сохранять layer/visibility/properties; tags merge; убрать
  «MVP scope» из докстринга.
- `task.rs` done/reopen, `hypothesis.rs` test/confirm/refute, `review.rs`,
  `supersedes.rs`, `consolidate.rs`, `lib.rs:700,708` — механическая замена
  ручных `:put node` на `read_node_snapshot` + `reassert_node` с точечными overrides
  (например, done меняет только tags/properties статуса).

### 4.3. Shared-узлы: расхождение с облаком после revise

Сохранение `visibility='shared'` локально решает потерю флага, но после смены body
локальная и облачная копии расходятся. Решение (в порядке предпочтения):

1. **Авто-re-push**: если `snap.visibility == "shared"`, после успешного re-assert
   вызвать тот же путь, что `share` (пере-push в облако под тем же id). Уровень:
   kaeru-mcp обёртки (`revise`, `done`, `confirm`, …) или сервисный слой ядра —
   там, где сейчас живёт вызов share.
2. Если re-push невозможен (нет сети/облака) — не падать: оставить visibility=shared
   и вернуть в тексте ответа MCP-инструмента пометку «⚠ cloud copy is stale — run share».

### 4.4. Регресс-тесты (kaeru-core, cargo test)

1. `revise_preserves_layer_visibility`: узел layer=core, visibility=shared, ручной тэг
   `custom:x`, properties `{a:1}` → `improve(rename+body)` → все четыре сохранены,
   history показывает обе версии.
2. `done_preserves_layer_visibility`: аналогично для task done → reopen.
3. `confirm_preserves_layer_visibility`: hypothesis confirm.
4. `supersede_preserves_columns`: обе стороны ребра.
5. **Схема-замок**: тест, который сравнивает список колонок схемы `node` со списком
   колонок, известным `reassert_node` (например, константный массив имён) — новая
   колонка в схеме валит тест, пока хелпер не обновлён.
6. (kaeru-mcp, если есть интеграционные) revise shared-узла → ответ содержит
   подтверждение re-push либо пометку stale.

### 4.5. Вне скоупа

- Создающие мутации (episode/cite/task-create/claim/synthesise) — поведение дефолтов
  при создании корректно.
- Колонка `node.initiatives` — рудимент (истина в `node_initiative`); можно оставить
  как есть либо вынести отдельной задачей на выпил из схемы.
- `forget` — retract-семантика верна.

---

## 5. Критерии приёмки

- [ ] `revise` core+shared узла с ручными тэгами и properties не меняет ни layer, ни
      visibility, ни ручные тэги, ни properties (тест 1 зелёный).
- [ ] `done`/`reopen`/`confirm`/`refute`/`test`/`supersede`/`consolidate`/review-путь —
      то же (тесты 2–4).
- [ ] Все 🔴-места из таблицы §3 переведены на общий хелпер; ручных
      `:put node` с неполным списком колонок по живым узлам не осталось
      (`grep ':put node' kaeru-core/src` — только хелпер, создающие мутации и retract-плейсхолдеры).
- [ ] Схема-замок (тест 5) существует и зелёный.
- [ ] Shared-узел после revise: либо облачная копия обновлена, либо инструмент
      явно сообщил про stale (§4.3).
- [ ] `cargo build && cargo test` по workspace зелёные; `cargo fmt --check` чистый.

---

## 6. Как воспроизвести (до фикса)

```text
episode  name=x body=b layer=core visibility=shared (в team-инициативе)
revise   name=x rename=y
at       y     → layer: warm, visibility: local   # ОЖИДАЛОСЬ: core / shared
```

## 7. Workaround до фикса (уже применяется агентами)

После каждого `revise`/`rename` важного узла — заново `layer <name> core|hot` и
`share <name> <initiative>`. Зафиксировано в core-хендоффе инициативы 1c-agent.
