use super::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub gui: String,
    pub cterm: String,
}

pub type ColorName = Option<&'static str>;

pub type Palette = HashMap<&'static str, Color>;

#[derive(Debug)]
pub enum HighlightAttr {
    Nothing,
    None,
    Bold,
    Italic,
    Underline,
    Reverse,
}

#[derive(Debug)]
pub struct Highlight {
    pub name: &'static str,
    pub fg: ColorName,
    pub bg: ColorName,
    pub sp: ColorName,
    pub attr: HighlightAttr,
}

macro_rules! highlight {
    ($name: ident, $fg: expr, $bg: expr, $sp: expr, $attr: ident) => {
        Highlight {
            name: stringify!($name),
            fg: $fg,
            bg: $bg,
            sp: $sp,
            attr: HighlightAttr::$attr,
        }
    };
}

macro_rules! hi {
    ($name: ident, -, -, -, -) => {
        highlight!($name, None, None, None, Nothing)
    };
    ($name: ident, $fg: ident, -, -, -) => {
        highlight!($name, Some(stringify!($fg)), None, None, Nothing)
    };
    ($name: ident, -, $bg: ident, -, -) => {
        highlight!($name, None, Some(stringify!($bg)), None, Nothing)
    };
    ($name: ident, -, -, $sp: ident, -) => {
        highlight!($name, None, None, Some(stringify!($sp)), Nothing)
    };
    ($name: ident, -, -, -, $attr: ident) => {
        highlight!($name, None, None, None, $attr)
    };
    ($name: ident, $fg: ident, $bg: ident, -, -) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            Nothing
        )
    };
    ($name: ident, $fg: ident, -, -, $attr: ident) => {
        highlight!($name, Some(stringify!($fg)), None, None, $attr)
    };
    ($name: ident, -, $bg: ident, -, $attr: ident) => {
        highlight!($name, None, Some(stringify!($bg)), None, $attr)
    };
    ($name: ident, $fg: ident, $bg: ident, -, -) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            Nothing
        )
    };
    ($name: ident, $fg: ident, $bg: ident, -, $attr: ident) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            $attr
        )
    };
}

