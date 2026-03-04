import { basename, join } from "node:path";

import { extractSections, getMdFilesShallow, parseArgs, readText, resolveBaseDir, usageAndExit } from "./common";

type Config = {
  headings: string[];
  allowMissingBackticks?: boolean;
  usage: string;
};

export function runCodeBacktickCheck(config: Config): void {
  const args = parseArgs(process.argv.slice(2));
  if (args.help) {
    usageAndExit(config.usage);
  }

  const baseDir = resolveBaseDir(args);
  const srcDir = join(baseDir, "src_files");
  for (const filePath of getMdFilesShallow(srcDir)) {
    const content = readText(filePath);
    const sections = extractSections(content);
    for (const heading of config.headings) {
      const body = sections.get(heading);
      if (!body) {
        continue;
      }

      for (const rawLine of body.split("\n")) {
        const line = rawLine.trim();
        if (!line) {
          continue;
        }
        if (line.includes("`")) {
          continue;
        }
        if (!config.allowMissingBackticks) {
          process.stdout.write(
            `${basename(filePath)} - ${heading}: Missing backticks: ${line}\n`,
          );
        }
      }
    }
  }
}
