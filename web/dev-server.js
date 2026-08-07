import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

import { getWorktreeDevPort } from "./worktree-port.js";

const forwardedArguments = process.argv.slice(2);
const portOverride = forwardedArguments.find(
  (argument) =>
    argument === "--port" ||
    argument.startsWith("--port=") ||
    argument === "-p" ||
    /^-p(?:=)?\d+$/.test(argument),
);

if (portOverride) {
  throw new Error(
    `Development ports are assigned per worktree; use \`pnpm run dev:url\` instead of ${portOverride}`,
  );
}

const vinextEntry = fileURLToPath(new URL("cli.js", import.meta.resolve("vinext")));
const result = spawnSync(
  process.execPath,
  [vinextEntry, "dev", "--port", String(getWorktreeDevPort()), ...forwardedArguments],
  {
    env: {
      ...process.env,
      WRANGLER_LOG_PATH: process.env.WRANGLER_LOG_PATH ?? ".wrangler/wrangler.log",
    },
    stdio: "inherit",
  },
);

if (result.error) throw result.error;
if (result.signal) process.kill(process.pid, result.signal);
process.exitCode = result.status ?? 1;
