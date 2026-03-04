#!/usr/bin/env bun

import { join } from "node:path";

import { parseArgs, readText, resolveBaseDir, usageAndExit, walkMdFiles } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_spacing.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
const files = walkMdFiles(join(baseDir, "types"));
files.push(join(baseDir, "08_Type_Index.md"));

for (const filePath of files) {
  const content = readText(filePath);
  if (/[ \t]+$/m.test(content)) {
    process.stdout.write(`File: ${filePath} - Issue: Trailing spaces\n`);
  }
  if (/\n{3,}/.test(content)) {
    process.stdout.write(`File: ${filePath} - Issue: Multiple blank lines\n`);
  }
  if (!content.endsWith("\n")) {
    process.stdout.write(`File: ${filePath} - Issue: No trailing newline\n`);
  }
  if (content.endsWith("\n\n")) {
    process.stdout.write(`File: ${filePath} - Issue: Multiple trailing newlines\n`);
  }
}
