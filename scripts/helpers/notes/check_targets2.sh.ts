#!/usr/bin/env bun

import { basename, join } from "node:path";

import { getMdFilesShallow, parseArgs, readText, resolveBaseDir, usageAndExit } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_targets2.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
for (const filePath of getMdFilesShallow(join(baseDir, "src_files"))) {
  const content = readText(filePath);
  for (const match of content.matchAll(/\[\[(.*?)\|.*?\]\]/g)) {
    const target = match[1] ?? "";
    const hasPrefix =
      target.startsWith("types/") ||
      target.startsWith("test_files/") ||
      target.startsWith("src_files/");
    if (!hasPrefix) {
      process.stdout.write(`${basename(filePath)}: Target missing prefix: ${target}\n`);
    }
  }
}
