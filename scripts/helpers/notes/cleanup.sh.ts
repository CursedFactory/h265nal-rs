#!/usr/bin/env bun

import { join } from "node:path";

import {
  extractSections,
  getMdFilesShallow,
  parseArgs,
  readText,
  resolveBaseDir,
  usageAndExit,
  writeText,
} from "./common";

const ORDER = [
  "Purpose",
  "Linked types/classes",
  "Key functions/classes",
  "Primary parse entrypoints",
  "Local dependencies",
  "High-level test coverage",
  "Theory/background context",
];

function processFile(filePath: string): boolean {
  const content = readText(filePath);
  const match = /^(---[\s\S]*?---\n+)(# .*?\n+)/.exec(content);
  if (!match) {
    process.stdout.write(`Could not parse frontmatter for ${filePath}\n`);
    return false;
  }

  const frontmatter = match[1];
  const title = match[2];
  const after = content.slice(match[0].length);
  const sections = extractSections(after);

  const normalized = new Map<string, string>();
  for (const [heading, body] of sections.entries()) {
    if (heading.startsWith("Local dependencies")) {
      normalized.set("Local dependencies", body);
    } else {
      normalized.set(heading, body);
    }
  }

  let next = `${frontmatter}${title}`;
  for (const heading of ORDER) {
    const body = normalized.get(heading);
    if (!body) {
      continue;
    }

    const cleaned: string[] = [];
    for (const rawLine of body.split("\n")) {
      let line = rawLine.trim();
      if (!line) {
        continue;
      }

      if (line.startsWith("* ")) {
        line = `- ${line.slice(2)}`;
      } else if (!line.startsWith("- ")) {
        line = `- ${line}`;
      }

      if (heading === "Purpose" || heading === "Theory/background context") {
        if (line.length > 2 && /[A-Za-z]/.test(line[2])) {
          line = `${line.slice(0, 2)}${line[2].toUpperCase()}${line.slice(3)}`;
        }
        const endsWithPunctuation =
          line.endsWith(".") || line.endsWith(";") || line.endsWith(":");
        if (!endsWithPunctuation) {
          line = `${line}.`;
        }
      }

      cleaned.push(line);
    }

    next += `## ${heading}\n${cleaned.join("\n")}\n\n`;
  }

  next = `${next.trim()}\n`;
  if (next === content) {
    return false;
  }

  writeText(filePath, next);
  return true;
}

const args = parseArgs(process.argv.slice(2));
if (args.help) {
  usageAndExit("Usage: bun scripts/helpers/notes/cleanup.sh.ts [--dir <notes-dir>]");
}

const baseDir = resolveBaseDir(args);
let touched = 0;
for (const filePath of getMdFilesShallow(join(baseDir, "src_files"))) {
  if (processFile(filePath)) {
    touched += 1;
  }
}

process.stdout.write(`Touched ${touched} files.\n`);
