#!/usr/bin/env bun

import { basename, join } from "node:path";

import { getMdFilesShallow, parseArgs, readText, resolveBaseDir, usageAndExit } from "./common";

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/check_links.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
for (const filePath of getMdFilesShallow(join(baseDir, "src_files"))) {
  const content = readText(filePath);
  for (const match of content.matchAll(/\[\[(.*?)\]\]/g)) {
    const link = match[1] ?? "";
    if (!link.includes("|")) {
      process.stdout.write(`${basename(filePath)}: Link missing pipe: ${link}\n`);
      continue;
    }
    const [target, text] = link.split("|", 2);
    if (!text.startsWith("`") || !text.endsWith("`")) {
      process.stdout.write(
        `${basename(filePath)}: Malformed link text (missing backticks): ${link}\n`,
      );
    }
    if (target.includes(" ")) {
      process.stdout.write(`${basename(filePath)}: Malformed link target (contains space): ${link}\n`);
    }
  }
}
