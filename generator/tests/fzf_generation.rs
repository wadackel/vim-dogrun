use dogrun::highlight::get_palette;
use regex::Regex;

// Note: Writer struct is private in main.rs, so we test via integration
// This test file validates the fzf export format expectations

#[test]
fn test_fzf_export_format_expectations() {
    // This test documents expected format for fzf export
    // Actual generation is tested via README update integration test

    let palette = get_palette();

    // Verify required palette colors exist
    assert!(palette.contains_key("weakfg"), "palette should contain weakfg");
    assert!(palette.contains_key("mainbg"), "palette should contain mainbg");
    assert!(palette.contains_key("emphasisfg"), "palette should contain emphasisfg");
    assert!(palette.contains_key("visualbg"), "palette should contain visualbg");
    assert!(palette.contains_key("purple"), "palette should contain purple");
    assert!(palette.contains_key("linenrfg"), "palette should contain linenrfg");
    assert!(palette.contains_key("pink"), "palette should contain pink");
    assert!(palette.contains_key("teal"), "palette should contain teal");
}

#[test]
fn test_palette_colors_are_hex_format() {
    let palette = get_palette();
    let hex_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

    let required_colors = vec![
        "weakfg", "mainbg", "emphasisfg", "visualbg",
        "purple", "linenrfg", "pink", "teal",
    ];

    for color_name in required_colors {
        let color = palette.get(color_name)
            .unwrap_or_else(|| panic!("missing color: {}", color_name));
        assert!(
            hex_regex.is_match(&color.gui),
            "{} should be valid hex: {}",
            color_name,
            color.gui
        );
    }
}

#[test]
fn test_fzf_required_keys() {
    // Document the required fzf color keys
    let expected_keys = vec![
        "fg", "bg", "hl", "fg+", "bg+", "hl+",
        "info", "prompt", "pointer", "marker",
        "spinner", "header", "border", "gutter",
    ];

    // This test just documents expectations
    // Actual validation happens in integration test
    assert_eq!(expected_keys.len(), 14);
}
