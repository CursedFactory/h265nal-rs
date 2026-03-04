#!/usr/bin/env bun

import { join } from "node:path";

import { getMdFilesShallow, parseArgs, readText, resolveBaseDir, usageAndExit } from "./common";

const expectedSections = [
  "## Purpose",
  "## Test cases in this file",
  "## Linked source modules",
  "## Linked types/classes",
  '## Local dependencies (`#include "..."`)',
  "## Related map notes",
  "## Dependency/context notes",
];

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_sections.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
for (const filePath of getMdFilesShallow(join(baseDir, "test_files"))) {
  const content = readText(filePath);
  for (const section of expectedSections) {
    if (!content.includes(section)) {
      process.stdout.write(`Missing '${section}' in ${filePath}\n`);
    }
  }
}
