use dogrun::highlight::{get_highlights, get_palette};
use dogrun::writer::Writer;
use std::io::Cursor;

fn render<F>(write: F) -> String
where
    F: FnOnce(&Writer, &mut Cursor<Vec<u8>>) -> std::io::Result<()>,
{
    let writer = Writer::new(get_palette(), get_highlights());
    let mut output = Cursor::new(Vec::new());
    write(&writer, &mut output).expect("writer failed");
    String::from_utf8(output.into_inner()).expect("Invalid UTF-8")
}

#[test]
fn test_colorscheme_structure() {
    let out = render(|w, o| w.write_colorscheme(o));

    assert!(
        out.starts_with("\" dogrun: Take a sweet dog with you."),
        "missing header"
    );
    assert!(out.contains("let g:colors_name = 'dogrun'"));

    // The nvim-0.8.0 block carries every Nvim080OrLater highlight (the
    // treesitter/LSP groups); its disappearance would silently drop ~45
    // groups from the shipped colorscheme.
    let nvim08_block = out
        .split(r#"if has("nvim-0.8.0")"#)
        .nth(1)
        .expect("missing nvim-0.8.0 block")
        .split("endif")
        .next()
        .unwrap();
    assert!(
        nvim08_block.lines().filter(|l| l.contains("hi ")).count() >= 40,
        "nvim-0.8.0 block lost its highlights"
    );
    assert!(nvim08_block.contains("hi @string"));

    // 16 terminal colors plus background/foreground aliases
    for i in 0..16 {
        assert!(
            out.contains(&format!("let g:terminal_color_{} = '#", i)),
            "missing terminal color {}",
            i
        );
    }
    assert!(out.contains("let g:terminal_color_background = g:terminal_color_0"));
    assert!(out.contains("let g:terminal_color_foreground = g:terminal_color_7"));

    assert!(out.contains("let g:fzf_colors"), "missing fzf block");
}

#[test]
fn test_lightline_structure() {
    let out = render(|w, o| w.write_lightline(o));

    assert!(out.contains(
        "let s:p = {'normal': {}, 'inactive': {}, 'insert': {}, 'replace': {}, 'visual': {}, 'tabline': {}}"
    ));
    let entry = regex::Regex::new(r"let s:p\.normal\.left = \[\[\['#[0-9a-f]{6}', \d+\]").unwrap();
    assert!(entry.is_match(&out), "unexpected palette entry format");
    assert!(out.contains(
        "let g:lightline#colorscheme#dogrun#palette = lightline#colorscheme#flatten(s:p)"
    ));
}

#[test]
fn test_clap_theme_structure() {
    let out = render(|w, o| w.write_clap(o));

    assert!(out.contains("let s:palette.display = {"));
    assert!(out.contains("let g:clap#themes#dogrun#palette = s:palette"));

    // Regression guard for the h! macro bug where background colors were
    // emitted as a second ctermfg / bare cterm key on the same :hi line.
    let hi_lines: Vec<&str> = out.lines().filter(|l| l.starts_with("hi ")).collect();
    assert_eq!(hi_lines.len(), 22, "expected 22 Clap* highlight lines");
    for line in hi_lines {
        assert!(line.contains("ctermbg=NONE"), "missing ctermbg: {}", line);
        assert_eq!(
            line.matches("ctermfg=").count(),
            1,
            "duplicated ctermfg: {}",
            line
        );
        assert_eq!(
            line.matches("ctermbg=").count(),
            1,
            "duplicated ctermbg: {}",
            line
        );
        assert_eq!(
            line.matches(" cterm=").count(),
            1,
            "cterm attr key should appear exactly once: {}",
            line
        );
    }
}
