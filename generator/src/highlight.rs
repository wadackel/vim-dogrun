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

#[derive(Debug, PartialEq)]
pub enum HighlightScope {
    All,
    Nvim,
    Nvim080OrLater,
}

#[derive(Debug)]
pub struct Highlight {
    pub name: &'static str,
    pub fg: ColorName,
    pub bg: ColorName,
    pub sp: ColorName,
    pub attr: HighlightAttr,
    pub scope: HighlightScope,
}

macro_rules! highlight {
    ($name: literal, $fg: expr, $bg: expr, $sp: expr, $attr: ident, $scope: ident) => {
        Highlight {
            name: $name,
            fg: $fg,
            bg: $bg,
            sp: $sp,
            attr: HighlightAttr::$attr,
            scope: HighlightScope::$scope,
        }
    };
}

macro_rules! hi {
    ($name: literal, -, -, -, -, -) => {
        highlight!($name, None, None, None, Nothing, All)
    };
    ($name: literal, $fg: ident, -, -, -, -) => {
        highlight!($name, Some(stringify!($fg)), None, None, Nothing, All)
    };
    ($name: literal, -, $bg: ident, -, -, -) => {
        highlight!($name, None, Some(stringify!($bg)), None, Nothing, All)
    };
    ($name: literal, -, -, $sp: ident, -, -) => {
        highlight!($name, None, None, Some(stringify!($sp)), Nothing, All)
    };
    ($name: literal, -, -, -, $attr: ident, -) => {
        highlight!($name, None, None, None, $attr, All)
    };
    ($name: literal, -, -, -, -, $scope: expr) => {
        highlight!($name, None, None, None, $attr, $scope)
    };
    ($name: literal, $fg: ident, $bg: ident, -, -, -) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            Nothing,
            All
        )
    };
    ($name: literal, $fg: ident, $bg: ident, -, -, $scope: ident) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            Nothing,
            $scope
        )
    };
    ($name: literal, $fg: ident, -, -, -, $scope: ident) => {
        highlight!($name, Some(stringify!($fg)), None, None, Nothing, $scope)
    };
    ($name: literal, $fg: ident, -, -, $attr: ident, -) => {
        highlight!($name, Some(stringify!($fg)), None, None, $attr, All)
    };
    ($name: literal, $fg: ident, -, -, $attr: ident, $scope: ident) => {
        highlight!($name, Some(stringify!($fg)), None, None, $attr, $scope)
    };
    ($name: literal, -, $bg: ident, -, $attr: ident, -) => {
        highlight!($name, None, Some(stringify!($bg)), None, $attr, All)
    };
    ($name: literal, -, $bg: ident, -, -, $scope: ident) => {
        highlight!($name, None, Some(stringify!($bg)), None, Nothing, $scope)
    };
    ($name: literal, -, $bg: ident, -, $attr: ident, $scope: ident) => {
        highlight!($name, None, Some(stringify!($bg)), None, $attr, $scope)
    };
    ($name: literal, $fg: ident, $bg: ident, -, -, -) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            Nothing,
            -
        )
    };
    ($name: literal, $fg: ident, $bg: ident, -, -, $scope: ident) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            Nothing,
            $scope
        )
    };
    ($name: literal, $fg: ident, $bg: ident, -, $attr: ident, -) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            $attr,
            All
        )
    };
    ($name: literal, $fg: ident, $bg: ident, -, $attr: ident, $scope: ident) => {
        highlight!(
            $name,
            Some(stringify!($fg)),
            Some(stringify!($bg)),
            None,
            $attr,
            $scope
        )
    };
    ($name: literal, -, -, -, $attr: ident, $scope: ident) => {
        highlight!($name, None, None, None, $attr, $scope)
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
                None => panic!("\"{}\" does not exists", stringify!($parent)),
            }
        };
        ($parent: ident, $h: expr, $s: expr, $v: expr) => {
            conv::hue(conv::saturate(conv::lighten(extends!($parent), $v), $s), $h)
        };
    }

    // palettes
    def!(red, "#dc6f7a");
    def!(darkred, extends!(red, 0.0, 0.0, -0.2));
    def!(pink, "#b871b8");
    def!(darkpink, extends!(pink, 0.0, -0.05, -0.35));
    def!(purple, "#929be5");
    def!(darkpurple, extends!(purple, 0.0, 0.05, -0.15));
    def!(darkestpurple, extends!(purple, 0.0, 0.05, -0.42));
    def!(blue, "#589ec6");
    def!(darkblue, extends!(blue, 0.0, 0.0, -0.2));
    def!(darkestblue, extends!(blue, 0.0, 0.05, -0.48));
    def!(cyan, "#59b6b6");
    def!(darkcyan, extends!(cyan, 0.0, 0.0, -0.2));
    def!(teal, "#73c1a9");
    def!(darkteal, extends!(teal, 0.0, 0.0, -0.2));
    def!(darkestteal, extends!(teal, 0.0, 0.05, -0.48));
    def!(green, "#7cbe8c");
    def!(darkgreen, extends!(green, 0.0, -0.05, -0.4));
    def!(yellow, "#a8a384");
    def!(darkyellow, extends!(yellow, 0.0, -0.15, -0.5));
    def!(orange, "#ac8b83");
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
    def!(diffaddbg, extends!(darkestblue));
    def!(diffaddfg, extends!(mainfg, 0.0, 0.0, 0.2));
    def!(diffchangebg, extends!(darkestteal));
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
    def!(searchbg, extends!(purple, 0.0, 0.2, 0.0));
    def!(searchfg, extends!(searchbg, 0.2, -0.2, 0.15));
    def!(incsearchbg, extends!(searchbg, 0.0, 0.08, -0.1));
    def!(incsearchfg, extends!(searchfg, 0.1, 0.1, 0.15));
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

    p
}

