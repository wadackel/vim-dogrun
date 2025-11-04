#![allow(clippy::deprecated_cfg_attr)]

use clap::{crate_authors, crate_name, crate_version, Arg, Command};
use dogrun::highlight::*;
use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;

fn highlight(palette: &Palette, hl: &Highlight) -> String {
    let mut args = vec![hl.name.to_string()];
    let variants = &[(&hl.fg, "guifg", "ctermfg"), (&hl.bg, "guibg", "ctermbg")];

    // fg, bg
    for (color_name, gui, cterm) in variants {
        if let Some(name) = color_name {
            if name != &"NONE" {
                let color = &palette[name];
                args.push(format!("{}={}", gui, color.gui));
                args.push(format!("{}={}", cterm, color.cterm));
            } else {
                args.push(format!("{}=NONE", gui));
                args.push(format!("{}=NONE", cterm));
            }
        }
    }

    // sp
    if let Some(name) = hl.sp {
        let color = &palette[name];
        args.push(format!("guisp={}", color.gui));
    }

    // attr
    let attr = match hl.attr {
        HighlightAttr::Nothing => "",
        HighlightAttr::Bold => "gui=bold cterm=bold",
        HighlightAttr::Italic => "gui=italic cterm=italic",
        HighlightAttr::Underline => "gui=underline cterm=underline",
        HighlightAttr::Strikethrough => "gui=strikethrough cterm=strikethrough",
        HighlightAttr::Reverse => "gui=reverse cterm=reverse",
        HighlightAttr::None => "gui=NONE cterm=NONE",
    };

    if !attr.is_empty() {
        args.push(attr.to_string());
    }

    format!("hi {}", args.join(" "))
}

#[derive(Debug)]
struct Writer {
    palette: Palette,
    highlights: Vec<Highlight>,
}

impl Writer {
    fn new(palette: Palette, highlights: Vec<Highlight>) -> Self {
        Self {
            palette,
            highlights,
        }
    }

