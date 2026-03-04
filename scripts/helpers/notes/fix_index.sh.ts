#!/usr/bin/env bun

import { basename, join } from "node:path";

import { parseArgs, readText, resolveBaseDir, usageAndExit, walkMdFiles, writeText } from "./common";

function fixIndex(filePath: string): void {
  const content = readText(filePath);
  if (content.includes("- Total notes:")) {
    return;
  }

  const count = [...content.matchAll(/\[\[.*?\]\]/g)].length;
  if (count <= 0) {
    return;
  }

  if (basename(filePath) !== "00_Index.md") {
    return;
  }

  const titleMatch = /^#\s+(.+)$/m.exec(content);
  if (!titleMatch || titleMatch.index === undefined) {
    return;
  }

  const titleEnd = titleMatch.index + titleMatch[0].length;
  const insertion = `\n- Total notes: **${count}**`;
  const next = `${content.slice(0, titleEnd)}${insertion}${content.slice(titleEnd)}`;
  writeText(filePath, next);
}

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/fix_index.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
for (const filePath of walkMdFiles(join(baseDir, "types"))) {
  if (filePath.endsWith("00_Index.md")) {
    fixIndex(filePath);
  }
}
