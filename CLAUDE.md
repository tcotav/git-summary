# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/claude-code) when working with this codebase.

## Project Overview

git-summary is a Rust CLI tool that summarizes git commits using Claude's LLM API. It extracts commit data from a local git repository and generates human-readable summaries organized by theme/area.

## Build & Run Commands

```bash
# Build (debug)
cargo build

# Build (release) - binary goes to ~/.cargo/target/release/git-summary
cargo build --release

# Run directly
cargo run -- --repo /path/to/repo --since "1 week ago"

# Run with LLM summary (requires ANTHROPIC_API_KEY)
cargo run -- --since yesterday --llm

# Run tests
cargo test
```

Note: This project uses a global Cargo config that places the target directory at `~/.cargo/target/` rather than `./target/`.

## Architecture

```
src/
├── main.rs           # Entry point, CLI orchestration
├── cli.rs            # Clap argument parsing
├── git.rs            # Git operations (shells out to git CLI)
├── summarizer.rs     # Anthropic API integration
└── formatters/
    ├── mod.rs        # Formatter trait
    ├── pretty.rs     # Terminal output with colors
    ├── markdown.rs   # Markdown tables
    └── json.rs       # JSON output
```

## Key Design Decisions

- **Shells out to git** rather than using libgit2 - simpler and matches user's git behavior
- **No author tracking** - intentionally omitted to prevent misuse for employee monitoring
- **Bullet point summaries** - LLM outputs bulleted lists for easier scanning
- **Area grouping** - commits grouped by top-level directory

## Environment Requirements

- `ANTHROPIC_API_KEY` environment variable required for LLM summaries (when using `--llm` flag)
- By default, no API key is needed - LLM summaries are opt-in with the `--llm` flag
