#!/usr/bin/env bun

import { runCodeBacktickCheck } from "./check_code_lib";

runCodeBacktickCheck({
  headings: ["High-level test coverage"],
  usage: "Usage: bun scripts/helpers/notes/check_code7.sh.ts [--dir <notes-dir>]",
});
