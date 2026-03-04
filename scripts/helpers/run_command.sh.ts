#!/usr/bin/env bun

type RunCommandOptions = {
  cwd?: string;
  env?: Record<string, string>;
  allowFailure?: boolean;
};

export async function runCommand(
  command: string[],
  options: RunCommandOptions = {},
): Promise<number> {
  console.log(`$ ${command.join(" ")}`);

  const child = Bun.spawn(command, {
    cwd: options.cwd,
    env: {
      ...globalThis.process.env,
      ...(options.env ?? {}),
    },
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  });

  const exitCode = await child.exited;
  if (exitCode !== 0 && !options.allowFailure) {
    throw new Error(`Command failed (exit=${exitCode}): ${command.join(" ")}`);
  }

  return exitCode;
}
