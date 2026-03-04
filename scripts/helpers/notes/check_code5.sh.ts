#!/usr/bin/env bun

import { runCodeBacktickCheck } from "./check_code_lib";

runCodeBacktickCheck({
  headings: ["Primary parse entrypoints"],
  usage: "Usage: bun scripts/helpers/notes/check_code5.sh.ts [--dir <notes-dir>]",
});
