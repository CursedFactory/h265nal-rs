#!/usr/bin/env bun

import { runCodeBacktickCheck } from "./check_code_lib";

runCodeBacktickCheck({
  headings: ["Key functions/classes"],
  usage: "Usage: bun scripts/helpers/notes/check_code3.sh.ts [--dir <notes-dir>]",
});
