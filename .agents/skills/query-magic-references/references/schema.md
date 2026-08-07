# Scryfall SQLite schema and query patterns

The generated database is a schema-versioned artifact in the Git-common cache
shared by every worktree in a clone. Resolve its absolute path rather than
hardcoding a worktree-relative location:

```sh
REFERENCE_SCRIPT=.agents/skills/refresh-magic-references/scripts/reference_material.py
SCRYFALL_DB="$(python3 "$REFERENCE_SCRIPT" path scryfall-index)"
sqlite3 -readonly -header -column "$SCRYFALL_DB"
```

Schema version 1 derives from the cached `oracle-cards` and `rulings` Scryfall
JSONL sources; those compressed files remain canonical. Resolve either source
with the refresh script's `path` command when an unmodeled field is needed.
Other schema versions may coexist for worktrees whose skill implementations
differ.

## Tables

`metadata(key, value)` records the schema and build time, SQLite and FTS5
versions, input paths at build time, input checksums/source timestamps, source
counts, indexed counts, removed duplicate rulings, and orphan rulings. A
migrated database can retain its old build-time path strings; locate current
inputs through the refresh script rather than treating those strings as paths.

`cards` has one row per Oracle ID:

- IDs: `card_id`, `oracle_id`, `scryfall_id`
- Lookup: `name`, `normalized_name`, `layout`
- Rules: `mana_cost`, `mana_value`, `type_line`, `oracle_text`, `power`,
  `toughness`, `loyalty`, `defense`
- Compact JSON: `colors_json`, `color_identity_json`, `produced_mana_json`,
  `keywords_json`, `legalities_json`, `games_json`
- Flags: `reserved`, `digital`, `game_changer`
- Representative printing: `released_at`, `representative_set_code`,
  `representative_set_name`, `representative_collector_number`,
  `representative_rarity`
- Links: `scryfall_uri`, `rulings_uri`, `image_uri`

`normalized_name` is Unicode NFKC plus case folding. It is indexed but not
unique; Scryfall contains cards and tokens with colliding names.

`card_faces` contains face-specific fields keyed by `(card_id, face_index)`.
Parent fields can be absent for multifaced cards, so inspect faces when needed.

`card_names` covers both primary and face names. `name_index = -1` and
`name_kind = 'card'` identify the parent name; nonnegative indexes are faces.

`rulings` contains unique `(oracle_id, published_at, source, comment)` content.
The builder removes exact duplicate source rows using the internal
`fingerprint` column. Join it to `cards` through `oracle_id`.

`card_keywords(card_id, keyword)`, `card_colors(card_id, kind, color)`, and
`card_parts` provide indexed relationships. Color `kind` is `color`,
`identity`, or `produced`. `card_parts` models Scryfall `all_parts` links.

`card_search` and `ruling_search` are optional contentless FTS5 indexes. Their
rowids map to `cards.card_id` and `rulings.ruling_id`; select display text from
the base tables, not the FTS columns. Check the `fts5` key in `metadata` before
relying on them.

## Useful queries

### Build provenance and counts

```sql
SELECT key, value
FROM metadata
ORDER BY key;
```

### Exact card or face name with rulings

Use the NFKC/case-folded name literal. Most English card names simply become
lowercase.

```sql
SELECT
  c.name,
  c.mana_cost,
  c.type_line,
  c.oracle_text,
  r.published_at,
  r.source,
  r.comment
FROM card_names AS n
JOIN cards AS c USING (card_id)
LEFT JOIN rulings AS r USING (oracle_id)
WHERE n.normalized_name = 'chaos orb'
ORDER BY c.name, r.published_at, r.ruling_id;
```

Because a name can match more than one object, retain identifying fields such
as `oracle_id`, `layout`, or `type_line` until ambiguity is resolved.

### Multifaced card

