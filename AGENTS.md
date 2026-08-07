# Repository instructions

## UI changes

For every change that can affect the web interface:

1. Start or restart the local server from the current working tree. Confirm that
   the worktree-specific URL from `cd web && pnpm run dev:url` is served by that
   process; do not accept a fallback port or assume an older server picked up
   the change.
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
