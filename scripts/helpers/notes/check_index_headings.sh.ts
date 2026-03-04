#!/usr/bin/env bun

import { join } from "node:path";

import { parseArgs, readText, resolveBaseDir, usageAndExit, walkMdFiles } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_index_headings.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
const issues: Array<{ path: string; issue: string }> = [];

const checkFile = (filePath: string): void => {
  const headings = [...readText(filePath).matchAll(/^##\s+(.*)$/gm)].map((match) => (match[1] ?? "").trim());
  for (const heading of headings) {
    if (!heading) {
      issues.push({ path: filePath, issue: "Empty heading" });
    }
  }
};

for (const filePath of walkMdFiles(join(baseDir, "types"))) {
  if (filePath.endsWith("00_Index.md")) {
    checkFile(filePath);
  }
}

checkFile(join(baseDir, "08_Type_Index.md"));

for (const item of issues) {
  process.stdout.write(`File: ${item.path} - Issue: ${item.issue}\n`);
}
