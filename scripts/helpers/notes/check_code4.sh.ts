#!/usr/bin/env bun

import { runCodeBacktickCheck } from "./check_code_lib";

runCodeBacktickCheck({
  headings: ["Linked types/classes"],
  usage: "Usage: bun scripts/helpers/notes/check_code4.sh.ts [--dir <notes-dir>]",
});
