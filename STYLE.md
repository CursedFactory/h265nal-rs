# Rust Style Notes

These notes are the default style for Rust changes in this repository.

## Architecture
- Keep code small and modular.
- Prefer one primary struct per file.
- Use folder modules to group related concerns (`cli/`, `config/`, `render/`, `ffi/`, `tests/`).
- Keep dependencies flowing inward (CLI -> app -> ffi), not sideways.

## Readability
- Optimize for human-readable code over clever abstractions.
- Prefer explicit names over short or overloaded names.
- Keep functions focused; split when function intent becomes mixed.
- Keep comments concise and only for non-obvious intent.

## Error Handling
- Return `Result` at module boundaries.
- Keep `unsafe` small and localized to FFI edges.
- Avoid defensive over-checking in internal code paths after validation.

## CLI and Config
- Define clap argument structs in dedicated files.
- Convert CLI args into serde-serializable runtime config structs.
- Keep derived/default behavior in one normalization layer.

## Output and Formatting
- Keep renderers separated by output mode.
- `c` output mode should prioritize exact parity where required.
- `json` output should use stable field naming and predictable structure.
- `pretty` output should be readable first, compact second.

## Tests
- Prefer integration-style behavior tests for CLI.
- Use `cargo nextest run` as the primary test runner.
- For output parity tests, use string diff assertions with clear mismatch context.
