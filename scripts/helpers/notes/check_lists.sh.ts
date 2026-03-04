#!/usr/bin/env bun

import { join } from "node:path";

import { parseArgs, readText, resolveBaseDir, usageAndExit, walkMdFiles } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_lists.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
const files = walkMdFiles(join(baseDir, "types"));
files.push(join(baseDir, "08_Type_Index.md"));

for (const filePath of files) {
  const lines = readText(filePath).split("\n");
  for (let i = 0; i < lines.length; i += 1) {
    const line = lines[i];
    if (/^\s*[*+]\s+/.test(line)) {
      process.stdout.write(`File: ${filePath}:${i + 1} - Issue: ${line}\n`);
    }
  }
}
