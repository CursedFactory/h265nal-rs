#!/usr/bin/env bun

import { join } from "node:path";

import { parseArgs, readText, resolveBaseDir, usageAndExit, walkMdFiles } from "./common";

const requiredHeadings = [
  "## Kind",
  "## Defined in",
  "## Purpose in this repo",
  "## Key API touchpoints",
  "## Direct type dependencies",
  "## Used by source files",
  "## High-level test coverage",
];

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_headings.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
for (const filePath of walkMdFiles(join(baseDir, "types"))) {
  if (filePath.endsWith("00_Index.md")) {
    continue;
  }

  const headingsWithHash = [...readText(filePath).matchAll(/^##\s+(.*)$/gm)].map(
    (match) => `## ${match[1] ?? ""}`,
  );
  const missing = requiredHeadings.filter((item) => !headingsWithHash.includes(item));
  const extra = headingsWithHash.filter((item) => !requiredHeadings.includes(item));

  if (missing.length > 0 || extra.length > 0) {
    process.stdout.write(`File: ${filePath}\n`);
    if (missing.length > 0) {
      process.stdout.write(`  Missing: ${JSON.stringify(missing)}\n`);
    }
    if (extra.length > 0) {
      process.stdout.write(`  Extra: ${JSON.stringify(extra)}\n`);
    }
  }
}
