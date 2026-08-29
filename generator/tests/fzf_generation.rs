use dogrun::highlight::{get_highlights, get_palette};
use dogrun::writer::Writer;
use regex::Regex;

#[test]
fn test_fzf_export_format() {
    let writer = Writer::new(get_palette(), get_highlights());
    let export = writer
        .generate_fzf_export()
        .expect("generate_fzf_export failed");

    assert!(
        export.starts_with("export FZF_DEFAULT_OPTS='--color="),
        "unexpected prefix: {}",
        export
    );
    assert!(
        export.ends_with(",gutter:-1'"),
        "should end with gutter:-1 and closing quote: {}",
        export
    );
    assert_eq!(
        export.matches("--color=").count(),
        2,
        "should have exactly two --color groups"
    );

    for key in [
        "fg:", "bg:", "hl:", "fg+:", "bg+:", "hl+:", "info:", "prompt:", "pointer:", "marker:",
        "spinner:", "header:", "border:", "gutter:",
    ] {
        assert!(
            export.contains(key),
            "missing fzf key {} in {}",
            key,
            export
        );
    }

    let entry_regex = Regex::new(r"[a-z+]+:#[0-9a-f]{6}").unwrap();
    assert_eq!(
        entry_regex.find_iter(&export).count(),
        13,
        "13 keys should map to hex colors (gutter uses -1): {}",
        export
    );
}
