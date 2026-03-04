#!/usr/bin/env bun

import { basename, dirname, join } from "node:path";

import {
  parseArgs,
  readText,
  resolveBaseDir,
  usageAndExit,
  walkMdFiles,
  writeText,
} from "./common";

function processFile(filePath: string): { success: boolean; reason: string } {
  let content = readText(filePath);

  content = content.replace(/\n## TL;DR\n[\s\S]*?(?=\n## |$)/g, "");
  content = content.replace(/\n## Theory and background\n[\s\S]*?(?=\n## |$)/g, "");

  const titleMatch = /^#\s+(.+)$/m.exec(content);
  if (!titleMatch || titleMatch.index === undefined) {
    return { success: false, reason: "no title found" };
  }
  const title = titleMatch[1] ?? "";
  const titleEnd = titleMatch.index + titleMatch[0].length;

  const isIndex = basename(filePath) === "00_Index.md";
  const isTest = filePath.includes("test_files");

  if (isIndex) {
    const folderName = basename(dirname(filePath));
    const tldr = `Index of ${folderName} types.`;
    const theory =
      `This index provides a structured overview of the types defined in the ${folderName} ` +
      "module, mapping out the parser engineering components.";
    const newSections = `\n\n## TL;DR\n${tldr}\n\n## Theory and background\n${theory}`;
    const next = `${content.slice(0, titleEnd)}${newSections}${content.slice(titleEnd)}`;
    writeText(filePath, next);
    return { success: true, reason: "index" };
  }

  const purposePattern = /\n##\s+Purpose(?: in this repo)?\s*\n([\s\S]*?)(?=\n##\s|$)/m;
  const purposeMatch = purposePattern.exec(content);
  if (!purposeMatch || purposeMatch.index === undefined) {
    return { success: false, reason: "no purpose found" };
  }

  const purposeBody = purposeMatch[1] ?? "";
  content = `${content.slice(0, purposeMatch.index)}${content.slice(purposeMatch.index + purposeMatch[0].length)}`;

  const bullets = purposeBody
    .split("\n")
    .map((line) => line.trim().replace(/^-\s*/, "").trim())
    .filter(Boolean);
  let cleanedPurpose = bullets.join(" ");

  if (
    cleanedPurpose.length > 0 &&
    /[A-Z]/.test(cleanedPurpose[0]) &&
    (cleanedPurpose.length === 1 || !/[A-Z]/.test(cleanedPurpose[1]))
  ) {
    cleanedPurpose = `${cleanedPurpose[0].toLowerCase()}${cleanedPurpose.slice(1)}`;
  }

  let tldr: string;
  let theory: string;
  if (isTest) {
    tldr = "Unit tests verifying parser behavior and edge cases for the associated H.265 module.";
    theory =
      "This file contains unit tests designed to validate the correctness of the parser " +
      "implementation. It exercises both standard parsing paths and selected edge cases to ensure " +
      "robust handling of the bitstream.";
  } else {
    if (cleanedPurpose.endsWith(".")) {
      cleanedPurpose = cleanedPurpose.slice(0, -1);
    }
    tldr = `Implements ${cleanedPurpose}.`;
    theory =
      `The primary focus of this component is ${cleanedPurpose}. ` +
      "It provides the necessary parsing logic and state management to process this aspect of the " +
      "H.265 bitstream, ensuring correct interpretation of the syntax elements.";
  }

  const titleMatchAfter = /^#\s+(.+)$/m.exec(content);
  if (!titleMatchAfter || titleMatchAfter.index === undefined) {
    return { success: false, reason: `title missing after purpose removal (${title})` };
  }

  const titleEndAfter = titleMatchAfter.index + titleMatchAfter[0].length;
  const newSections = `\n\n## TL;DR\n${tldr}\n\n## Theory and background\n${theory}`;
  const next = `${content.slice(0, titleEndAfter)}${newSections}${content.slice(titleEndAfter)}`;
  writeText(filePath, next);
  return { success: true, reason: "standard" };
}

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/normalize_notes.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
const files = [
  ...walkMdFiles(join(baseDir, "src_files")),
  ...walkMdFiles(join(baseDir, "test_files")),
  ...walkMdFiles(join(baseDir, "types")),
].sort();

const stats = {
  src_files: 0,
  test_files: 0,
  types: 0,
  anomalies: [] as string[],
};

for (const filePath of files) {
  try {
    const result = processFile(filePath);
    if (result.success) {
      if (filePath.includes("src_files")) {
        stats.src_files += 1;
      } else if (filePath.includes("test_files")) {
        stats.test_files += 1;
      } else if (filePath.includes("types")) {
        stats.types += 1;
      }
    } else {
      stats.anomalies.push(`${basename(filePath)}: ${result.reason}`);
    }
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    stats.anomalies.push(`${basename(filePath)}: Error - ${message}`);
  }
}

process.stdout.write(`src_files touched: ${stats.src_files}\n`);
process.stdout.write(`test_files touched: ${stats.test_files}\n`);
process.stdout.write(`types touched: ${stats.types}\n`);
process.stdout.write("Anomalies:\n");
for (const item of stats.anomalies) {
  process.stdout.write(`  - ${item}\n`);
}