    fn write_colorscheme<W: io::Write>(&mut self, mut out: W) -> io::Result<()> {
        // header
        write!(
            out,
            r#"" dogrun: Take a sweet dog with you.
"
" Author: wadackel
" License: MIT
"   Copyright (c) 2020 wadackel

if &background !=# 'dark'
  set background=dark
endif

if exists('g:colors_name')
  hi clear
endif

if exists('g:syntax_on')
  syntax reset
endif

let g:colors_name = 'dogrun'

"#
        )?;

        // vim & nvim
        for hl in self.highlights.iter() {
            if hl.scope == HighlightScope::All {
                writeln!(out, "{}", highlight(&self.palette, hl))?;
            }
        }

        writeln!(out, r#"if has("nvim")"#)?;

        // only nvim
        for hl in self.highlights.iter() {
            if hl.scope == HighlightScope::Nvim {
                writeln!(out, "  {}", highlight(&self.palette, hl))?;
            }
        }

        // term colors
        let termcolors = vec![
            "termblack",
            "termmaroon",
            "termgreen",
            "termolive",
            "termnavy",
            "termpurple",
            "termteal",
            "termsilver",
            "termgray",
            "termred",
            "termlime",
            "termyellow",
            "termblue",
            "termfuchsia",
            "termaqua",
            "termwhite",
        ];

        for (index, name) in termcolors.iter().enumerate() {
            let color = &self.palette[name].gui;
            writeln!(out, "  let g:terminal_color_{} = '{}'", index, color)?;
        }

        writeln!(
            out,
            "  let g:terminal_color_background = g:terminal_color_0"
        )?;

        writeln!(
            out,
            "  let g:terminal_color_foreground = g:terminal_color_7"
        )?;

        // end nvim
        writeln!(out, "endif")?;

        // only nvim x version >= 800
        writeln!(out, r#"if has("nvim-0.8.0")"#)?;
        for hl in self.highlights.iter() {
            if hl.scope == HighlightScope::Nvim080OrLater {
                writeln!(out, "  {}", highlight(&self.palette, hl))?;
            }
        }
        writeln!(out, "endif")?;

        // defx-icons palette
        let defxicons = vec![
            ("brown", &self.palette["defxiconbrown"]),
            ("aqua", &self.palette["defxiconaqua"]),
            ("blue", &self.palette["defxiconblue"]),
            ("darkBlue", &self.palette["defxicondarkblue"]),
            ("purple", &self.palette["defxiconpurple"]),
            ("lightPurple", &self.palette["defxiconlightpurple"]),
            ("red", &self.palette["defxiconred"]),
            ("beige", &self.palette["defxiconbeige"]),
            ("yellow", &self.palette["defxiconyellow"]),
            ("orange", &self.palette["defxiconorange"]),
            ("darkOrange", &self.palette["defxicondarkorange"]),
            ("pink", &self.palette["defxiconpink"]),
            ("salmon", &self.palette["defxiconsalmon"]),
            ("green", &self.palette["defxicongreen"]),
            ("lightGreen", &self.palette["defxiconlightgreen"]),
            ("white", &self.palette["defxiconwhite"]),
        ];

        writeln!(out, "let g:defx_icons_gui_colors = {{")?;
        for (name, color) in defxicons.iter() {
            writeln!(
                out,
                "  \\ '{}': '{}',",
                name,
                &color.gui[1..color.gui.len()]
            )?;
        }
        writeln!(out, "  \\ }}")?;

        writeln!(out, "let g:defx_icons_term_colors = {{")?;
        for (name, color) in defxicons.iter() {
            writeln!(out, "  \\ '{}': {},", name, color.cterm)?;
        }
        writeln!(out, "  \\ }}")?;

        // fzf.vim colors
        writeln!(
            out,
            r#"let g:fzf_colors = {{
  \ 'fg':      ['fg', 'Normal'],
  \ 'bg':      ['bg', 'Normal'],
  \ 'hl':      ['fg', 'Comment'],
  \ 'fg+':     ['fg', 'CursorLine'],
  \ 'bg+':     ['bg', 'CursorLine'],
  \ 'hl+':     ['fg', 'Statement'],
  \ 'info':    ['fg', 'Comment'],
  \ 'gutter':  ['bg', 'Normal'],
  \ 'border':  ['fg', 'Ignore'],
  \ 'prompt':  ['fg', 'Label'],
  \ 'pointer': ['fg', 'Boolean'],
  \ 'marker':  ['fg', 'Boolean'],
  \ 'spinner': ['fg', 'Title'],
  \ 'header':  ['fg', 'Comment'],
  \ }}"#
        )?;

        Ok(())
    }

    fn write_lightline<W: io::Write>(&mut self, mut out: W) -> io::Result<()> {
        // header
        write!(
            out,
            r#"" dogrun lightline theme
"
" Author: wadackel
" License: MIT
"   Copyright (c) 2020 wadackel

let s:p = {{'normal': {{}}, 'inactive': {{}}, 'insert': {{}}, 'replace': {{}}, 'visual': {{}}, 'tabline': {{}}}}

"#
        )?;

        // body
        let palette = &self.palette;

        let color = |name: &str| {
            let highlight = palette.get(name).expect("error");
            format!("['{}', {}]", highlight.gui, highlight.cterm)
        };

        macro_rules! p {
            ($target: ident, $direction: ident, $fg1: ident, $bg1: ident) => {
                writeln!(
                    out,
                    "let s:p.{}.{} = [[{}, {}]]",
                    stringify!($target),
                    stringify!($direction),
                    color(stringify!($fg1)),
                    color(stringify!($bg1))
                )?;
            };
            ($target: ident, $direction: ident, $fg1: ident, $bg1: ident, $fg2: ident, $bg2: ident) => {
                writeln!(
                    out,
                    "let s:p.{}.{} = [[{}, {}], [{}, {}]]",
                    stringify!($target),
                    stringify!($direction),
                    color(stringify!($fg1)),
                    color(stringify!($bg1)),
                    color(stringify!($fg2)),
                    color(stringify!($bg2))
                )?;
            };
        }

        #[cfg_attr(rustfmt, rustfmt_skip)]
        {
            p!(normal, left, mainbg, purple, purple, xlinegradientbg);
            p!(normal, middle, xlinefg, xlinebg);
            p!(normal, right, mainbg, purple, purple, xlinegradientbg);
            p!(normal, error, errorfg, xlinebg);
            p!(normal, warning, warningfg, xlinebg);
            p!(inactive, left, statuslinencfg, statuslinencbg, statuslinencfg, statuslinencbg);
            p!(inactive, middle, statuslinencfg, statuslinencbg);
            p!(inactive, right, statuslinencfg, statuslinencbg, statuslinencfg, statuslinencbg);
            p!(insert, left, mainbg, teal, teal, xlinegradientbg);
            p!(insert, right, mainbg, teal, teal, xlinegradientbg);
            p!(visual, left, mainbg, pink, pink, xlinegradientbg);
            p!(visual, right, mainbg, pink, pink, xlinegradientbg);
            p!(replace, left, mainbg, red, red, xlinegradientbg);
            p!(replace, right, mainbg, red, red, xlinegradientbg);
            p!(tabline, left, xlinefg, xlinebg);
            p!(tabline, tabsel, mainbg, purple);
            p!(tabline, middle, xlinefg, xlinebg);
            p!(tabline, right, xlinefg, xlinebg);
        }

        // footer
        writeln!(
            out,
            r#"
let g:lightline#colorscheme#dogrun#palette = lightline#colorscheme#flatten(s:p)"#
        )?;

        Ok(())
    }

