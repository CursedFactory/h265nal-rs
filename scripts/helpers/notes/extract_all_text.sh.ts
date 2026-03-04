#!/usr/bin/env bun

import { join } from "node:path";

import { getMdFilesShallow, parseArgs, readText, resolveBaseDir, usageAndExit } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/extract_all_text.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
const files = getMdFilesShallow(join(baseDir, "test_files"));
files.push(join(baseDir, "09_Test_File_Index.md"));

const values = new Set<string>();
for (const filePath of files) {
  for (const line of readText(filePath).split("\n")) {
    const trimmed = line.trim();
    if (trimmed && !trimmed.startsWith("#") && !trimmed.startsWith("-") && !trimmed.startsWith("tags:")) {
      values.add(trimmed);
    }
  }
}

for (const value of [...values].sort()) {
  process.stdout.write(`${value}\n`);
}