pub fn get_highlights() -> Vec<Highlight> {
    vec![
        // general
        hi!("Normal", mainfg, mainbg, -, -, -),
        hi!("Delimiter", lightfg, -, -, -, -),
        hi!("NonText", darkfg, NONE, -, -, -),
        hi!("VertSplit", weakbg, NONE, -, None, -),
        hi!("LineNr", linenrfg, NONE, -, None, -),
        hi!("EndOfBuffer", darkfg, NONE, -, None, -),
        hi!("Comment", weakfg, -, -, None, -),
        hi!("Cursor", mainbg, mainfg, -, -, -),
        hi!("CursorIM", mainbg, mainfg, -, -, -),
        hi!("SignColumn", weakfg, NONE, -, -, -),
        hi!("ColorColumn", -, cursorlinebg, -, None, -),
        hi!("CursorColumn", -, cursorlinebg, -, None, -),
        hi!("CursorLine", -, cursorlinebg, -, None, -),
        hi!("CursorLineNr", cursorlinenrfg, NONE, -, None, -),
        hi!("Conceal", orange, mainbg, -, None, -),
        hi!("NormalFloat", lightfg, NONE, -, None, -),
        hi!("FloatBorder", weakfg, NONE, -, None, -),
        hi!("WinSeparator", darkfg, NONE, -, None, -),
        hi!("Folded", foldfg, foldbg, -, None, -),
        hi!("FoldColumn", linenrfg, NONE, -, None, -),
        hi!("MatchParen", -, matchparenbg, -, -, -),
        hi!("Directory", yellow, -, -, -, -),
        hi!("Underlined", -, -, -, Underline, -),
        hi!("String", green, -, -, -, -),
        hi!("Statement", purple, -, -, None, -),
        hi!("Label", purple, -, -, None, -),
        hi!("Function", purple, -, -, None, -),
        hi!("Constant", teal, -, -, -, -),
        hi!("Boolean", teal, -, -, -, -),
        hi!("Number", teal, -, -, -, -),
        hi!("Float", teal, -, -, -, -),
        hi!("Title", yellow, -, -, Bold, -),
        hi!("Keyword", orange, -, -, -, -),
        hi!("Identifier", orange, -, -, -, -),
        hi!("Exception", yellow, -, -, -, -),
        hi!("Type", yellow, -, -, None, -),
        hi!("TypeDef", yellow, -, -, None, -),
        hi!("PreProc", purple, -, -, -, -),
        hi!("Special", pink, -, -, -, -),
        hi!("SpecialKey", pink, -, -, -, -),
        hi!("SpecialChar", pink, -, -, -, -),
        hi!("SpecialComment", pink, -, -, -, -),
        hi!("Error", errorfg, errorbg, -, Bold, -),
        hi!("ErrorMsg", errorfg, NONE, -, Bold, -),
        hi!("WarningMsg", warningfg, -, -, Bold, -),
        hi!("MoreMsg", morefg, -, -, -, -),
        hi!("Todo", yellow, NONE, -, Bold, -),
        hi!("Pmenu", pmenufg, pmenubg, -, -, -),
        hi!("PmenuSel", pmenuselfg, pmenuselbg, -, -, -),
        hi!("PmenuSbar", -, pmenubar, -, -, -),
        hi!("PmenuThumb", -, pmenuthumb, -, -, -),
        hi!("Visual", -, visualbg, -, None, -),
        hi!("Search", searchfg, searchbg, -, -, -),
        hi!("IncSearch", incsearchfg, incsearchbg, -, None, -),
        hi!("Question", teal, -, -, Bold, -),
        hi!("WildMenu", mainbg, purple, -, -, -),
        hi!("SpellBad", errorfg, -, -, Underline, -),
        hi!("SpellCap", -, -, -, Underline, -),
        hi!("SpellLocal", errorfg, -, -, Underline, -),
        hi!("SpellRare", yellow, -, -, Underline, -),
        hi!("DiffAdd", -, diffaddbg, -, Bold, -),
        hi!("DiffChange", -, diffchangebg, -, Bold, -),
        hi!("DiffDelete", diffdeletefg, diffdeletebg, -, Bold, -),
        hi!("DiffText", -, difftextbg, -, None, -),
        hi!("QuickFixLine", mainfg, visualbg, -, -, -),
        hi!("StatusLine", statuslinefg, statuslinebg, -, Bold, -),
        hi!("StatusLineTerm", statuslinefg, statuslinebg, -, Bold, -),
        hi!("StatusLineNC", statuslinencfg, statuslinencbg, -, None, -),
        hi!("StatusLineTermNC", statuslinencfg, statuslinencbg, -, None, -),
        hi!("TabLine", statuslinefg, statuslinebg, -, None, -),
        hi!("TabLineFill", statuslinefg, statuslinebg, -, None, -),
        hi!("TabLineSel", tablineselfg, tablineselbg, -, Bold, -),
        hi!("qfFileName", teal, -, -, -, -),
        hi!("qfLineNr", weakfg, -, -, -, -),
        // treesitter
        // https://github.com/nvim-treesitter/nvim-treesitter
        hi!("@string", green, -, -, -, Nvim080OrLater),
        hi!("@string.regex", green, -, -, -, Nvim080OrLater),
        hi!("@string.escape", pink, -, -, -, Nvim080OrLater),
        hi!("@string.special.url", weakfg, -, -, -, Nvim080OrLater),
        hi!("@text.title", yellow, -, -, Bold, Nvim080OrLater),
        hi!("@text.reference", purple, -, -, -, Nvim080OrLater),
        hi!("@text.uri", weakfg, -, -, -, Nvim080OrLater),
        hi!("@text.strong", -, -, -, Bold, Nvim080OrLater),
        hi!("@text.literal", teal, -, -, -, Nvim080OrLater),
        hi!("@parameter", purple, -, -, -, Nvim080OrLater),
        hi!("@property", purple, -, -, -, Nvim080OrLater),
        hi!("@keyword", pink, -, -, -, Nvim080OrLater),
        hi!("@operator", purple, -, -, -, Nvim080OrLater),
        hi!("@module", mainfg, -, -, -, Nvim080OrLater),
        hi!("@type", orange, -, -, -, Nvim080OrLater),
        hi!("@type.builtin", orange, -, -, -, Nvim080OrLater),
        hi!("@function.tsx", mainfg, -, -, -, Nvim080OrLater),
        hi!("@punctuation.special.typescript", lightfg, -, -, -, Nvim080OrLater),
        hi!("@include", purple, -, -, -, Nvim080OrLater),
        hi!("@variable", mainfg, -, -, -, Nvim080OrLater),
        hi!("@variable.builtin", orange, -, -, -, Nvim080OrLater),
        hi!("@constant.builtin", teal, -, -, -, Nvim080OrLater),
        hi!("@constructor", mainfg, -, -, -, Nvim080OrLater),
        hi!("@tag", mainfg, -, -, -, Nvim080OrLater),
        hi!("@tag.delimiter", purple, -, -, -, Nvim080OrLater),
        hi!("@tag.attribute", purple, -, -, -, Nvim080OrLater),
        hi!("@tag.builtin.tsx", mainfg, -, -, -, Nvim080OrLater),
        hi!("@markup.heading", yellow, -, -, Bold, Nvim080OrLater),
        hi!("@markup.strong", -, -, -, Bold, Nvim080OrLater),
        hi!("@markup.list", weakfg, -, -, -, Nvim080OrLater),
        hi!("@markup.raw", teal, -, -, -, Nvim080OrLater),
        hi!("@markup.link", purple, -, -, -, Nvim080OrLater),
        hi!("@markup.link.url", lightfg, -, -, -, Nvim080OrLater),
        hi!("@markup.quote", weakfg, -, -, -, Nvim080OrLater),
        // LSP Semantic highlights
        hi!("@lsp.type.class", mainfg, -, -, -, Nvim080OrLater),
        hi!("@lsp.type.interface", orange, -, -, -, Nvim080OrLater),
        hi!("@lsp.type.parameter", purple, -, -, -, Nvim080OrLater),
        hi!("@lsp.type.property", purple, -, -, -, Nvim080OrLater),
        hi!("@lsp.type.struct", mainfg, -, -, -, Nvim080OrLater),
        hi!("@lsp.type.type", orange, -, -, -, Nvim080OrLater),
        hi!("@lsp.type.typeParameter", mainfg, -, -, -, Nvim080OrLater),
        hi!("@lsp.type.variable", mainfg, -, -, -, Nvim080OrLater),
        hi!("@lsp.type.member", purple, -, -, -, Nvim080OrLater),
        hi!("@lsp.type.namespace", mainfg, -, -, -, Nvim080OrLater),
        // built-in LSP
        hi!("DiagnosticError", errorfg, -, -, -, -),
        hi!("DiagnosticVirtualTextError", errorfg, -, -, Bold, -),
        hi!("DiagnosticUnderlineError", errorfg, -, -, Underline, -),
        hi!("DiagnosticWarn", warningfg, -, -, -, -),
        hi!("DiagnosticVirtualTextWarn", warningfg, -, -, Bold, -),
        hi!("DiagnosticUnderlineWarn", warningfg, -, -, Underline, -),
        hi!("DiagnosticInfo", infofg, -, -, -, -),
        hi!("DiagnosticVirtualTextInfo", weakfg, -, -, Bold, -),
        hi!("DiagnosticUnderlineInfo", -, -, -, Underline, -),
        hi!("DiagnosticHint", infofg, -, -, -, -),
        hi!("DiagnosticVirtualTextHint", weakfg, -, -, Bold, -),
        hi!("DiagnosticUnderlineHint", -, -, -, Underline, -),
        hi!("LspSignatureActiveParameter", -, -, -, Italic, -),
        hi!("LspReferenceText", -, matchparenbg, -, -, -),
        hi!("LspReferenceRead", -, matchparenbg, -, -, -),
        hi!("LspReferenceWrite", -, matchparenbg, -, -, -),
        // html
        hi!("htmlTag", lightfg, -, -, -, -),
        hi!("htmlEndTag", lightfg, -, -, -, -),
        hi!("htmlSpecialTagName", orange, -, -, -, -),
        hi!("htmlArg", lightfg, -, -, -, -),
        // json
        hi!("jsonQuote", lightfg, -, -, -, -),
        // yaml
        hi!("yamlBlockMappingKey", purple, -, -, -, -),
        hi!("yamlAnchor", pink, -, -, -, -),
        // python
        hi!("pythonStatement", orange, -, -, -, -),
        hi!("pythonBuiltin", cyan, -, -, -, -),
        hi!("pythonRepeat", orange, -, -, -, -),
        hi!("pythonOperator", orange, -, -, -, -),
        hi!("pythonDecorator", pink, -, -, -, -),
        hi!("pythonDecoratorName", pink, -, -, -, -),
        // zsh
        hi!("zshVariableDef", purple, -, -, -, -),
        hi!("zshFunction", purple, -, -, -, -),
        hi!("zshKSHFunction", purple, -, -, -, -),
        // C
        hi!("cPreCondit", orange, -, -, -, -),
        hi!("cIncluded", pink, -, -, -, -),
        hi!("cStorageClass", orange, -, -, -, -),
        // C++
        // octol/vim-cpp-enhanced-highlight
        hi!("cppStructure", pink, -, -, -, -),
        hi!("cppSTLnamespace", orange, -, -, -, -),
        // C#
        hi!("csStorage", orange, -, -, -, -),
        hi!("csModifier", purple, -, -, -, -),
        hi!("csClass", purple, -, -, -, -),
        hi!("csClassType", pink, -, -, -, -),
        hi!("csNewType", orange, -, -, -, -),
        // ruby
        hi!("rubyConstant", orange, -, -, -, -),
        hi!("rubySymbol", purple, -, -, -, -),
        hi!("rubyBlockParameter", purple, -, -, -, -),
        hi!("rubyClassName", pink, -, -, -, -),
        hi!("rubyInstanceVariable", pink, -, -, -, -),
        // yats.vim
        // https://github.com/HerringtonDarkholme/yats.vim
        hi!("typescriptImport", purple, -, -, -, -),
        hi!("typescriptDocRef", weakfg, -, -, Underline, -),
        // vim-markdown
        // https://github.com/plasticboy/vim-markdown
        hi!("mkdHeading", weakfg, -, -, -, -),
        hi!("mkdLink", purple, -, -, -, -),
        hi!("mkdCode", purple, -, -, -, -),
        hi!("mkdCodeStart", purple, -, -, -, -),
        hi!("mkdCodeEnd", purple, -, -, -, -),
        hi!("mkdCodeDelimiter", purple, -, -, -, -),
        // vim-toml
        // https://github.com/cespare/vim-toml
        hi!("tomlTable", purple, -, -, -, -),
        // rust.vim
        // https://github.com/rust-lang/rust.vim
        hi!("rustModPath", purple, -, -, -, -),
        hi!("rustTypedef", purple, -, -, -, -),
        hi!("rustStructure", purple, -, -, -, -),
        hi!("rustMacro", purple, -, -, -, -),
        hi!("rustExternCrate", purple, -, -, -, -),
        // vim-graphql
        // https://github.com/jparise/vim-graphql
        hi!("graphqlStructure", pink, -, -, -, -),
        hi!("graphqlDirective", pink, -, -, -, -),
        hi!("graphqlName", purple, -, -, -, -),
        hi!("graphqlTemplateString", mainfg, -, -, -, -),
        // vimfiler
        // https://github.com/Shougo/vimfiler.vim
        hi!("vimfilerOpenedFile", darkpurple, -, -, -, -),
        hi!("vimfilerClosedFile", darkpurple, -, -, -, -),
        hi!("vimfilerNonMark", teal, -, -, -, -),
        hi!("vimfilerLeaf", teal, -, -, -, -),
        // defx-icons
        // https://github.com/kristijanhusak/defx-icons
        hi!("DefxIconsMarkIcon", darkpurple, -, -, None, -),
        hi!("DefxIconsDirectory", darkpurple, -, -, None, -),
        hi!("DefxIconsParentDirectory", darkpurple, -, -, None, -),
        hi!("DefxIconsSymlinkDirectory", teal, -, -, None, -),
        hi!("DefxIconsOpenedTreeIcon", darkpurple, -, -, None, -),
        hi!("DefxIconsNestedTreeIcon", darkpurple, -, -, None, -),
        hi!("DefxIconsClosedTreeIcon", darkpurple, -, -, None, -),
        // defx-git
        // https://github.com/kristijanhusak/defx-git
        hi!("Defx_git_Untracked", purple, -, -, None, -),
        hi!("Defx_git_Ignored", weakfg, -, -, None, -),
        hi!("Defx_git_Unknown", weakfg, -, -, None, -),
        hi!("Defx_git_Renamed", diffchangebg, -, -, -, -),
        hi!("Defx_git_Modified", diffchangebg, -, -, -, -),
        hi!("Defx_git_Unmerged", pink, -, -, -, -),
        hi!("Defx_git_Deleted", diffdeletebg, -, -, -, -),
        hi!("Defx_git_Staged", teal, -, -, -, -),
        // nvim-tree/nvim-tree.lua
        // https://github.com/nvim-tree/nvim-tree.lua
        hi!("NvimTreeSymlink", darkteal, -, -, None, -),
        hi!("NvimTreeSymlinkFolderName", darkteal, -, -, None, -),
        hi!("NvimTreeFolderName", purple, -, -, None, -),
        hi!("NvimTreeRootFolder", darkestpurple, -, -, Bold, -),
        hi!("NvimTreeFolderIcon", darkpurple, -, -, None, -),
        hi!("NvimTreeFileIcon", darkpurple, -, -, None, -),
        hi!("NvimTreeEmptyFolderName", weakfg, -, -, None, -),
        hi!("NvimTreeOpenedFolderName", purple, -, -, None, -),
        hi!("NvimTreeExecFile", lightfg, -, -, None, -),
        hi!("NvimTreeOpenedHL", lightfg, -, -, None, -),
        hi!("NvimTreeSpecialFile", lightfg, -, -, Bold, -),
        hi!("NvimTreeImageFile", lightfg, -, -, None, -),
        hi!("NvimTreeIndentMarker", darkestpurple, -, -, None, -),
        hi!("NvimTreeModifiedIcon", mainfg, -, -, None, -),
        hi!("NvimTreeGitDirtyIcon", yellow, -, -, None, -),
        hi!("NvimTreeGitStagedIcon", green, -, -, None, -),
        hi!("NvimTreeGitMergeIcon", yellow, -, -, None, -),
        hi!("NvimTreeGitRenamedIcon", yellow, -, -, None, -),
        hi!("NvimTreeGitNewIcon", teal, -, -, None, -),
        hi!("NvimTreeGitDeletedIcon", difftextbg, -, -, None, -),
        hi!("NvimTreeWindowPicker", tablineselfg, tablineselbg, -, Bold, -),
        hi!("NvimTreeNormal", lightfg, -, -, None, -),
        hi!("NvimTreeLiveFilterPrefix", darkteal, -, -, None, -),
        hi!("NvimTreeLiveFilterValue", teal, -, -, None, -),
        hi!("NvimTreeBookmarkIcon", yellow, -, -, None, -),
        // fern.vim
        // https://github.com/lambdalisue/fern.vim
        hi!("FernBranchSymbol", darkpurple, -, -, None, -),
        hi!("FernBranchText", purple, -, -, None, -),
        hi!("FernLeafSymbol", darkteal, -, -, None, -),
        hi!("FernLeafText", mainfg, -, -, None, -),
        hi!("FernMarked", cyan, -, -, None, -),
        // gitsigns.nvim
        // https://github.com/lewis6991/gitsigns.nvim
        hi!("GitSignsAdd", green, -, -, -, -),
        hi!("GitSignsChange", yellow, -, -, -, -),
        hi!("GitSignsDelete", pink, -, -, -, -),
        hi!("GitSignsChangeDelete", difftextbg, -, -, -, -),
        // vim-gitgutter
        // https://github.com/airblade/vim-gitgutter
        hi!("GitGutterAdd", green, -, -, -, -),
        hi!("GitGutterChange", yellow, -, -, -, -),
        hi!("GitGutterDelete", pink, -, -, -, -),
        hi!("GitGutterChangeDelete", difftextbg, -, -, -, -),
        // fugitive.vim
        // https://github.com/tpope/vim-fugitive
        hi!("fugitiveHeader", teal, -, -, Bold, -),
        // Diffview.nvim
        // https://github.com/sindrets/diffview.nvim
        hi!("DiffviewDim1", weakfg, -, -, -, -),
        hi!("DiffviewPrimary", purple, -, -, -, -),
        hi!("DiffviewSecondary", pink, -, -, -, -),
        hi!("DiffviewStatusAdded", blue, -, -, -, -),
        hi!("DiffviewStatusUntracked", yellow, -, -, -, -),
        hi!("DiffviewStatusModified", teal, -, -, -, -),
        hi!("DiffviewStatusRenamed", teal, -, -, -, -),
        hi!("DiffviewStatusCopied", teal, -, -, -, -),
        hi!("DiffviewStatusTypeChanged", teal, -, -, -, -),
        hi!("DiffviewStatusUnmerged", pink, -, -, -, -),
        hi!("DiffviewStatusUnknown", yellow, -, -, -, -),
        hi!("DiffviewStatusDeleted", lightfg, -, -, -, -),
        hi!("DiffviewStatusBroken", pink, -, -, -, -),
        hi!("DiffviewStatusIgnored", yellow, -, -, -, -),
        hi!("DiffviewFilePanelRootPath", darkpurple, -, -, -, -),
        hi!("DiffviewFilePanelTitle", purple, -, -, Bold, -),
        hi!("DiffviewFilePanelCounter", lightfg, -, -, Bold, -),
        hi!("DiffviewFilePanelFileName", mainfg, -, -, -, -),
        hi!("DiffviewFilePanelPath", weakfg, -, -, Bold, -),
        hi!("DiffviewFilePanelSelected", yellow, -, -, -, -),
        hi!("DiffviewFilePanelInsertions", teal, -, -, -, -),
        hi!("DiffviewFilePanelDeletions", pink, -, -, -, -),
        hi!("DiffviewFilePanelConflicts", warningfg, -, -, -, -),
        hi!("DiffviewHash", darkpurple, -, -, -, -),
        // ale
        // https://github.com/dense-analysis/ale
        hi!("ALEWarningSign", warningfg, -, -, Bold, -),
        hi!("ALEInfoSign", infofg, -, -, None, -),
        // null-ls.nvim
        // https://github.com/jose-elias-alvarez/null-ls.nvim
        hi!("NullLsInfoBorder", weakfg, mainbg, -, None, -),
        // coc.nvim
        // https://github.com/neoclide/coc.nvim
        hi!("CocErrorSign", errorfg, -, -, Bold, -),
        hi!("CocWarningSign", warningfg, -, -, Bold, -),
        hi!("CocInfoSign", infofg, -, -, Bold, -),
        hi!("CocHintSign", infofg, -, -, Bold, -),
        // vim-lsp
        // https://github.com/prabirshrestha/vim-lsp
        hi!("LspError", errorfg, -, -, -, -),
        hi!("LspErrorText", errorfg, -, -, Bold, -),
        hi!("LspErrorHighlight", -, -, -, Underline, -),
        hi!("LspErrorVirtualText", errorfg, -, -, Bold, -),
        hi!("LspWarning", warningfg, -, -, -, -),
        hi!("LspWarningText", warningfg, -, -, Bold, -),
        hi!("LspWarningHighlight", -, -, -, Underline, -),
        hi!("LspWarningVirtualText", warningfg, -, -, Bold, -),
        hi!("LspInformation", infofg, -, -, -, -),
        hi!("LspInformationText", infofg, -, -, Bold, -),
        hi!("LspInformationHighlight", -, -, -, Underline, -),
        hi!("LspInformationVirtualText", weakfg, -, -, Bold, -),
        hi!("LspHint", infofg, -, -, -, -),
        hi!("LspHintText", infofg, -, -, Bold, -),
        hi!("LspHintHighlight", -, -, -, Underline, -),
        hi!("LspHintVirtualText", weakfg, -, -, Bold, -),
        hi!("LspCodeActionText", darkpurple, -, -, Bold, -),
        // nvim-cmp
        // https://github.com/hrsh7th/nvim-cmp
        hi!("CmpItemAbbr", mainfg, -, -, -, -),
        hi!("CmpItemAbbrMatch", purple, -, -, Bold, -),
        hi!("CmpItemAbbrMatchFuzzy", purple, -, -, Bold, -),
        hi!("CmpItemKind", lightfg, -, -, -, -),
        hi!("CmpItemKindDefault", lightfg, -, -, -, -),
        hi!("CmpItemKindText", lightfg, -, -, -, -),
        hi!("CmpItemKindVariable", lightfg, -, -, -, -),
        hi!("CmpItemKindKeyword", lightfg, -, -, -, -),
        hi!("CmpItemKindInterface", lightfg, -, -, -, -),
        hi!("CmpItemKindFunction", lightfg, -, -, -, -),
        hi!("CmpItemKindMethod", lightfg, -, -, -, -),
        hi!("CmpItemKindProperty", lightfg, -, -, -, -),
        hi!("CmpItemKindUnit", lightfg, -, -, -, -),
        // dressing.nvim
        // https://github.com/stevearc/dressing.nvim
        hi!("FloatTitle", purple, mainbg, -, None, -),
        // telescope.nvim
        // https://github.com/nvim-telescope/telescope.nvim
        hi!("TelescopeNormal", lightfg, -, -, -, -),
        hi!("TelescopeTitle", purple, -, -, -, -),
        hi!("TelescopeMatching", emphasisfg, -, -, Bold, -),
        hi!("TelescopeBorder", weakfg, -, -, -, -),
        hi!("TelescopePromptPrefix", teal, -, -, -, -),
        hi!("TelescopePromptCounter", weakfg, -, -, -, -),
        hi!("TelescopeMultiIcon", yellow, -, -, -, -),
        hi!("TelescopeMultiSelection", yellow, -, -, -, -),
        // Copilot.vim
        // https://github.com/github/copilot.vim
        hi!("CopilotSuggestion", weakfg, -, -, -, -),
        // clever-f.vim
        // https://github.com/rhysd/clever-f.vim
        hi!("CleverFChar", searchfg, searchbg, -, Underline, -),
        // conflict-marker.vim
        // https://github.com/rhysd/conflict-marker.vim
        hi!("ConflictMarkerBegin", -, darkteal, -, Bold, -),
        hi!("ConflictMarkerOurs", -, darkestteal, -, None, -),
        hi!("ConflictMarkerTheirs", -, darkestblue, -, None, -),
        hi!("ConflictMarkerEnd", -, darkblue, -, Bold, -),
        hi!("ConflictMarkerSeparator", darkfg, -, -, Bold, -),
        // easymotion
        // https://github.com/easymotion/vim-easymotion
        hi!("EasyMotionTarget", yellow, -, -, Bold, -),
        hi!("EasyMotionShade", weakfg, mainbg, -, -, -),
        hi!("EasyMotionIncCursor", mainfg, mainbg, -, -, -),
        // flash.nvim
        // https://github.com/folke/flash.nvim
        hi!("FlashPrompt", purple, -, -, Bold, -),
        hi!("FlashPromptIcon", teal, -, -, Bold, -),
        hi!("FlashLabel", teal, -, -, Bold, -),
        // fidget.nvim
        // https://github.com/j-hui/fidget.nvim
        hi!("FidgetTitle", teal, -, -, Bold, -),
        hi!("FidgetTask", weakfg, -, -, -, -),
    ]
}
