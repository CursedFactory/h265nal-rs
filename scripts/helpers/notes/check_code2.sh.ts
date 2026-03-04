#!/usr/bin/env bun

import { runCodeBacktickCheck } from "./check_code_lib";

runCodeBacktickCheck({
  headings: ["Linked types/classes", "High-level test coverage"],
  usage: "Usage: bun scripts/helpers/notes/check_code2.sh.ts [--dir <notes-dir>]",
});
