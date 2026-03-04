#!/usr/bin/env bun

import { join } from "node:path";

import { getMdFilesShallow, parseArgs, readText, resolveBaseDir, usageAndExit } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_whitespace.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
const files = getMdFilesShallow(join(baseDir, "test_files"));
files.push(join(baseDir, "09_Test_File_Index.md"));

for (const filePath of files) {
  const content = readText(filePath);
  if (content.includes(" \n")) {
    process.stdout.write(`Trailing whitespace in ${filePath}\n`);
  }
  if (content.includes("\n\n\n")) {
    process.stdout.write(`Multiple blank lines in ${filePath}\n`);
  }
}
