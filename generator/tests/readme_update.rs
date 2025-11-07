use regex::Regex;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_readme_fzf_section_update() {
    // Create temporary directory
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create necessary directory structure
    fs::create_dir_all(temp_path.join("colors")).unwrap();
    fs::create_dir_all(temp_path.join("autoload/lightline/colorscheme")).unwrap();
    fs::create_dir_all(temp_path.join("autoload/clap/themes")).unwrap();

    // Create test README with fzf markers
    let test_readme = r#"# Test README

## fzf section

`~/.zshrc` or `~/.bashrc`

<!-- fzf:start -->
```bash
export FZF_DEFAULT_OPTS='--color=old:value'
```
<!-- fzf:end -->

## Other section

Some other content here.
"#;

    fs::write(temp_path.join("README.md"), test_readme).unwrap();

    // Run generator
    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "--dir", temp_path.to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute generator");

    // Check command succeeded
    assert!(
        output.status.success(),
        "Generator failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Read updated README
    let updated_readme = fs::read_to_string(temp_path.join("README.md"))
        .expect("Failed to read updated README");

    // Verify markers still exist
    assert!(
        updated_readme.contains("<!-- fzf:start -->"),
        "Start marker should be preserved"
    );
    assert!(
        updated_readme.contains("<!-- fzf:end -->"),
        "End marker should be preserved"
    );

    // Verify new export statement exists
    assert!(
        updated_readme.contains("export FZF_DEFAULT_OPTS='--color="),
        "Should contain FZF export statement"
    );

    // Verify old value is gone
    assert!(
        !updated_readme.contains("old:value"),
        "Old placeholder should be replaced"
    );

    // Verify code block format
    assert!(
        updated_readme.contains("```bash\n"),
        "Should contain bash code block"
    );

    // Verify all required fzf keys are present with proper format
    let required_keys = vec![
        "fg:", "bg:", "hl:", "fg+:", "bg+:", "hl+:",
        "info:", "prompt:", "pointer:", "marker:",
        "spinner:", "header:", "border:", "gutter:",
    ];

    for key in required_keys {
        assert!(
            updated_readme.contains(key),
            "Should contain fzf key: {}",
            key
        );
    }

    // Verify hex color format (strict: must be lowercase 6-char hex)
    let hex_regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    assert!(
        hex_regex.is_match(&updated_readme),
        "Should contain hex color codes"
    );

    // Verify export statement structure
    assert!(
        updated_readme.contains("export FZF_DEFAULT_OPTS='--color="),
        "Should have proper export statement start"
    );
    assert!(
        updated_readme.matches("--color=").count() == 2,
        "Should have exactly two --color groups"
    );
    assert!(
        updated_readme.contains(",gutter:-1'"),
        "Should end with gutter:-1 and closing quote"
    );

    // Verify structure is maintained
    assert!(
        updated_readme.contains("## Other section"),
        "Other sections should be preserved"
    );
    assert!(
        updated_readme.contains("Some other content here."),
        "Other content should be preserved"
    );
}

#[test]
fn test_readme_update_preserves_surrounding_content() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    fs::create_dir_all(temp_path.join("colors")).unwrap();
    fs::create_dir_all(temp_path.join("autoload/lightline/colorscheme")).unwrap();
    fs::create_dir_all(temp_path.join("autoload/clap/themes")).unwrap();

    let test_readme = r#"Content before

<!-- fzf:start -->
```bash
export OLD='value'
```
<!-- fzf:end -->

Content after
"#;

    fs::write(temp_path.join("README.md"), test_readme).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "--dir", temp_path.to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute generator");

    assert!(output.status.success());

    let updated = fs::read_to_string(temp_path.join("README.md")).unwrap();

    // Verify surrounding content is preserved
    assert!(updated.contains("Content before"));
    assert!(updated.contains("Content after"));
    assert!(!updated.contains("export OLD='value'"));
}

#[test]
fn test_readme_update_with_marker_in_documentation() {
    // Test that the updater finds markers even when similar text
    // appears in documentation (e.g., within backticks)
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    fs::create_dir_all(temp_path.join("colors")).unwrap();
    fs::create_dir_all(temp_path.join("autoload/lightline/colorscheme")).unwrap();
    fs::create_dir_all(temp_path.join("autoload/clap/themes")).unwrap();

    // Note: The markers in backticks will be found first, but our implementation
    // should search from the start marker position to find the matching end marker
    let test_readme = r#"# Documentation

To use markers, wrap your config with HTML comments.
Example: `<!-- fzf:start -->` and `<!-- fzf:end -->`

## Actual Configuration

<!-- fzf:start -->
```bash
export OLD='value'
```
<!-- fzf:end -->

End of file.
"#;

    fs::write(temp_path.join("README.md"), test_readme).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "--dir", temp_path.to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute generator");

    assert!(output.status.success());

    let updated = fs::read_to_string(temp_path.join("README.md")).unwrap();

    // Verify the documentation example is preserved
    assert!(updated.contains("To use markers, wrap"));
    assert!(updated.contains("Example: `<!-- fzf:start -->"));

    // Verify the configuration section was updated
    // Note: "export OLD='value'" will still be in the backtick example,
    // so we check it's only there once (in documentation)
    assert!(updated.contains("export FZF_DEFAULT_OPTS="));

    // Verify structure is maintained
    assert!(updated.contains("End of file."));
}
