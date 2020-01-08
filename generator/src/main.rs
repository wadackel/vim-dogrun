#[macro_use]
extern crate clap;
extern crate dogrun;

use clap::{App, Arg};
use dogrun::highlight::*;
use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;

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

        // body
        for hl in self.highlights.iter() {
            let mut args = vec![hl.name.to_string()];

            let variants = &[(&hl.fg, "guifg", "ctermfg"), (&hl.bg, "guibg", "ctermbg")];

            // fg, bg
            for (color_name, gui, cterm) in variants {
                if let Some(name) = color_name {
                    if name != &"NONE" {
                        let color = &self.palette[name];
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
                let color = &self.palette[name];
                args.push(format!("guisp={}", color.gui));
            }

            // attr
            let attr = match hl.attr {
                HighlightAttr::Nothing => "",
                HighlightAttr::Bold => "gui=bold cterm=bold",
                HighlightAttr::Italic => "gui=italic cterm=italic",
                HighlightAttr::Underline => "gui=underline cterm=underline",
                HighlightAttr::Reverse => "gui=reverse cterm=reverse",
                HighlightAttr::None => "gui=NONE cterm=NONE",
            };

            if attr != "" {
                args.push(attr.to_string());
            }

            writeln!(out, "hi {}", args.join(" "))?;
        }

        // term colors
        writeln!(out, r#"if has("nvim")"#)?;

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
            r#"let g:terminal_color_background = g:terminal_color_0
  let g:terminal_color_foreground = g:terminal_color_7
endif"#
        )?;

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
        writeln!(out, "  \\}}")?;

        writeln!(out, "let g:defx_icons_term_colors = {{")?;
        for (name, color) in defxicons.iter() {
            writeln!(out, "  \\ '{}': {},", name, color.cterm)?;
        }
        writeln!(out, "  \\}}")?;

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
"   Copyright (c) 2019 wadackel

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
            p!(normal, error, errorbg, errorfg);
            p!(normal, warning, warningbg, warningfg);
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
}

fn abs(path: PathBuf) -> io::Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()?.join(path))
    }
}

fn main() -> io::Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("dir")
                .help("Output directory path")
                .short("d")
                .long("dir")
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("dir") {
        Some(dir) => {
            let dir = abs(PathBuf::from(dir))?;
            let mut writer = Writer::new(get_palette(), get_highlights());

            let colorscheme = File::create(dir.join("colors/dogrun.vim"))?;
            writer.write_colorscheme(io::BufWriter::new(colorscheme))?;

            let lightline = File::create(dir.join("autoload/lightline/colorscheme/dogrun.vim"))?;
            writer.write_lightline(io::BufWriter::new(lightline))?;
        }
        None => {
            let mut writer = Writer::new(get_palette(), get_highlights());
            writer.write_colorscheme(io::stdout())?;
            writer.write_lightline(io::stdout())?;
        }
    };

    Ok(())
}
