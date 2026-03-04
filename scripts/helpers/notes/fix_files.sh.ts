#!/usr/bin/env bun

import { join } from "node:path";

import {
  getMdFilesShallow,
  parseArgs,
  readText,
  resolveBaseDir,
  usageAndExit,
  writeText,
} from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/fix_files.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
let filesChanged = 0;

for (const filePath of getMdFilesShallow(join(baseDir, "test_files"))) {
  const content = readText(filePath);
  let next = content.replaceAll("## Test cases in this file", "## Test cases");
  next = next.replaceAll('## Local dependencies (`#include "..."`)', "## Local dependencies");
  if (next !== content) {
    writeText(filePath, next);
    filesChanged += 1;
  }
}

const indexPath = join(baseDir, "09_Test_File_Index.md");
const indexContent = readText(indexPath);
const indexNext = indexContent.replaceAll("->", "\u2192");
if (indexNext !== indexContent) {
  writeText(indexPath, indexNext);
  filesChanged += 1;
}

process.stdout.write(`Files changed: ${filesChanged}\n`);
