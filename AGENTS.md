# h265nal-rs Agent Guidelines (AGENTS.md)

Welcome, Agent! These are the essential guidelines for operating autonomously in the `h265nal-rs` repository. Read them carefully before planning your changes or writing code.

## 1. Project Overview & Build System

This repository provides Rust bindings and a CLI for the C++ `h265nal` parsing library.
It consists of a Cargo workspace with several crates:
- `h265nal-sys`: FFI bindings and build logic for the C++ library.
- `h265nal-cli`: The command-line interface application.
- `h265nal-parity`: Tools and tests for ensuring output parity with the C++ implementation.
- `h265nal-test-ports`: Additional porting and testing infrastructure.

### Build Commands
- **Standard Build**: Use `cargo build`.
  - The C++ library (`h265nal`) is automatically built via CMake in the `h265nal-sys/build.rs` script. You do not need to build it manually before running `cargo build`.
- **Release Build**: Use `cargo build --release`.
- **Clean**: Use `cargo clean` if you suspect stale build artifacts.

### Test Commands
**Important:** We use `cargo nextest` as our primary test runner, NOT `cargo test`.
- **Run all tests**: `cargo nextest run`
- **Run a single test**: `cargo nextest run <test_name>` (or `cargo nextest run -E 'test(<test_name>)'` for exact matches)
- **Run tests for a specific crate**: `cargo nextest run -p <crate_name>`
- **Run with standard `cargo test` (fallback only)**: `cargo test -- <test_name>`
- **Run doc tests**: `cargo test --doc` (nextest does not support doc tests yet).

### Linting & Formatting Commands
- **Rust Linter**: `cargo clippy --workspace --all-targets -- -D warnings`
- **Rust Formatter**: `cargo fmt --all` (Run this before finalizing any Rust code changes).
- **C++ Formatter**: `make format` (Uses `clang-format -i -style=google` for the C++ code in `include/`, `src/`, and `tools/`).

---

## 2. Code Style & Architecture Guidelines

Adhere strictly to these guidelines when making modifications, refactoring, or adding new features.

### Architecture & Modularity
- **Small & Modular**: Keep code small and modular.
- **One Struct per File**: Prefer defining one primary struct per file to make navigation easier.
- **Folder Modules**: Use folder-based modules to group related concerns (e.g., `cli/`, `config/`, `render/`, `ffi/`, `tests/`).
- **Dependency Direction**: Keep dependencies flowing inward: `CLI` -> `App` -> `FFI`. Do not create sideways or cyclical dependencies between modules.

### Readability & Naming
- **Human-Readable**: Optimize for human-readable code over clever or overly dense abstractions.
- **Explicit Names**: Prefer explicit, descriptive names over short, cryptic, or overloaded names. Avoid single-letter variables except for very short loops or common mathematical conventions.
- **Focused Functions**: Keep functions focused on a single responsibility. Split functions when their intent becomes mixed.
- **Concise Comments**: Keep comments concise. Only document non-obvious intent or complex logic ("why" rather than "what"). Do not state the obvious.

### Error Handling & Safety
- **Module Boundaries**: Return `Result<T, E>` at module boundaries. Avoid panicking (`unwrap()`, `expect()`) in application logic unless it's a truly unrecoverable state or during initialization.
- **Unsafe Code**: Keep `unsafe` blocks as small as possible and strictly localized to FFI edges (e.g., inside `h265nal-sys` or specific wrappers). Always document the safety invariants of an `unsafe` block with a `// SAFETY: ...` comment.
- **No Over-checking**: Avoid defensive over-checking in internal code paths once input validation has occurred at the system boundary. Trust your internal types.

### CLI and Configuration
- **Argument Structs**: Define `clap` argument structs in dedicated files to avoid cluttering the main executable logic.
- **Runtime Config**: Convert raw CLI arguments into strongly typed, `serde`-serializable runtime configuration structs before passing them to the application logic.
- **Normalization Layer**: Keep derived or default behavior confined to a single normalization/resolution layer, rather than spreading it throughout the business logic.

### Output and Formatting Modes
- **Renderer Separation**: Keep output renderers cleanly separated by output mode. Do not mix rendering logic with parsing logic.
- **`c` Mode**: The `c` output mode must prioritize **exact parity** with the original C++ tool's output format where required. Always prefer parity-preserving behavior for native compatibility paths.
- **`json` Mode**: The `json` output should use stable field naming and a predictable structure suitable for programmatic parsing. Avoid making breaking changes to the JSON schema.
- **`pretty` Mode**: The `pretty` output should be readable first, and compact second.

### Testing Strategy
- **Integration Tests**: Prefer integration-style behavior tests for the CLI logic. Test the inputs and outputs of the application as a whole.
- **Parity Tests**: For output parity tests, use string diff assertions with clear mismatch context to clearly identify regressions compared to the C++ baseline. Ensure any changes to parsing output do not break backwards compatibility with the original `h265nal` C++ binary unless explicitly requested.

---

## 3. General Agent Workflow

To operate successfully in this repository, follow this workflow:

1. **Understand Before Modifying**: Use search tools (`glob`, `grep`, `read`) extensively to understand existing patterns before adding new code. Always look for existing abstractions that solve the problem.
2. **Mimic Existing Style**: Look at surrounding files and follow the established conventions exactly. Your code should look like it was written by the original authors.
3. **Plan Your Changes**: If the task is complex, briefly outline your plan and consider using the `todowrite` tool.
4. **Self-Correction & Verification**: Never consider a task complete without first verifying it locally. Run `cargo fmt`, `cargo clippy`, and `cargo nextest run`. Use the output of these commands to iteratively fix any issues you introduced.
5. **C++ Modifications**: If your task requires modifying the underlying C++ code (e.g., in `src/` or `include/`), ensure you run `make format` before building and testing.

## 4. External Rules and References
- This repository utilizes standard Rust community guidelines.
- For issues related to the C++ `h265nal` implementation, refer to `README.md` for historical context and rationale.
- FFI interactions should strictly follow Rust's Nomicon guidelines for C-interoperability.