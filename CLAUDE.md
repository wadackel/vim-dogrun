# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

vim-dogrun is a dark Neovim/Vim colorscheme with extensive plugin support (50+ plugins). The project uses a **Rust-based generator** to create optimized, precompiled colorscheme files rather than maintaining Vim script manually.

**Key Architecture:**
- Source code: Rust generator in `generator/` directory
- Generated files: `colors/dogrun.vim`, `autoload/lightline/colorscheme/dogrun.vim`, `autoload/clap/themes/dogrun.vim`
- **NEVER edit generated files directly** - always modify the Rust source and regenerate

## Development Commands

All commands should be run from the `generator/` directory:

```bash
# Generate colorscheme files (writes to parent directory)
cd generator && make build

# Watch mode for development (uses bacon)
cd generator && make watch

# Run generator without output (for testing)
cd generator && make debug

# Run tests
cd generator && cargo test

# Check formatting
cd generator && cargo fmt --check

# Run linter (strict mode)
cd generator && cargo clippy -- -D warnings

# Build release binary
cd generator && cargo build --release
```

## Code Architecture

### Generator Structure

**Main Components:**
1. `generator/src/main.rs` - CLI entry point, file writers
2. `generator/src/highlight.rs` - All highlight group definitions (~821 lines)
3. `generator/src/conv.rs` - Color conversion utilities (hex→LAB→cterm)
4. `generator/src/lib.rs` - Module exports

### Color System

**Color Conversion Pipeline:**
```
Hex (#rrggbb) → LAB color space → 256-color terminal code
```

**Key Features:**
- Uses Delta E 2000 algorithm for perceptually accurate color matching
- 256-color palette pre-computed in `lazy_static`
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
    pub scope: HighlightScope,  // All, Nvim, Nvim080OrLater
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

The `Writer` struct generates three files:
1. `write_colorscheme()` → `colors/dogrun.vim` (main colorscheme)
2. `write_lightline()` → `autoload/lightline/colorscheme/dogrun.vim`
3. `write_clap()` → `autoload/clap/themes/dogrun.vim`

## Development Workflow

### Adding a New Plugin

1. Add highlight groups to `generator/src/highlight.rs`
2. Use the `hi!()` macro with appropriate color names
3. Run `cd generator && make build`
4. Test in Neovim/Vim with the plugin installed
5. Update plugin list in README.md

### Modifying Colors

1. Locate color definitions in `generator/src/highlight.rs`
   - Base colors defined at top (e.g., `mainbg`, `mainfg`)
   - Derived colors use `saturate()`, `darken()`, etc.
2. Modify color values or relationships
3. Run generator: `cd generator && make build`
4. Verify changes in Vim/Neovim

### Testing Changes

```bash
# 1. Generate files
cd generator && make build

# 2. Check git diff to verify expected changes
git diff colors/dogrun.vim
git diff autoload/lightline/colorscheme/dogrun.vim
git diff autoload/clap/themes/dogrun.vim

# 3. Test in Neovim/Vim
nvim -c "colorscheme dogrun"
```

## CI/CD

GitHub Actions workflow validates:
1. Code formatting (`cargo fmt`)
2. Linting with clippy (`-D warnings` strict mode)
3. Release build succeeds
4. Tests pass (`cargo test`)
5. **Generated files are up-to-date** - fails if generator produces different output than committed files

**Important:** Always run `make build` and commit generated files before pushing.

## File Organization

**Source Files (edit these):**
- `generator/src/main.rs` - Generator logic
- `generator/src/highlight.rs` - Color scheme definitions
- `generator/src/conv.rs` - Color utilities
- `generator/Cargo.toml` - Rust dependencies

**Generated Files (do not edit directly):**
- `colors/dogrun.vim` - Main colorscheme (501 lines)
- `autoload/lightline/colorscheme/dogrun.vim` - lightline theme
- `autoload/clap/themes/dogrun.vim` - vim-clap theme

**Configuration:**
- `.github/workflows/ci.yaml` - CI pipeline
- `generator/Makefile` - Build commands
- `term/dogrun.itermcolors` - iTerm2 theme

## Dependencies

**Rust Crates:**
- `clap` 2.33.0 - CLI argument parsing
- `tint` 1.0.0 - Color manipulation
- `lazy_static` 1.4.0 - Static initialization
- `delta_e` 0.2 - Perceptual color difference (CIE Delta E 2000)
- `lab` 0.7.2 - LAB color space conversions

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
