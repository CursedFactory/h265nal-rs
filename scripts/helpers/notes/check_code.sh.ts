#!/usr/bin/env bun

import { runCodeBacktickCheck } from "./check_code_lib";

runCodeBacktickCheck({
  headings: ["Key functions/classes", "Primary parse entrypoints", "Local dependencies"],
  usage: "Usage: bun scripts/helpers/notes/check_code.sh.ts [--dir <notes-dir>]",
});