pub fn get_palette() -> Palette {
    let mut p = HashMap::new();

    macro_rules! def {
        ($name: ident, $hex: expr) => {
            assert_eq!(
                p.insert(
                    stringify!($name),
                    Color {
                        gui: String::from($hex),
                        cterm: conv::to_cterm($hex.to_string()).to_string(),
                    }
                ),
                None
            );
        };
    }

    macro_rules! extends {
        ($parent: ident) => {
            match p.get(stringify!($parent)) {
                Some(highlight) => highlight.gui.to_string(),
                None => panic!(format!("\"{}\" does not exists", stringify!($parent))),
            }
        };
        ($parent: ident, $h: expr, $s: expr, $v: expr) => {
            conv::hue(conv::saturate(conv::lighten(extends!($parent), $v), $s), $h)
        };
    }

    // palettes
    def!(red, "#f56574");
    def!(darkred, extends!(red, 0.0, 0.0, -0.2));
    def!(pink, "#c173c1");
    def!(darkpink, extends!(pink, 0.0, -0.05, -0.35));
    def!(purple, "#929be5");
    def!(darkpurple, extends!(purple, 0.0, 0.05, -0.15));
    def!(blue, "#31a9e0");
    def!(darkblue, extends!(blue, 0.0, 0.0, -0.2));
    def!(darkestblue, extends!(blue, 0.0, 0.05, -0.48));
    def!(cyan, "#2aacbd");
    def!(darkcyan, extends!(cyan, 0.0, 0.0, -0.2));
    def!(teal, "#73c1a9");
    def!(darkteal, extends!(teal, 0.0, 0.0, -0.2));
    def!(darkestteal, extends!(teal, 0.0, 0.05, -0.48));
    def!(green, "#7cbe8c");
    def!(darkgreen, extends!(green, 0.0, -0.05, -0.4));
    def!(yellow, "#b5ae7d");
    def!(darkyellow, extends!(yellow, 0.0, -0.15, -0.5));
    def!(orange, "#c2968c");
    def!(darkorange, extends!(orange, 0.0, -0.05, -0.2));

    // neutral
    def!(mainfg, "#9ea3c0");
    def!(mainbg, "#222433");
    def!(weakfg, extends!(mainbg, 0.0, 0.05, 0.35));
    def!(weakbg, extends!(mainbg, 0.0, 0.0, 0.1));
    def!(emphasisfg, extends!(mainfg, 0.0, 0.0, 0.15));
    def!(emphasisbg, extends!(mainbg, 0.0, 0.0, 0.05));
    def!(darkfg, extends!(mainbg, 0.0, 0.05, 0.15));
    def!(darkbg, extends!(mainbg, 0.0, 0.0, 0.05));
    def!(lightfg, extends!(mainfg, 0.0, 0.05, -0.1));
    def!(lightbg, extends!(mainbg, 0.0, 0.0, 0.2));
    def!(white, "#ffffff");
    def!(black, "#000000");

    // messages
    def!(morefg, extends!(teal));
    def!(errorbg, extends!(mainbg));
    def!(errorfg, extends!(red, 0.0, 0.0, 0.0));
    def!(warningbg, extends!(mainbg));
    def!(warningfg, extends!(orange, 0.0, 0.0, 0.0));
    def!(infobg, extends!(mainbg));
    def!(infofg, extends!(teal, 0.0, 0.0, 0.1));

    // visual
    def!(visualbg, extends!(purple, 0.0, 0.2, -0.4));

    // linenr
    def!(linenrfg, extends!(mainbg, 0.0, 0.0, 0.1));
    def!(cursorlinebg, extends!(mainbg, 0.0, 0.0, 0.05));
    def!(cursorlinenrfg, extends!(linenrfg, 0.0, 0.1, 0.3));

    // pmenu
    def!(pmenubg, extends!(mainbg, 0.0, 0.0, 0.1));
    def!(pmenufg, extends!(mainfg, 0.0, 0.0, 0.0));
    def!(pmenuselbg, extends!(pmenubg, 0.0, 0.0, 0.1));
    def!(pmenuselfg, extends!(mainfg, 0.0, 0.0, 0.0));
    def!(pmenubar, extends!(pmenubg, 0.0, 0.0, -0.05));
    def!(pmenuthumb, extends!(pmenubg, 0.0, 0.1, 0.2));

    // fold
    def!(foldbg, extends!(mainbg, 0.0, 0.0, 0.1));
    def!(foldfg, extends!(mainbg, 0.0, 0.0, 0.4));

    // diff
    def!(diffaddbg, extends!(darkgreen));
    def!(diffaddfg, extends!(mainfg, 0.0, 0.0, 0.2));
    def!(diffchangebg, extends!(darkyellow));
    def!(diffchangefg, extends!(mainfg, 0.0, 0.0, 0.2));
    def!(diffdeletebg, extends!(darkpink));
    def!(diffdeletefg, extends!(mainfg, 0.0, 0.0, 0.25));
    def!(difftextbg, extends!(diffchangebg, 0.0, 0.2, 0.2));
    def!(difftextfg, extends!(mainfg, 0.0, 0.0, 0.2));

    // status & tab line
    def!(statuslinebg, extends!(mainbg, 0.0, 0.0, 0.05));
    def!(statuslinefg, extends!(statuslinebg, 0.0, -0.05, 0.4));
    def!(statuslinencbg, extends!(mainbg, 0.0, -0.03, 0.03));
    def!(statuslinencfg, extends!(statuslinencbg, 0.0, 0.0, 0.2));
    def!(tablineselbg, extends!(purple));
    def!(tablineselfg, extends!(mainbg));

    // misc
    def!(searchbg, extends!(purple, 0.0, 0.2, -0.05));
    def!(searchfg, extends!(searchbg, 0.1, -0.1, -0.3));
    def!(matchparenbg, extends!(mainbg, 0.1, 0.0, 0.08));

    // terminal colors
    def!(termblack, extends!(mainbg, 0.0, 0.0, -0.1));
    def!(termmaroon, extends!(red, 0.0, 0.0, -0.1));
    def!(termgreen, extends!(green, 0.0, 0.0, 0.0));
    def!(termolive, extends!(yellow, 0.0, 0.0, -0.1));
    def!(termnavy, extends!(blue, 0.0, 0.0, -0.1));
    def!(termpurple, extends!(purple, 0.0, 0.1, -0.1));
    def!(termteal, extends!(teal));
    def!(termsilver, extends!(mainfg));
    def!(termgray, extends!(weakfg));
    def!(termred, extends!(pink));
    def!(termlime, extends!(green));
    def!(termyellow, extends!(yellow));
    def!(termblue, extends!(blue));
    def!(termfuchsia, extends!(purple));
    def!(termaqua, extends!(cyan));
    def!(termwhite, extends!(mainfg));

    // defx-icons
    def!(defxiconbrown, extends!(red, 0.0, 0.2, -0.2));
    def!(defxiconaqua, extends!(cyan, 0.0, -0.1, -0.1));
    def!(defxiconblue, extends!(blue, 0.0, -0.1, -0.1));
    def!(defxicondarkblue, extends!(blue, 0.0, -0.2, -0.25));
    def!(defxiconpurple, extends!(darkpurple));
    def!(defxiconlightpurple, extends!(purple, 0.0, -0.1, -0.1));
    def!(defxiconred, extends!(red, 0.0, -0.0, -0.1));
    def!(defxiconbeige, extends!(yellow, 0.0, -0.2, -0.25));
    def!(defxiconyellow, extends!(yellow, 0.0, 0.0, -0.1));
    def!(defxiconorange, extends!(orange, 0.0, 0.0, 0.1));
    def!(defxicondarkorange, extends!(orange, 0.0, 0.1, -0.2));
    def!(defxiconpink, extends!(pink, 0.0, 0.0, -0.1));
    def!(defxiconsalmon, extends!(pink, 0.0, 0.1, -0.05));
    def!(defxicongreen, extends!(green, 0.0, 0.0, -0.15));
    def!(defxiconlightgreen, extends!(green, 0.0, 0.1, -0.1));
    def!(defxiconwhite, extends!(mainfg, 0.0, 0.0, -0.1));

    // lightline
    def!(xlinebg, extends!(statuslinencbg));
    def!(xlinefg, extends!(statuslinencfg));
    def!(xlineedgebg, extends!(statuslinebg));
    def!(xlineedgefg, extends!(statuslinefg));
    def!(xlinegradientbg, extends!(statuslinencbg));
    def!(xlinegradientfg, extends!(statuslinencfg));

    return p;
}

