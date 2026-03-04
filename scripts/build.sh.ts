#!/usr/bin/env bun

import { mkdirSync } from "node:fs";
import { join } from "node:path";

import { findRepoRoot } from "./helpers/run_root.sh.ts";
import { runCommand } from "./helpers/run_command.sh.ts";

const repoRoot = findRepoRoot(import.meta.dir);
const buildDir = join(repoRoot, "build");

mkdirSync(buildDir, { recursive: true });

await runCommand([
  "cmake",
  "..",
  "-DBUILD_TESTS=ON",
  "-DBUILD_CLANG_FUZZER=OFF",
  "-DCMAKE_CXX_CLANG_TIDY=",
], { cwd: buildDir });
await runCommand(["make"], { cwd: buildDir });
await runCommand(["ctest", "--output-on-failure"], { cwd: buildDir });
