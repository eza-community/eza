# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

**eza** is a modern, maintained replacement for the `ls` command written in Rust. It's designed as a fast, feature-rich alternative that adds colors, Git integration, icons, and many other enhancements over traditional file listing commands.

## Development Commands

### Building and Checking
- `cargo build` - Compile eza in debug mode
- `cargo build --release` - Compile eza in release mode with optimizations
- `cargo check` - Quick syntax and type checking
- `just build` - Alias for `cargo build`
- `just build-release` - Alias for `cargo build --release`

### Testing
- `cargo test` - Run unit tests
- `just test` - Run unit tests with workspace flag
- `just test-release` - Run tests in release mode
- `just itest` - Run integration tests (requires Nix)
- `just regen` - Fully regenerate integration tests using powertest

### Linting and Quality
- `cargo clippy` - Run Rust linter
- `just clippy` - Run clippy with touch src/main.rs first
- `nix fmt` - Format code according to project standards
- `nix flake check` - Run all checks (linting, formatting, tests)

### Documentation
- `just man` - Build man pages from Markdown sources
- `just man-1-preview` - Build and preview main man page
- `just man-5-preview` - Build and preview color configuration man page

### Development Environment (Nix recommended)
- `nix develop` - Enter development shell with all tools
- `direnv allow` - Auto-enter dev shell when entering directory (if direnv installed)

### Testing Integration Tests
- For test modifications: Use `just idump` to update test expectations
- For new features: Run `just regen` to regenerate powertest files
- Look in `tests/gen`, `tests/cmd`, and `tests/ptests` for test definitions

## Architecture Overview

### Module Structure
- **`src/main.rs`** - Entry point, command line parsing, and main execution loop
- **`src/fs/`** - File system operations and data structures
  - `file.rs` - Core `File` struct with metadata and display logic
  - `dir.rs` - Directory traversal and filtering
  - `filter.rs` - File filtering (hidden files, git ignore, etc.)
  - `feature/` - Optional features like Git integration
- **`src/options/`** - Command line argument parsing and configuration
  - `parser.rs` - Core argument parsing logic
  - `view.rs` - Output view configuration (grid, details, etc.)
  - `theme.rs` - Color and styling options
- **`src/output/`** - Display and formatting logic
  - `grid.rs`, `lines.rs`, `details.rs` - Different output formats
  - `file_name.rs` - File name rendering with colors/icons
  - `table.rs` - Details view table formatting
  - `icons.rs` - Icon assignment and display
- **`src/theme/`** - Color theme and styling system
  - `mod.rs` - Main theme configuration and YAML parsing
  - `ui_styles.rs` - UI element styling
  - `lsc.rs` - LS_COLORS and EXA_COLORS support

### Key Design Patterns
- **Options Parsing**: Uses a custom parser that handles option precedence (later args override earlier ones)
- **View System**: Modular output formats (Grid, Details, GridDetails, Lines) with shared file rendering
- **Theme System**: Supports both environment variables (LS_COLORS/EXA_COLORS) and YAML configuration files
- **Feature Flags**: Git support is optional and can be disabled at compile time
- **Error Handling**: Custom error types with user-friendly messages and suggestions

### Important Configuration
- **Theme Files**: Look for `theme.yml` in `$EZA_CONFIG_DIR` or `$XDG_CONFIG_HOME/eza`
- **Git Features**: Conditional compilation with `#[cfg(feature = "git")]`
- **Cross-platform**: Different code paths for Unix/Windows with conditional compilation

### Testing Strategy
- **Unit Tests**: Standard Rust `cargo test` for individual modules
- **Integration Tests**: Custom `trycmd` system in `tests/` with expected output files
- **Powertest**: Tool for bulk test generation and output comparison
- **Nix Integration**: Reproducible test environments using Nix flakes

### Commit and PR Requirements
- Follow Conventional Commits format (feat:, fix:, docs:, etc.)
- PRs adding flags must include: completions, man page docs, help text, and README updates
- Run `nix flake check` before submitting
- Use `just idump` or `just regen` to update tests when output changes

### Environment Variables
- `EZA_DEBUG` / `EXA_DEBUG` - Enable debug logging
- `EZA_STRICT` / `EXA_STRICT` - Control argument parsing strictness
- `EZA_CONFIG_DIR` - Theme configuration directory override