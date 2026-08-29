# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

vim-dogrun is a dark Neovim/Vim colorscheme with extensive plugin support (50+ plugins). The project uses a **Rust-based generator** to create optimized, precompiled colorscheme files rather than maintaining Vim script manually.

**Key Architecture:**
- Source code: Rust generator in `generator/` directory
- Generated files: `colors/dogrun.vim`, `autoload/lightline/colorscheme/dogrun.vim`, `autoload/clap/themes/dogrun.vim`, `wezterm/dogrun.toml`, and the fzf block in `README.md`
- **NEVER edit generated files directly** - always modify the Rust source and regenerate

## Development Commands

All commands should be run from the `generator/` directory:

```bash
# Install development tools (just, bacon)
cd generator && mise install

# Generate colorscheme files (writes to parent directory)
cd generator && just build

# Watch mode for development (uses bacon)
cd generator && just watch

# Run generator without output (for testing)
cd generator && just debug

# Run tests
cd generator && just test

# Format code
cd generator && just fmt

# Run linter (strict mode with format check)
cd generator && just lint

# Build release binary
cd generator && just release

# Run all checks (lint, test)
cd generator && just check
```

## Code Architecture

### Generator Structure

**Main Components:**
1. `generator/src/main.rs` - CLI entry point (argument parsing, file IO)
2. `generator/src/writer.rs` - `Writer` and the output formats
3. `generator/src/highlight.rs` - All highlight group definitions (~880 lines)
4. `generator/src/conv.rs` - Color conversion utilities (hex→LAB→cterm)
5. `generator/src/lib.rs` - Module exports
6. `generator/tests/` - Integration tests, including `golden_outputs.rs` which byte-compares generated output with the committed files

### Color System

**Color Conversion Pipeline:**
```
Hex (#rrggbb) → LAB color space → 256-color terminal code
```

**Key Features:**
- Uses Delta E 2000 algorithm for perceptually accurate color matching
- 256-color palette held in a `const` table with Lab values pre-computed in a `std::sync::LazyLock`
- HSV math is a local port of tint 1.0's algorithm on purpose — palette's Hsv rounds differently and would shift shipped gui colors
- HSV manipulation utilities: `hue()`, `saturate()`, `darken()`, `lighten()`

**Core Data Structures:**
```rust
pub struct Color {
    pub gui: String,    // Hex color
    pub cterm: String,  // 256-color code
}

pub struct Highlight {
    pub name: &'static str,
    pub fg: ColorName,
    pub bg: ColorName,
    pub sp: ColorName,          // Special/underline color
    pub attr: HighlightAttr,    // Bold, Italic, etc.
    pub scope: HighlightScope,  // All, Nvim080OrLater
}
```

### Highlight Definition System

Uses macro-based DSL in `highlight.rs`:
```rust
hi!("Normal", mainfg, mainbg, -, -, -);
hi!("Comment", commentfg, -, -, None, -);
```

**Categories:**
- Basic Vim highlights (Normal, Comment, etc.)
- Treesitter semantic tokens
- LSP highlights
- Plugin integrations (50+ plugins)

### Writer System

The `Writer` struct (in `generator/src/writer.rs`, exported as `dogrun::writer`) generates five outputs:
1. `write_colorscheme()` → `colors/dogrun.vim` (main colorscheme)
2. `write_lightline()` → `autoload/lightline/colorscheme/dogrun.vim`
3. `write_clap()` → `autoload/clap/themes/dogrun.vim`
4. `write_wezterm()` → `wezterm/dogrun.toml`
5. `update_readme_fzf()` → fzf color block in `README.md` (between the `<!-- fzf:start -->` / `<!-- fzf:end -->` markers)

## Development Workflow

### Adding a New Plugin

1. Add highlight groups to `generator/src/highlight.rs`
2. Use the `hi!()` macro with appropriate color names
3. Run `cd generator && just build`
4. Test in Neovim/Vim with the plugin installed
5. Update plugin list in README.md

### Modifying Colors

1. Locate color definitions in `generator/src/highlight.rs`
   - Base colors defined at top (e.g., `mainbg`, `mainfg`)
   - Derived colors use `saturate()`, `darken()`, etc.
2. Modify color values or relationships
3. Run generator: `cd generator && just build`
4. Verify changes in Vim/Neovim

### Testing Changes

```bash
# 1. Generate files
cd generator && just build

# 2. Check git diff to verify expected changes
git diff  # covers colors/, autoload/, wezterm/, and README.md

# (equivalently: cd generator && just fresh-check)

# 3. Test in Neovim/Vim
nvim -c "colorscheme dogrun"
```

## CI/CD

GitHub Actions workflow (triggered on pushes to `main` and on pull requests) validates:
1. Code formatting (`cargo fmt`)
2. Linting with clippy (`-D warnings` strict mode)
3. Tests pass (`cargo test --locked`)
4. **Generated files are up-to-date** - fails if regenerating changes or adds any file in the repository

The Rust toolchain is pinned in `generator/rust-toolchain.toml` (no Renovate manager exists for it — bump the `channel` manually when a new stable is wanted, keeping CI and local in sync).

**Important:** Always run `just build` and commit generated files before pushing.

## File Organization

**Source Files (edit these):**
- `generator/src/main.rs` - CLI entry point
- `generator/src/writer.rs` - Output writers
- `generator/src/highlight.rs` - Color scheme definitions
- `generator/src/conv.rs` - Color utilities
- `generator/tests/` - Integration tests
- `generator/Cargo.toml` - Rust dependencies

**Generated Files (do not edit directly):**
- `colors/dogrun.vim` - Main colorscheme (~570 lines)
- `autoload/lightline/colorscheme/dogrun.vim` - lightline theme
- `autoload/clap/themes/dogrun.vim` - vim-clap theme
- `wezterm/dogrun.toml` - WezTerm theme
- fzf block in `README.md`

**Configuration:**
- `.github/workflows/ci.yaml` - CI pipeline
- `generator/rust-toolchain.toml` - Pinned Rust toolchain (used by rustup locally and in CI)
- `generator/justfile` - Build commands (task runner)
- `generator/mise.toml` - Development tool management (just, bacon)
- `term/dogrun.itermcolors` - iTerm2 theme (hand-maintained)

## Dependencies

**Rust Crates:**
- `palette` 0.7 - sRGB→Lab conversion and CIEDE2000 color difference (the only runtime dependency; HSV math is implemented locally in `conv.rs`)
- Dev-dependencies: `toml`, `regex`, `tempfile` (integration tests)

Edition 2024, `rust-version = "1.85"`.

## Design Philosophy

- **Perceptually accurate** color matching (not Euclidean RGB distance)
- **Consistent brightness** across color pairs for readability
- **Semantic color assignment**:
  - Purple/Blue: keywords, statements
  - Green: strings
  - Cyan: constants
  - Yellow/Beige: types
  - Pink/Magenta: special characters
- **High contrast** for accessibility
- **Generator-based** for maintainability and consistency
