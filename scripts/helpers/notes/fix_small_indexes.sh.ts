#!/usr/bin/env bun

import { basename, join } from "node:path";

import {
  getMdFilesShallow,
  parseArgs,
  readText,
  resolveBaseDir,
  usageAndExit,
  writeText,
} from "./common";

function fixIndex(baseDir: string, folderName: string): void {
  const folderPath = join(baseDir, "types", folderName);
  const indexPath = join(folderPath, "00_Index.md");

  const files = getMdFilesShallow(folderPath)
    .filter((filePath) => !filePath.endsWith("00_Index.md"))
    .map((filePath) => basename(filePath));

  const links = files.map((nameWithExt) => {
    const name = nameWithExt.slice(0, -3);
    return `- [[types/${folderName}/${name}|\`${name}\`]]`;
  });

  let content = readText(indexPath);
  if (links.length > 0) {
    if (!content.includes("- Total notes:")) {
      const titleMatch = /^#\s+(.+)$/m.exec(content);
      if (titleMatch && titleMatch.index !== undefined) {
        const titleEnd = titleMatch.index + titleMatch[0].length;
        const insertion = `\n- Total notes: **${links.length}**`;
        content = `${content.slice(0, titleEnd)}${insertion}${content.slice(titleEnd)}`;
      }
    }

    content = `${content}\n\n${links.join("\n")}\n`;
    writeText(indexPath, content);
  }
}

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit(
    "Usage: bun scripts/helpers/notes/fix_small_indexes.sh.ts [--dir <notes-dir>] [--folders enums,io,support]",
  );
}

const baseDir = resolveBaseDir(args);
const foldersArg = typeof args.folders === "string" ? args.folders : "enums,io,support";
for (const folder of foldersArg.split(",").map((item) => item.trim()).filter(Boolean)) {
  fixIndex(baseDir, folder);
}
