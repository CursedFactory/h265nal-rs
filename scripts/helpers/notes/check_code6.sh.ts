#!/usr/bin/env bun

import { runCodeBacktickCheck } from "./check_code_lib";

runCodeBacktickCheck({
  headings: ["Local dependencies"],
  usage: "Usage: bun scripts/helpers/notes/check_code6.sh.ts [--dir <notes-dir>]",
});
