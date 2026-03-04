#!/usr/bin/env bun

import { join } from "node:path";

import { parseArgs, readText, resolveBaseDir, usageAndExit, walkMdFiles } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_empty_lists.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
const issues: Array<{ path: string; heading: string }> = [];

for (const filePath of walkMdFiles(join(baseDir, "types"))) {
  if (filePath.endsWith("00_Index.md")) {
    continue;
  }

  const content = readText(filePath);
  const headings = [...content.matchAll(/^##\s+(.*)$/gm)].map((match) => (match[1] ?? "").trim());
  for (let i = 0; i < headings.length; i += 1) {
    const heading = headings[i];
    if (!heading) {
      continue;
    }

    if (i < headings.length - 1) {
      const next = headings[i + 1];
      const pattern = new RegExp(`## ${heading}\\n+## ${next}`);
      if (pattern.test(content)) {
        issues.push({ path: filePath, heading });
      }
    } else {
      const pattern = new RegExp(`## ${heading}\\n+$`);
      if (pattern.test(content)) {
        issues.push({ path: filePath, heading });
      }
    }
  }
}

for (const item of issues) {
  process.stdout.write(`File: ${item.path} - Empty heading: ${item.heading}\n`);
}