    fn write_clap<W: io::Write>(&mut self, mut out: W) -> io::Result<()> {
        // header
        write!(
            out,
            r#"" dogrun vim-clap theme
"
" Author: wadackel
" License: MIT
"   Copyright (c) 2020 wadackel

let s:save_cpo = &cpoptions
set cpoptions&vim

let s:palette = {{}}
"#
        )?;

        // body
        let palette = &self.palette;

        macro_rules! p {
            ($target: ident, $fg: ident, $bg: ident, $attr: ident) => {
                let mut args: Vec<String> = vec![];

                if let Some(hi) = palette.get(stringify!($fg)) {
                    args.push(format!("'ctermfg': '{}', 'guifg': '{}'", hi.cterm, hi.gui,));
                }

                if let Some(hi) = palette.get(stringify!($bg)) {
                    args.push(format!("'ctermbg': '{}', 'guibg': '{}'", hi.cterm, hi.gui,));
                }

                match HighlightAttr::$attr {
                    HighlightAttr::None => args.push("'gui': 'NONE', 'cterm': 'NONE'".to_string()),
                    HighlightAttr::Bold => args.push("'gui': 'bold', 'cterm': 'bold'".to_string()),
                    HighlightAttr::Italic => {
                        args.push("'gui': 'italic', 'cterm': 'italic'".to_string())
                    }
                    HighlightAttr::Underline => {
                        args.push("'gui': 'underline', 'cterm': 'underline'".to_string())
                    }
                    HighlightAttr::Reverse => {
                        args.push("'gui': 'reverse', 'cterm': 'reverse'".to_string())
                    }
                    _ => {}
                }

                writeln!(
                    out,
                    "let s:palette.{} = {{ {} }}",
                    stringify!($target),
                    args.join(", "),
                )?;
            };
            ($target: ident, $fg: ident, -, $attr: ident) => {
                p!($target, $fg, None, $attr);
            };
        }

        macro_rules! h {
            ($name: ident, $fg: ident, $bg: ident, $attr: ident) => {
                let mut args: Vec<String> = vec![];

                if let Some(fg) = palette.get(stringify!($fg)) {
                    args.push(format!("guifg={}", fg.gui));
                    args.push(format!("ctermfg={}", fg.cterm));
                }

                if let Some(bg) = palette.get(stringify!($bg)) {
                    args.push(format!("guibg={}", bg.gui));
                    args.push(format!("ctermfg={}", bg.cterm));
                } else if stringify!($bg) == "None" {
                    args.push("guibg=NONE".to_string());
                    args.push("cterm=NONE".to_string());
                }

                match HighlightAttr::$attr {
                    HighlightAttr::None => args.push("gui=NONE cterm=NONE".to_string()),
                    HighlightAttr::Bold => args.push("gui=bold cterm=bold".to_string()),
                    HighlightAttr::Italic => args.push("gui=italic cterm=italic".to_string()),
                    HighlightAttr::Underline => {
                        args.push("gui=underline cterm=underline".to_string())
                    }
                    HighlightAttr::Reverse => args.push("gui=reverse cterm=reverse".to_string()),
                    _ => {}
                }

                writeln!(out, "hi {} {}", stringify!($name), args.join(" "),)?;
            };
            ($target: ident, $fg: ident, -, $attr: ident) => {
                h!($target, $fg, None, $attr);
            };
        }

        #[cfg_attr(rustfmt, rustfmt_skip)]
        {
            p!(input, purple, pmenubar, Bold);
            p!(display, pmenufg, pmenubg, None);
            p!(spinner, purple, pmenubar, Bold);
            p!(search_text, mainfg, pmenubar, None);
            p!(preview, pmenuselfg, pmenuselbg, None);
            p!(selected, cyan, -, Bold);
            p!(current_selection, emphasisfg, -, Bold);
        }

        #[cfg_attr(rustfmt, rustfmt_skip)]
        {
            h!(ClapMatches, teal, -, Bold);
            h!(ClapMatches1, teal, -, Bold);
            h!(ClapMatches2, teal, -, Bold);
            h!(ClapMatches3, teal, -, Bold);
            h!(ClapMatches4, teal, -, Bold);
            h!(ClapMatches5, teal, -, Bold);
            h!(ClapMatches6, teal, -, Bold);
            h!(ClapMatches7, teal, -, Bold);
            h!(ClapMatches8, teal, -, Bold);
            h!(ClapFuzzyMatches1, teal, -, Bold);
            h!(ClapFuzzyMatches2, teal, -, Bold);
            h!(ClapFuzzyMatches3, teal, -, Bold);
            h!(ClapFuzzyMatches4, teal, -, Bold);
            h!(ClapFuzzyMatches5, teal, -, Bold);
            h!(ClapFuzzyMatches6, teal, -, Bold);
            h!(ClapFuzzyMatches7, teal, -, Bold);
            h!(ClapFuzzyMatches8, teal, -, Bold);
            h!(ClapFuzzyMatches9, teal, -, Bold);
            h!(ClapFuzzyMatches10, teal, -, Bold);
            h!(ClapFuzzyMatches11, teal, -, Bold);
            h!(ClapFuzzyMatches12, teal, -, Bold);
            h!(ClapNoMatchesFound, warningfg, -, Bold);
        }

        // footer
        writeln!(
            out,
            r#"let g:clap#themes#dogrun#palette = s:palette

let &cpoptions = s:save_cpo
unlet s:save_cpo
"#
        )?;

        Ok(())
    }
}

fn abs(path: PathBuf) -> io::Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()?.join(path))
    }
}

fn main() -> io::Result<()> {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::new("dir")
                .help("Output directory path")
                .short('d')
                .long("dir"),
        )
        .get_matches();

    match matches.get_one::<String>("dir") {
        Some(dir) => {
            let dir = abs(PathBuf::from(dir))?;
            let mut writer = Writer::new(get_palette(), get_highlights());

            let path = File::create(dir.join("colors/dogrun.vim"))?;
            writer.write_colorscheme(io::BufWriter::new(path))?;

            let path = File::create(dir.join("autoload/lightline/colorscheme/dogrun.vim"))?;
            writer.write_lightline(io::BufWriter::new(path))?;

            let path = File::create(dir.join("autoload/clap/themes/dogrun.vim"))?;
            writer.write_clap(io::BufWriter::new(path))?;
        }
        None => {
            let mut writer = Writer::new(get_palette(), get_highlights());
            writer.write_colorscheme(io::stdout())?;
            writer.write_lightline(io::stdout())?;
            writer.write_clap(io::stdout())?;
        }
    };

    Ok(())
}
