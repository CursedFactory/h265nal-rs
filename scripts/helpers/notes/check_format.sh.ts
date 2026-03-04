#!/usr/bin/env bun

import { join } from "node:path";

import { getMdFilesShallow, parseArgs, readText, resolveBaseDir, usageAndExit } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_format.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
const files = getMdFilesShallow(join(baseDir, "test_files"));
files.push(join(baseDir, "09_Test_File_Index.md"));

for (const filePath of files) {
  const content = readText(filePath);
  if (content.includes("* ")) {
    process.stdout.write(`Found '* ' in ${filePath}\n`);
  }
  if (content.includes("  -")) {
    process.stdout.write(`Found '  -' in ${filePath}\n`);
  }
  if (content.includes("## Purpose\n\n")) {
    process.stdout.write(`Found extra newline after Purpose in ${filePath}\n`);
  }
}
