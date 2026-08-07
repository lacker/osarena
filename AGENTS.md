# Repository instructions

## Optional reference material

Magic rules and card data may be available in an optional development cache
under Git's common directory. One cache is shared by every linked worktree in
the clone. Use `$query-magic-references` for efficient read-only access to its
generated Scryfall SQLite index and `$refresh-magic-references` to locate,
inspect, migrate, populate, or rebuild the cache.

Locating the cache, querying it, checking `status scryfall-index`, and viewing
`lock-status` are low-friction read operations; they do not justify a refresh.
Fetch, index, and migration commands mutate shared clone state and require
explicit human approval. Run them rarely: only when material is missing or
corrupt, the required database schema is unavailable, or the current task
genuinely requires fresher source data. Do not refresh merely because a new
worktree was created. If the cache is absent, stale for an irrelevant purpose,
or unavailable, continue with appropriate authoritative online sources.

Treat `refresh.lock` metadata as diagnostic information. The kernel lock is
authoritative; never delete or bypass the lock merely because its recorded
owner appears stale. Do not commit or ship downloaded reference payloads.

Treat both reference skills as maintained development tooling. When repeated
work exposes a missing field, relationship, index, or query pattern, update the
refresh builder, both skills, and the documented schema together, then rebuild
and validate the cache. Avoid expanding them for isolated one-off questions.

## UI changes

For every change that can affect the web interface:

1. Start or restart the local server from the current working tree. Confirm that
   `http://localhost:3000` is served by that process; do not accept a fallback
   port or assume an older server picked up the change.
2. Open the rendered application in a browser and inspect it visually. A
   successful build, DOM snapshot, or HTTP response is not sufficient.
3. Check at least a 1280×720 laptop viewport. Verify that important content is
   visible and readable, with no unintended clipping, overlap, off-screen
   controls, or inaccessible horizontal overflow.
4. Exercise enough UI state to display the changed component. For game-table
   changes, check cards in hand and cards on the battlefield when applicable.
5. Take a fresh screenshot after the final code change and inspect it before
   reporting completion.

Keep the verified local server running for the user unless they ask otherwise.
