#!/usr/bin/env bun

import { basename, join } from "node:path";

import { getMdFilesShallow, parseArgs, readText, resolveBaseDir, usageAndExit } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_links3.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
for (const filePath of getMdFilesShallow(join(baseDir, "src_files"))) {
  const content = readText(filePath);
  const open = content.split("[[").length - 1;
  const close = content.split("]]" ).length - 1;
  if (open !== close) {
    process.stdout.write(`${basename(filePath)}: Mismatched brackets: ${open} open, ${close} close\n`);
  }
}