pub fn get_highlights() -> Vec<Highlight> {
    return vec![
        // general
        hi!(Normal, mainfg, mainbg, -, -),
        hi!(Delimiter, lightfg, -, -, -),
        hi!(NonText, darkfg, NONE, -, -),
        hi!(VertSplit, weakbg, NONE, -, None),
        hi!(LineNr, linenrfg, NONE, -, None),
        hi!(EndOfBuffer, darkfg, NONE, -, None),
        hi!(Comment, weakfg, -, -, None),
        hi!(Cursor, mainbg, mainfg, -, -),
        hi!(CursorIM, mainbg, mainfg, -, -),
        hi!(SignColumn, weakfg, NONE, -, -),
        hi!(ColorColumn, -, cursorlinebg, -, None),
        hi!(CursorColumn, -, cursorlinebg, -, None),
        hi!(CursorLine, -, cursorlinebg, -, None),
        hi!(CursorLineNr, cursorlinenrfg, NONE, -, None),
        hi!(Conceal, orange, mainbg, -, None),
        hi!(NormalFloat, mainfg, pmenubg, -, None),
        hi!(Folded, foldfg, foldbg, -, None),
        hi!(FoldColumn, linenrfg, NONE, -, None),
        hi!(MatchParen , -, matchparenbg, -, -),
        hi!(Directory , yellow, -, -, -),
        hi!(Underlined , -, -, -, Underline),
        hi!(String, green, -, -, -),
        hi!(Statement, purple, -, -, None),
        hi!(Label, purple, -, -, None),
        hi!(Function, purple, -, -, None),
        hi!(Constant, teal, -, -, -),
        hi!(Boolean, teal, -, -, -),
        hi!(Number, teal, -, -, -),
        hi!(Float, teal, -, -, -),
        hi!(Title, yellow, -, -, Bold),
        hi!(Keyword, orange, -, -, -),
        hi!(Identifier, orange, -, -, -),
        hi!(Exception, yellow, -, -, -),
        hi!(Type, yellow, -, -, None),
        hi!(TypeDef, yellow, -, -, None),
        hi!(PreProc, purple, -, -, -),
        hi!(Special, pink, -, -, -),
        hi!(SpecialKey, pink, -, -, -),
        hi!(SpecialChar, pink, -, -, -),
        hi!(SpecialComment, pink, -, -, -),
        hi!(Error, errorfg, errorbg, -, Bold),
        hi!(ErrorMsg, errorfg, NONE, -, Bold),
        hi!(WarningMsg, warningfg, -, -, Bold),
        hi!(MoreMsg, morefg, -, -, -),
        hi!(Todo, yellow, NONE, -, Bold),
        hi!(Pmenu, pmenufg, pmenubg, -, -),
        hi!(PmenuSel, pmenuselfg, pmenuselbg, -, -),
        hi!(PmenuSbar, -, pmenubar, -, -),
        hi!(PmenuThumb, -, pmenuthumb, -, -),
        hi!(Visual, -, visualbg, -, None),
        hi!(Search, searchfg, searchbg, -, -),
        hi!(IncSearch, searchfg, searchbg, -, -),
        hi!(Question, teal, -, -, Bold),
        hi!(WildMenu, mainbg, purple, -, -),
        hi!(SpellBad, errorfg, -, -, Underline),
        hi!(SpellCap, -, -, -, Underline),
        hi!(SpellLocal, errorfg, -, -, Underline),
        hi!(SpellRare, yellow, -, -, Underline),
        hi!(DiffAdd, -, diffaddbg, -, Bold),
        hi!(DiffChange, -, diffchangebg, -, Bold),
        hi!(DiffDelete, diffdeletefg, diffdeletebg, -, Bold),
        hi!(DiffText, -, difftextbg, -, None),
        hi!(QuickFixLine, mainfg, visualbg, -, -),
        hi!(StatusLine, statuslinefg, statuslinebg, -, Bold),
        hi!(StatusLineTerm, statuslinefg, statuslinebg, -, Bold),
        hi!(StatusLineNC, statuslinencfg, statuslinencbg, -, None),
        hi!(StatusLineTermNC, statuslinencfg, statuslinencbg, -, None),
        hi!(TabLine, statuslinefg, statuslinebg, -, None),
        hi!(TabLineFill, statuslinefg, statuslinebg, -, None),
        hi!(TabLineSel, tablineselfg, tablineselbg, -, Bold),
        hi!(qfFileName, teal, -, -, -),
        hi!(qfLineNr, weakfg, -, -, -),
        // html
        hi!(htmlTag, lightfg, -, -, -),
        hi!(htmlEndTag, lightfg, -, -, -),
        hi!(htmlSpecialTagName, orange, -, -, -),
        hi!(htmlArg, lightfg, -, -, -),
        // yaml
        hi!(yamlBlockMappingKey, purple, -, -, -),
        hi!(yamlAnchor, pink, -, -, -),
        // python
        hi!(pythonStatement, orange, -, -, -),
        hi!(pythonBuiltin, cyan, -, -, -),
        hi!(pythonRepeat, orange, -, -, -),
        hi!(pythonOperator, orange, -, -, -),
        hi!(pythonDecorator, pink, -, -, -),
        hi!(pythonDecoratorName, pink, -, -, -),
        // zsh
        hi!(zshVariableDef, purple, -, -, -),
        hi!(zshFunction, purple, -, -, -),
        hi!(zshKSHFunction, purple, -, -, -),
        // C
        hi!(cPreCondit, orange, -, -, -),
        hi!(cIncluded, pink, -, -, -),
        hi!(cStorageClass, orange, -, -, -),
        // C++
        // octol/vim-cpp-enhanced-highlight
        hi!(cppStructure, pink, -, -, -),
        hi!(cppSTLnamespace, orange, -, -, -),
        // C#
        hi!(csStorage, orange, -, -, -),
        hi!(csModifier, purple, -, -, -),
        hi!(csClass, purple, -, -, -),
        hi!(csClassType, pink, -, -, -),
        hi!(csNewType, orange, -, -, -),
        // vim-markdown
        // https://github.com/plasticboy/vim-markdown
        hi!(mkdHeading, weakfg, -, -, -),
        hi!(mkdLink, purple, -, -, -),
        hi!(mkdCode, purple, -, -, -),
        hi!(mkdCodeStart , purple, -, -, -),
        hi!(mkdCodeEnd, purple, -, -, -),
        hi!(mkdCodeDelimiter, purple, -, -, -),
        // yats.vim
        // https://github.com/HerringtonDarkholme/yats.vim
        hi!(typescriptImport, purple, -, -, -),
        // vim-markdown
        // https://github.com/plasticboy/vim-markdown
        hi!(mkdHeading, weakfg, -, -, -),
        hi!(mkdLink, purple, -, -, -),
        hi!(mkdCode, purple, -, -, -),
        hi!(mkdCodeStart , purple, -, -, -),
        hi!(mkdCodeEnd, purple, -, -, -),
        hi!(mkdCodeDelimiter, purple, -, -, -),
        // vim-toml
        // https://github.com/cespare/vim-toml
        hi!(tomlTable, purple, -, -, -),
        // rust.vim
        // https://github.com/rust-lang/rust.vim
        hi!(rustModPath, purple, -, -, -),
        hi!(rustTypedef, purple, -, -, -),
        hi!(rustStructure, purple, -, -, -),
        hi!(rustMacro, purple, -, -, -),
        hi!(rustExternCrate, purple, -, -, -),
        // vimfiler
        // https://github.com/Shougo/vimfiler.vim
        hi!(vimfilerOpenedFile, darkpurple, -, -, -),
        hi!(vimfilerClosedFile, darkpurple, -, -, -),
        hi!(vimfilerNonMark, teal, -, -, -),
        hi!(vimfilerLeaf, teal, -, -, -),
        // defx-icons
        // https://github.com/kristijanhusak/defx-icons
        hi!(DefxIconsMarkIcon, darkpurple, -, -, None),
        hi!(DefxIconsDirectory, darkpurple, -, -, None),
        hi!(DefxIconsParentDirectory, darkpurple, -, -, None),
        hi!(DefxIconsSymlinkDirectory, teal, -, -, None),
        hi!(DefxIconsOpenedTreeIcon, darkpurple, -, -, None),
        hi!(DefxIconsNestedTreeIcon, darkpurple, -, -, None),
        hi!(DefxIconsClosedTreeIcon, darkpurple, -, -, None),
        // defx-git
        // https://github.com/kristijanhusak/defx-git
        hi!(Defx_git_Untracked, purple, -, -, None),
        hi!(Defx_git_Ignored, weakfg, -, -, None),
        hi!(Defx_git_Unknown, weakfg, -, -, None),
        hi!(Defx_git_Renamed, diffchangebg, -, -, -),
        hi!(Defx_git_Modified, diffchangebg, -, -, -),
        hi!(Defx_git_Unmerged, pink, -, -, -),
        hi!(Defx_git_Deleted, diffdeletebg, -, -, -),
        hi!(Defx_git_Staged, teal, -, -, -),
        // vim-gitgutter
        // https://github.com/airblade/vim-gitgutter
        hi!(GitGutterAdd, green, -, -, -),
        hi!(GitGutterChange, yellow, -, -, -),
        hi!(GitGutterDelete, pink, -, -, -),
        hi!(GitGutterChangeDelete, difftextbg, -, -, -),
        // ale
        // https://github.com/dense-analysis/ale
        hi!(ALEWarningSign, warningfg, -, -, Bold),
        hi!(ALEInfoSign, infofg, -, -, None),
        // coc.nvim
        // https://github.com/neoclide/coc.nvim
        hi!(CocErrorSign, errorfg, -, -, Bold),
        hi!(CocWarningSign, warningfg, -, -, Bold),
        hi!(CocInfoSign, infofg, -, -, Bold),
        hi!(CocHintSign, infofg, -, -, Bold),
        // clever-f.vim
        // https://github.com/rhysd/clever-f.vim
        hi!(CleverFChar, searchfg, searchbg, -, Underline),
        // conflict-marker.vim
        // https://github.com/rhysd/conflict-marker.vim
        hi!(ConflictMarkerBegin, -, darkteal, -, Bold),
        hi!(ConflictMarkerOurs, -, darkestteal, -, None),
        hi!(ConflictMarkerTheirs, -, darkestblue, -, None),
        hi!(ConflictMarkerEnd, -, darkblue, -, Bold),
        hi!(ConflictMarkerSeparator, darkfg, -, -, Bold),
        // easymotion
        // https://github.com/easymotion/vim-easymotion
        hi!(EasyMotionTarget, yellow, -, -, Bold),
        hi!(EasyMotionShade, weakfg, mainbg, -, -),
        hi!(EasyMotionIncCursor, mainfg, mainbg, -, -),
    ];
}
