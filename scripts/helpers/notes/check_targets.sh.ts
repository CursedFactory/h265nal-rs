#!/usr/bin/env bun

import { basename, join } from "node:path";

import { getMdFilesShallow, parseArgs, readText, resolveBaseDir, usageAndExit } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_targets.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
for (const filePath of getMdFilesShallow(join(baseDir, "src_files"))) {
  const content = readText(filePath);
  for (const match of content.matchAll(/\[\[(.*?)\|.*?\]\]/g)) {
    const target = match[1] ?? "";
    if (target.endsWith(".cc") || target.endsWith(".h")) {
      process.stdout.write(`${basename(filePath)}: Target ends with extension: ${target}\n`);
    }
    if (target.includes(" ")) {
      process.stdout.write(`${basename(filePath)}: Target contains space: ${target}\n`);
    }
  }
}
