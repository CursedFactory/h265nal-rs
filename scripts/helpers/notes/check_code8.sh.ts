#!/usr/bin/env bun

import { runCodeBacktickCheck } from "./check_code_lib";

runCodeBacktickCheck({
  headings: ["Theory/background context"],
  allowMissingBackticks: true,
  usage: "Usage: bun scripts/helpers/notes/check_code8.sh.ts [--dir <notes-dir>]",
});
