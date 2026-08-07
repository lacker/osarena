---
name: query-magic-references
description: "Query this repository's optional generated Scryfall SQLite index for card characteristics, faces, Oracle text, legalities, keywords, colors, related parts, and rulings. Use for fast exact-card lookups, full-text searches, joins, aggregate analysis, batch audits, or comparisons while developing Penta. Do not use it as the authority for Comprehensive Rules, live prices, or printing history not represented by the indexed sources; refresh the cache or use an authoritative online source when necessary."
---

# Query Magic References

Use the clone-wide, schema-versioned Scryfall index as a read-only development
reference. It is shared by all linked worktrees. Keep results narrow enough to
inspect; never dump the database into model context.

## Start safely

Read [references/schema.md](references/schema.md) before composing a nontrivial
query. It documents the schema, indexes, FTS behavior, examples, and
interpretation limits.

Resolve the database path instead of constructing or hardcoding it:

```sh
REFERENCE_SCRIPT=.agents/skills/refresh-magic-references/scripts/reference_material.py
SCRYFALL_DB="$(python3 "$REFERENCE_SCRIPT" path scryfall-index)"
```

Check the database against its cached inputs without networking or refreshing:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py status scryfall-index
```

Path resolution and this local status check are low-friction reads. Use the
database if it is current relative to its inputs; do not refresh merely because
the worktree is new. For a genuinely freshness-sensitive task, separately
compare the inputs with Scryfall:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py status oracle-cards rulings
```

If the database is missing, corrupt, or unavailable for the required schema,
use `$refresh-magic-references` to repair it with explicit human approval. Only
refresh intact source data when the current task truly needs newer material.
If shared mutation is unavailable or unwarranted, use retained source data or
an authoritative online source instead.

## Choose the narrowest query

- Use `card_names.normalized_name` for an exact primary or face name.
- Use `cards` for characteristics and compact JSON fields.
- Use `card_faces` for face-specific cost, type, text, or stats.
- Join `cards` to `rulings` through `oracle_id`.
- Use `card_keywords`, `card_colors`, and `card_parts` for relationships.
- Use `card_search` or `ruling_search` with `MATCH` for words and concepts.
- Use bounded `LIKE` queries when FTS5 is unavailable or punctuation matters.

Open the database read-only:

```sh
sqlite3 -readonly -header -column "$SCRYFALL_DB"
```

Use `-json` for structured consumption. Project only needed columns, run
`COUNT(*)` before broad extraction, add `LIMIT` while exploring, and use
`EXPLAIN QUERY PLAN` for slow or repeated batch queries. Never modify the
generated database directly.

## Interpret results carefully

`oracle-cards` has one representative Scryfall object per Oracle ID, not every
printing or language. Representative set fields do not establish Old School
legality, and Scryfall format legality is not a substitute for Eternal
Central's rules. Oracle text may intentionally differ from Penta's adapted
historical behavior. Use the compressed bulk files or Scryfall online for
unmodeled printing fields, resolving cached paths through the refresh script,
and use an appropriately live source for prices.
