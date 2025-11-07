use dogrun::highlight::{get_highlights, get_palette};
use std::io::Cursor;

#[derive(Debug)]
struct Writer {
    palette: dogrun::highlight::Palette,
    #[allow(dead_code)]
    highlights: Vec<dogrun::highlight::Highlight>,
}

impl Writer {
    fn new(
        palette: dogrun::highlight::Palette,
        highlights: Vec<dogrun::highlight::Highlight>,
    ) -> Self {
        Self {
            palette,
            highlights,
        }
    }

    fn write_wezterm<W: std::io::Write>(&mut self, mut out: W) -> std::io::Result<()> {
        // [colors] section
        writeln!(out, "[colors]")?;
        writeln!(out, "background = \"{}\"", self.palette["mainbg"].gui)?;
        writeln!(out, "foreground = \"{}\"", self.palette["mainfg"].gui)?;
        writeln!(out, "cursor_bg = \"{}\"", self.palette["mainfg"].gui)?;
        writeln!(out, "cursor_fg = \"{}\"", self.palette["mainbg"].gui)?;
        writeln!(out, "cursor_border = \"{}\"", self.palette["mainfg"].gui)?;
        writeln!(out, "selection_bg = \"{}\"", self.palette["visualbg"].gui)?;

        // ansi array (0-7)
        writeln!(out, "ansi = [")?;
        for name in [
            "termblack",
            "termmaroon",
            "termgreen",
            "termolive",
            "termnavy",
            "termpurple",
            "termteal",
            "termsilver",
        ] {
            writeln!(out, "  \"{}\",", self.palette[name].gui)?;
        }
        writeln!(out, "]")?;

        // brights array (8-15)
        writeln!(out, "brights = [")?;
        for name in [
            "termgray",
            "termred",
            "termlime",
            "termyellow",
            "termblue",
            "termfuchsia",
            "termaqua",
            "termwhite",
        ] {
            writeln!(out, "  \"{}\",", self.palette[name].gui)?;
        }
        writeln!(out, "]")?;

        // [metadata] section
        writeln!(out)?;
        writeln!(out, "[metadata]")?;
        writeln!(out, "name = \"dogrun\"")?;
        writeln!(out, "author = \"wadackel\"")?;
        writeln!(
            out,
            "origin_url = \"https://github.com/wadackel/vim-dogrun\""
        )?;

        Ok(())
    }
}

#[test]
fn test_wezterm_toml_structure() {
    let mut writer = Writer::new(get_palette(), get_highlights());
    let mut output = Cursor::new(Vec::new());

    writer
        .write_wezterm(&mut output)
        .expect("Failed to write WezTerm colorscheme");

    let generated = String::from_utf8(output.into_inner()).expect("Invalid UTF-8");
    let parsed: toml::Value = toml::from_str(&generated).expect("Failed to parse TOML");

    // Verify structure
    let colors = parsed
        .get("colors")
        .expect("Missing [colors] section")
        .as_table()
        .expect("[colors] is not a table");

    let metadata = parsed
        .get("metadata")
        .expect("Missing [metadata] section")
        .as_table()
        .expect("[metadata] is not a table");

    // Check required color fields
    assert!(colors.contains_key("background"), "Missing background");
    assert!(colors.contains_key("foreground"), "Missing foreground");
    assert!(colors.contains_key("cursor_bg"), "Missing cursor_bg");
    assert!(colors.contains_key("cursor_fg"), "Missing cursor_fg");
    assert!(
        colors.contains_key("cursor_border"),
        "Missing cursor_border"
    );
    assert!(colors.contains_key("selection_bg"), "Missing selection_bg");

    // Check ANSI array
    let ansi = colors
        .get("ansi")
        .expect("Missing ansi array")
        .as_array()
        .expect("ansi is not an array");
    assert_eq!(ansi.len(), 8, "ansi array should have 8 colors");

    // Check brights array
    let brights = colors
        .get("brights")
        .expect("Missing brights array")
        .as_array()
        .expect("brights is not an array");
    assert_eq!(brights.len(), 8, "brights array should have 8 colors");

    // Verify hex format for all colors
    let hex_regex = regex::Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();

    for key in [
        "background",
        "foreground",
        "cursor_bg",
        "cursor_fg",
        "cursor_border",
        "selection_bg",
    ] {
        let color = colors
            .get(key)
            .unwrap_or_else(|| panic!("Missing {}", key))
            .as_str()
            .unwrap_or_else(|| panic!("{} is not a string", key));
        assert!(
            hex_regex.is_match(color),
            "{} has invalid hex format: {}",
            key,
            color
        );
    }

    for color_val in ansi.iter().chain(brights.iter()) {
        let color = color_val
            .as_str()
            .unwrap_or_else(|| panic!("Color is not a string"));
        assert!(
            hex_regex.is_match(color),
            "Invalid hex format in ansi/brights: {}",
            color
        );
    }

    // Check metadata fields
    assert_eq!(
        metadata.get("name").and_then(|v| v.as_str()),
        Some("dogrun"),
        "Invalid or missing name"
    );
    assert_eq!(
        metadata.get("author").and_then(|v| v.as_str()),
        Some("wadackel"),
        "Invalid or missing author"
    );
    assert_eq!(
        metadata.get("origin_url").and_then(|v| v.as_str()),
        Some("https://github.com/wadackel/vim-dogrun"),
        "Invalid or missing origin_url"
    );
}