```sql
SELECT
  c.name AS card_name,
  f.face_index,
  f.name AS face_name,
  f.mana_cost,
  f.type_line,
  f.oracle_text
FROM card_names AS n
JOIN cards AS c USING (card_id)
JOIN card_faces AS f USING (card_id)
WHERE n.normalized_name = 'fire // ice'
ORDER BY c.card_id, f.face_index;
```

### Full-text card search

```sql
SELECT
  c.name,
  c.type_line,
  c.oracle_text,
  bm25(card_search) AS rank
FROM card_search
JOIN cards AS c ON c.card_id = card_search.rowid
WHERE card_search MATCH 'draw AND discard'
ORDER BY rank
LIMIT 20;
```

FTS aggregates parent and face names, types, Oracle text, and keywords into one
search row per card. Use quoted FTS phrases for adjacent words. Use `LIKE` on a
base column for punctuation-heavy strings such as mana symbols.

### Full-text ruling search

```sql
SELECT c.name, r.published_at, r.comment, bm25(ruling_search) AS rank
FROM ruling_search
JOIN rulings AS r ON r.ruling_id = ruling_search.rowid
LEFT JOIN cards AS c USING (oracle_id)
WHERE ruling_search MATCH 'copy AND target'
ORDER BY rank
LIMIT 20;
```

If FTS5 is unavailable, use a bounded fallback:

```sql
WITH searchable AS (
  SELECT
    c.card_id,
    c.name,
    c.type_line,
    lower(
      coalesce(c.oracle_text, '') || char(10) ||
      coalesce(group_concat(f.oracle_text, char(10)), '')
    ) AS search_text
  FROM cards AS c
  LEFT JOIN card_faces AS f USING (card_id)
  GROUP BY c.card_id
)
SELECT name, type_line
FROM searchable
WHERE search_text LIKE '%draw%' AND search_text LIKE '%discard%'
ORDER BY name
LIMIT 20;
```

### Characteristics, legality, keyword, and color

Legalities stay as compact JSON because normalizing every format would add
hundreds of thousands of rows. JSON scans over the card table are inexpensive.

```sql
SELECT DISTINCT c.name, c.mana_value, c.type_line
FROM cards AS c
JOIN card_keywords AS k USING (card_id)
JOIN card_colors AS color USING (card_id)
WHERE json_extract(c.legalities_json, '$.vintage') = 'legal'
  AND k.keyword = 'Flying'
  AND color.kind = 'identity'
  AND color.color = 'U'
ORDER BY c.name
LIMIT 50;
```

Scryfall format legality is not automatically equivalent to Eternal Central
93/94 legality. Use the cached EC rules for that determination.

### Related card parts

```sql
SELECT c.name AS parent, p.component, p.name AS part, p.type_line
FROM cards AS c
JOIN card_parts AS p USING (card_id)
WHERE c.normalized_name = 'hanweir battlements';
```

### Batch exact-name lookup

```sql
WITH requested(normalized_name) AS (
  VALUES ('black lotus'), ('chaos orb'), ('time vault')
)
SELECT requested.normalized_name, c.oracle_id, c.name, c.type_line, c.oracle_text
FROM requested
LEFT JOIN card_names AS n USING (normalized_name)
LEFT JOIN cards AS c USING (card_id)
ORDER BY requested.normalized_name, c.card_id;
```

### Inspect query planning

```sql
EXPLAIN QUERY PLAN
SELECT c.name
FROM card_names AS n
JOIN cards AS c USING (card_id)
WHERE n.normalized_name = 'lightning bolt';
```

Use indexed equality on `normalized_name`, `oracle_id`, keyword, and color
relationships. Use FTS for prose. Project only needed columns and add `LIMIT`
while exploring.

## Interpretation limits

- Oracle bulk data supplies one representative object per Oracle ID, not full
  printing or language history.
- Representative set fields do not prove format legality.
- Current Oracle text and rulings may intentionally differ from Penta's
  historical adaptations.
- Prices are intentionally not indexed as a freshness guarantee.
- For unmodeled fields, stream the retained JSONL or use Scryfall online.
