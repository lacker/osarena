---
name: refresh-magic-references
description: "Locate, inspect, migrate, refresh, and index this repository's optional shared Magic: The Gathering reference cache, including the Wizards Comprehensive Rules, Eternal Central 93/94 rules, Scryfall bulk card and ruling data, and schema-versioned SQLite indexes. Use when Codex needs reproducible local rules or card data, needs to diagnose cache freshness or locking, needs to build or repair the Scryfall index, or is asked to populate or update the clone-wide cache. Do not require it for questions that authoritative online sources can answer reliably."
---

# Refresh Magic References

Maintain the optional development cache shared by all worktrees in a clone.
The script resolves it beneath Git's common directory by default; never infer
the path from a worktree's `.git` entry or copy the cache into each worktree.
Keep downloaded payloads and generated indexes out of commits and runtime
artifacts. Relative environment or Git-config overrides are also resolved from
the Git common directory so they remain shared; only an explicit
`--reference-dir` is worktree-relative.

Run from the repository root and let the script resolve paths:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py path
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py path scryfall-index
```

`path`, `status scryfall-index`, and `lock-status` are read-only and should not
require approval. Prefer them before considering a shared mutation.

## Check freshness

Check only the derived database against its cached inputs, without networking:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py status scryfall-index
```

Check authoritative source metadata only when the task's freshness needs make
it relevant:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py status
```

Interpret `current` and `fresh` as usable under the selected freshness policy.
Treat `unknown` as uncertainty, not proof that an intact cache is wrong.
Missing, corrupt, or schema-incompatible data can justify repair. Staleness
justifies refresh only when it matters to the current task; otherwise use the
cached version with its limitation or consult an authoritative online source.

Default status checks source freshness and the current schema's
`scryfall-index`. The database's status is relative to cached `oracle-cards`
and `rulings` checksums; source freshness is reported separately.

The Scryfall cache uses a seven-day gameplay-data freshness window by default.
That window does not make cached prices current; use a one-day window or an
appropriate live source for price-sensitive work. For work that requires the
newest advertised Scryfall snapshot, set the window to zero:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py status --max-age-days 0
```

## Diagnose locking

Inspect the persistent lock metadata and live kernel-lock state with:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py lock-status
```

The append-only metadata records the owner, worktree, operation, phase, and
timestamps, process-start identity, Git revision, and outcome so an interrupted
refresh can be diagnosed. Kernel `flock` state is authoritative and is released
when a process exits. An acquired event without a matching release may be stale
metadata; never delete `refresh.lock` or bypass a live lock based on metadata or
a PID check alone.

## Mutate the shared cache sparingly

Fetch, index, and migration commands write clone-wide state under Git's common
directory and require explicit human approval. Do not invoke them as
worktree setup or merely to see whether data exists. First use the read-only
commands above, then mutate only for missing or corrupt material, an unavailable
database schema, explicit user direction, or freshness required by the task.

When the checks above justify mutation, obtain explicit human approval and
record it on the command:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py fetch --approve-shared-write
```

The default set contains:

- `comprehensive-rules`: the current official Wizards TXT document
- `eternal-central-rules`: the canonical EC 93/94 page as searchable text
- `oracle-cards`: one representative Scryfall object per Oracle ID
- `rulings`: Scryfall's card-ruling objects

After fetching or validating the default Scryfall inputs, the command builds
or repairs the current schema-versioned SQLite index when an input checksum or
database schema changed. Other schema versions remain available to worktrees
that need them.

Request named resources when a narrower or larger corpus is appropriate:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py fetch comprehensive-rules eternal-central-rules --approve-shared-write
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py fetch default-cards --approve-shared-write
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py fetch all-cards --approve-shared-write
```

Use `default-cards` only when printing, set, language-default, or collector
metadata matters. Use `all-cards` only when every language and printing is
actually needed; it is substantially larger. Use `--force` after a known set
release or when explicitly asked to replace an otherwise-fresh snapshot.

Build or repair the database from intact cached inputs without networking:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py index --approve-shared-write
```

Use `index --force` only to diagnose or replace an otherwise-current database.
The builder streams compressed inputs, records checksums and row counts,
validates SQLite integrity, and atomically replaces the old database.

Migrate an intact legacy worktree cache only when the shared cache has not
already been populated:

```sh
python3 .agents/skills/refresh-magic-references/scripts/reference_material.py migrate --approve-shared-write
```

Treat migration as a rare shared mutation requiring the same approval as a
refresh. It holds both the shared and legacy locks, validates every payload and
schema-versioned database before copying anything, and refuses conflicting
destinations. Do not overwrite a populated shared cache merely because another
worktree still has legacy files. `--remove-source` removes only validated legacy
payloads and the legacy manifest; the persistent legacy lock inode is retained
so an old process cannot bypass it.

## Use the cache

Resolve `path manifest` to read source URLs, timestamps, checksums, and database
provenance. Resolve named source paths before searching rules TXT files with
`rg`. Use `$query-magic-references` for efficient read-only access to common
Scryfall fields and rulings. Stream retained `.jsonl.gz` files only for
unmodeled fields; do not load an entire bulk file into model context.

Treat the references as evidence, not executable engine definitions. This
repository intentionally adapts some card text and format behavior, so never
overwrite card implementations mechanically from bulk data.

## Fall back safely

If the cache is absent, stale, or cannot be refreshed, continue using the
official Wizards rules hub, Eternal Central page, or Scryfall API as
appropriate. State the version or freshness limitation when it matters. Never
make the local cache a prerequisite for unrelated work.
