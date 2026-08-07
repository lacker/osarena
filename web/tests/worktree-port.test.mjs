import assert from "node:assert/strict";
import test from "node:test";

import {
  chooseWorktreePort,
  parseWorktreeRoots,
  primaryWorktreePort,
} from "../worktree-port.js";

const roots = [
  "/projects/penta",
  "/projects/worktrees/bravo/penta",
  "/projects/worktrees/alpha/penta",
];

test("the primary checkout keeps port 3000 and linked choices are distinct", () => {
  const claimedPorts = new Set([primaryWorktreePort]);
  const firstPort = chooseWorktreePort(roots[1], claimedPorts);
  claimedPorts.add(firstPort);
  const secondPort = chooseWorktreePort(roots[2], claimedPorts);

  assert.equal(primaryWorktreePort, 3000);
  assert.notEqual(firstPort, secondPort);
  assert.ok(firstPort >= 10_000 && firstPort <= 49_151);
  assert.ok(secondPort >= 10_000 && secondPort <= 49_151);
});

test("a persisted assignment does not move when a colliding worktree is added", () => {
  const collisionOptions = {
    portStart: 4100,
    portEnd: 4101,
    hash: () => 0,
  };
  const persistedAssignments = new Map();
  const existingPort = chooseWorktreePort(roots[1], new Set(), collisionOptions);
  persistedAssignments.set(roots[1], existingPort);

  const newPort = chooseWorktreePort(
    roots[2],
    new Set(persistedAssignments.values()),
    collisionOptions,
  );

  assert.equal(persistedAssignments.get(roots[1]), 4100);
  assert.equal(newPort, 4101);
});

test("NUL-delimited porcelain output preserves unusual worktree paths", () => {
  const unusualRoot = "/projects/worktrees/line\nbreak/penta";
  const porcelain = [
    `worktree ${roots[0]}`,
    "HEAD abc123",
    "branch refs/heads/main",
    "",
    `worktree ${unusualRoot}`,
    "HEAD def456",
    "detached",
    "",
    "",
  ].join("\0");

  assert.deepEqual(parseWorktreeRoots(porcelain), [roots[0], unusualRoot]);
});
