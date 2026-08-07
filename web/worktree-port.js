import { closeSync, openSync, readFileSync, statSync, unlinkSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

export const primaryWorktreePort = 3000;

const linkedPortStart = 10_000;
const linkedPortEnd = 49_151;
const portFileName = ".dev-port";
const lockWaiter = new Int32Array(new SharedArrayBuffer(4));

const normalizeRoot = (root) => path.resolve(root);

const hashRoot = (root) => {
  let hash = 0x811c9dc5;

  for (const character of root) {
    hash ^= character.codePointAt(0);
    hash = Math.imul(hash, 0x01000193) >>> 0;
  }

  return hash;
};

const isMissingFileError = (error) =>
  error instanceof Error && "code" in error && error.code === "ENOENT";

const portFileForRoot = (root) => path.join(root, "web", portFileName);

const readPersistedPort = (root) => {
  const portFile = portFileForRoot(root);
  let value;

  try {
    value = readFileSync(portFile, "utf8").trim();
  } catch (error) {
    if (isMissingFileError(error)) return undefined;
    throw error;
  }

  const port = Number(value);
  if (!Number.isInteger(port) || port < linkedPortStart || port > linkedPortEnd) {
    throw new Error(`Invalid worktree development port in ${portFile}: ${value}`);
  }

  return port;
};

const withAllocationLock = (commonGitDirectory, allocate) => {
  const lockFile = path.join(
    tmpdir(),
    `penta-worktree-port-${hashRoot(commonGitDirectory)}.lock`,
  );
  const deadline = Date.now() + 5000;

  while (true) {
    try {
      closeSync(openSync(lockFile, "wx"));
      break;
    } catch (error) {
      if (!error || typeof error !== "object" || !("code" in error) || error.code !== "EEXIST") {
        throw error;
      }

      try {
        if (Date.now() - statSync(lockFile).mtimeMs > 30_000) {
          unlinkSync(lockFile);
          continue;
        }
      } catch (statError) {
        if (isMissingFileError(statError)) continue;
        throw statError;
      }

      if (Date.now() >= deadline) {
        throw new Error(`Timed out waiting to allocate a worktree port: ${lockFile}`);
      }

      Atomics.wait(lockWaiter, 0, 0, 25);
    }
  }

  try {
    return allocate();
  } finally {
    try {
      unlinkSync(lockFile);
    } catch (error) {
      if (!isMissingFileError(error)) throw error;
    }
  }
};

export function parseWorktreeRoots(porcelain) {
  return porcelain
    .split("\0")
    .filter((field) => field.startsWith("worktree "))
    .map((field) => field.slice("worktree ".length));
}

export function chooseWorktreePort(
  worktreeRoot,
  claimedPorts,
  {
    portStart = linkedPortStart,
    portEnd = linkedPortEnd,
    hash = hashRoot,
  } = {},
) {
  const availablePortCount = portEnd - portStart + 1;
  let port = portStart + (hash(normalizeRoot(worktreeRoot)) % availablePortCount);

  for (let attempt = 0; attempt < availablePortCount; attempt += 1) {
    if (!claimedPorts.has(port)) return port;
    port = port === portEnd ? portStart : port + 1;
  }

  throw new Error("There are more linked worktrees than available development ports");
}

export function getWorktreeDevPort({ cwd = fileURLToPath(new URL(".", import.meta.url)) } = {}) {
  let commonGitDirectory;
  let currentRoot;
  let worktreeOutput;

  try {
    currentRoot = execFileSync("git", ["rev-parse", "--show-toplevel"], {
      cwd,
      encoding: "utf8",
    }).trim();
    commonGitDirectory = execFileSync("git", ["rev-parse", "--git-common-dir"], {
      cwd,
      encoding: "utf8",
    }).trim();
    worktreeOutput = execFileSync("git", ["worktree", "list", "--porcelain", "-z"], {
      cwd,
      encoding: "utf8",
    });
  } catch {
    return primaryWorktreePort;
  }

  const worktreeRoots = parseWorktreeRoots(worktreeOutput).map(normalizeRoot);
  const normalizedCurrentRoot = normalizeRoot(currentRoot);

  if (!worktreeRoots.includes(normalizedCurrentRoot)) {
    throw new Error(`Current worktree is missing from \`git worktree list\`: ${currentRoot}`);
  }

  if (normalizedCurrentRoot === worktreeRoots[0]) return primaryWorktreePort;

  const normalizedCommonGitDirectory = path.resolve(cwd, commonGitDirectory);
  return withAllocationLock(normalizedCommonGitDirectory, () => {
    const persistedAssignments = new Map();

    for (const root of worktreeRoots.slice(1)) {
      const port = readPersistedPort(root);
      if (port !== undefined) persistedAssignments.set(root, port);
    }

    const persistedPort = persistedAssignments.get(normalizedCurrentRoot);
    if (persistedPort !== undefined) return persistedPort;

    const claimedPorts = new Set([primaryWorktreePort, ...persistedAssignments.values()]);
    const port = chooseWorktreePort(normalizedCurrentRoot, claimedPorts);
    writeFileSync(portFileForRoot(normalizedCurrentRoot), `${port}\n`, { flag: "wx" });
    return port;
  });
}

const invokedPath = process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href;

if (invokedPath === import.meta.url) {
  const port = getWorktreeDevPort();
  console.log(process.argv.includes("--url") ? `http://localhost:${port}` : port);
}
