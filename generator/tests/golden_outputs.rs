use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

// Byte-compares every generated output against the committed files, so any
// refactor of the generator that changes even one byte of output fails here
// with the offending file's name instead of surfacing later as a git diff.
#[test]
fn test_generated_outputs_match_committed_files() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // The generator only creates wezterm/ itself; the vim output directories
    // must exist beforehand, matching the layout of the real repository.
    fs::create_dir_all(temp_path.join("colors")).unwrap();
    fs::create_dir_all(temp_path.join("autoload/lightline/colorscheme")).unwrap();
    fs::create_dir_all(temp_path.join("autoload/clap/themes")).unwrap();

    // The README rewrite only runs when README.md exists at the output root.
    // Copying the committed README also makes the comparison below immune to
    // README edits outside the generated fzf block: both sides share them.
    fs::copy(repo_root.join("README.md"), temp_path.join("README.md")).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--dir", temp_path.to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute generator");

    assert!(
        output.status.success(),
        "Generator failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let outputs = [
        "colors/dogrun.vim",
        "autoload/lightline/colorscheme/dogrun.vim",
        "autoload/clap/themes/dogrun.vim",
        "wezterm/dogrun.toml",
        "README.md",
    ];

    for relative in outputs {
        let generated = fs::read(temp_path.join(relative))
            .unwrap_or_else(|e| panic!("missing generated {}: {}", relative, e));
        let committed = fs::read(repo_root.join(relative))
            .unwrap_or_else(|e| panic!("missing committed {}: {}", relative, e));

        assert!(
            generated == committed,
            "{} differs from the committed file — run `just build` and inspect `git diff`",
            relative
        );
    }
}
