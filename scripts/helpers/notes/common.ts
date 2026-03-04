import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { findRepoRoot } from "../run_root.sh.ts";

type ArgMap = Record<string, string | boolean>;

export function parseArgs(argv: string[]): ArgMap {
  const args: ArgMap = {};
  for (let i = 0; i < argv.length; i += 1) {
    const raw = argv[i];
    if (!raw.startsWith("--")) {
      throw new Error(`Unexpected argument: ${raw}`);
    }

    if (raw === "--help") {
      args.help = true;
      continue;
    }

    const eq = raw.indexOf("=");
    if (eq >= 0) {
      const key = raw.slice(2, eq);
      const value = raw.slice(eq + 1);
      args[key] = value;
      continue;
    }

    const key = raw.slice(2);
    const next = argv[i + 1];
    if (!next || next.startsWith("--")) {
      throw new Error(`Missing value for --${key}`);
    }
    args[key] = next;
    i += 1;
  }

  return args;
}

export function resolveBaseDir(args: ArgMap): string {
  const dirArg = typeof args.dir === "string" ? args.dir : undefined;
  if (dirArg) {
    if (!existsSync(dirArg)) {
      throw new Error(`Directory does not exist: ${dirArg}`);
    }
    return dirArg;
  }

  const envDir = process.env.H265NAL_MAP_DIR;
  if (envDir && existsSync(envDir)) {
    return envDir;
  }

  const repoDefault = join(findRepoRoot(import.meta.dir), "notes", "h265nal-rs-map");
  if (existsSync(repoDefault)) {
    return repoDefault;
  }

  throw new Error(
    "Could not resolve notes directory. Pass --dir <path> or set H265NAL_MAP_DIR.",
  );
}

export function getMdFilesShallow(dir: string): string[] {
  return readdirSync(dir)
    .filter((entry) => entry.endsWith(".md"))
    .sort()
    .map((entry) => join(dir, entry));
}

export function walkMdFiles(dir: string): string[] {
  const files: string[] = [];

  const walk = (current: string): void => {
    for (const entry of readdirSync(current)) {
      const full = join(current, entry);
      const st = statSync(full);
      if (st.isDirectory()) {
        walk(full);
      } else if (st.isFile() && entry.endsWith(".md")) {
        files.push(full);
      }
    }
  };

  walk(dir);
  files.sort();
  return files;
}

export function readText(path: string): string {
  return readFileSync(path, "utf8");
}

export function writeText(path: string, content: string): void {
  writeFileSync(path, content, "utf8");
}

export function splitLines(content: string): string[] {
  return content.split("\n");
}

export function extractSections(content: string): Map<string, string> {
  const sections = new Map<string, string>();
  const pattern = /^## (.*?)\n([\s\S]*?)(?=(^## |(?![\s\S])))/gm;
  for (const match of content.matchAll(pattern)) {
    const heading = (match[1] ?? "").trim();
    const body = (match[2] ?? "").trim();
    sections.set(heading, body);
  }
  return sections;
}

export function usageAndExit(text: string): never {
  process.stdout.write(`${text}\n`);
  process.exit(0);
}
