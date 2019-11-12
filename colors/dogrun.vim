" dogrun: WIP
"
" Author: wadackel
" License: MIT
"   Copyright (c) 2019 wadackel

hi clear

if exists("syntax on")
  syntax reset
endif

if &background !=# 'dark'
  set background=dark
endif

let g:colors_name = "dogrun"

function! s:h(group, style, ...)
  exe 'highlight' a:group
    \ 'guifg=' (has_key(a:style, 'fg') ? a:style.fg.gui : 'NONE')
    \ 'guibg=' (has_key(a:style, 'bg') ? a:style.bg.gui : 'NONE')
    \ 'guisp=' (has_key(a:style, 'sp') ? a:style.sp.gui : 'NONE')
    \ 'gui=' (has_key(a:style, 'gui') ? a:style.gui : 'NONE')
    \ 'ctermfg=' (has_key(a:style, 'fg') ? a:style.fg.cterm : 'NONE')
    \ 'ctermbg=' (has_key(a:style, 'bg') ? a:style.bg.cterm : 'NONE')
    \ 'term=NONE'
    \ 'cterm=NONE'
endfunction

" Define reusable color variables.
let s:gray900 = {'gui': '#1a1c29', 'cterm': '234'}
let s:gray800 = {'gui': '#222433', 'cterm': '235'}
let s:gray700 = {'gui': '#303242', 'cterm': '237'}
let s:gray600 = {'gui': '#686c8a', 'cterm': '60'}
let s:gray500 = {'gui': '#898da8', 'cterm': '103'}
let s:gray400 = {'gui': '#9ea3c0', 'cterm': '248'}
let s:gray300 = {'gui': '#c6c8d7', 'cterm': '252'}
let s:gray200 = {'gui': '#dadce6', 'cterm': '253'}
let s:gray100 = {'gui': '#e6e6ec', 'cterm': '254'}

let s:purple900 = {'gui': '#343b76', 'cterm': '239'}
let s:purple800 = {'gui': '#555c98', 'cterm': '60'}
let s:purple700 = {'gui': '#6973c6', 'cterm': '62'}
let s:purple600 = {'gui': '#808ada', 'cterm': '104'}
let s:purple500 = {'gui': '#929be5', 'cterm': '104'}
let s:purple400 = {'gui': '#a9b1f4', 'cterm': '147'}
let s:purple300 = {'gui': '#bdc3ee', 'cterm': '147'}
let s:purple200 = {'gui': '#d3d7f3', 'cterm': '189'}
let s:purple100 = {'gui': '#dfe2f4', 'cterm': '254'}

let s:red100 = {'gui': '#fff2f3', 'cterm': '231'}
let s:red200 = {'gui': '#fdcdd2', 'cterm': '224'}
let s:red300 = {'gui': '#fca9b1', 'cterm': '217'}
let s:red400 = {'gui': '#f56574', 'cterm': '204'}
let s:red500 = {'gui': '#eb2e43', 'cterm': '161'}
let s:red600 = {'gui': '#dc0823', 'cterm': '160'}
let s:red700 = {'gui': '#c7001c', 'cterm': '160'}
let s:red800 = {'gui': '#ab0019', 'cterm': '124'}
let s:red900 = {'gui': '#8a0014', 'cterm': '88'}

let s:pink100 = {'gui': '#e0c1e0', 'cterm': '182'}
let s:pink200 = {'gui': '#ddacdd', 'cterm': '182'}
let s:pink300 = {'gui': '#d898d8', 'cterm': '176'}
let s:pink400 = {'gui': '#ce85ce', 'cterm': '176'}
let s:pink500 = {'gui': '#c173c1', 'cterm': '133'}
let s:pink600 = {'gui': '#ae62ae', 'cterm': '133'}
let s:pink700 = {'gui': '#955195', 'cterm': '96'}
let s:pink800 = {'gui': '#773f77', 'cterm': '96'}
let s:pink900 = {'gui': '#572d57', 'cterm': '238'}

let s:orange100 = {'gui': '#e0c5c1', 'cterm': '252'}
let s:orange200 = {'gui': '#ddb5ac', 'cterm': '181'}
let s:orange300 = {'gui': '#d8a398', 'cterm': '180'}
let s:orange400 = {'gui': '#ce9385', 'cterm': '174'}
let s:orange500 = {'gui': '#c18373', 'cterm': '137'}
let s:orange600 = {'gui': '#ae7262', 'cterm': '131'}
let s:orange700 = {'gui': '#955f51', 'cterm': '95'}
let s:orange800 = {'gui': '#774b3f', 'cterm': '95'}
let s:orange900 = {'gui': '#57362d', 'cterm': '237'}

let s:yellow100 = {'gui': '#e0e0ca', 'cterm': '253'}
let s:yellow200 = {'gui': '#dddcba', 'cterm': '187'}
let s:yellow300 = {'gui': '#d8d4a8', 'cterm': '187'}
let s:yellow400 = {'gui': '#cec996', 'cterm': '186'}
let s:yellow500 = {'gui': '#c1ba85', 'cterm': '144'}
let s:yellow600 = {'gui': '#aea674', 'cterm': '144'}
let s:yellow700 = {'gui': '#958e60', 'cterm': '101'}
let s:yellow800 = {'gui': '#77714c', 'cterm': '95'}
let s:yellow900 = {'gui': '#575237', 'cterm': '239'}

let s:teal100 = {'gui': '#c1e0d7', 'cterm': '152'}
let s:teal200 = {'gui': '#acddcf', 'cterm': '152'}
let s:teal300 = {'gui': '#98d8c5', 'cterm': '116'}
let s:teal400 = {'gui': '#85ceb8', 'cterm': '115'}
let s:teal500 = {'gui': '#73c1a9', 'cterm': '73'}
let s:teal600 = {'gui': '#62ae96', 'cterm': '72'}
let s:teal700 = {'gui': '#519580', 'cterm': '66'}
let s:teal800 = {'gui': '#3f7765', 'cterm': '65'}
let s:teal900 = {'gui': '#2d574a', 'cterm': '238'}

let s:green100 = {'gui': '#b8d1bf', 'cterm': '251'}
let s:green200 = {'gui': '#aed0b6', 'cterm': '151'}
let s:green300 = {'gui': '#a4ceae', 'cterm': '151'}
let s:green400 = {'gui': '#8fc89d', 'cterm': '115'}
let s:green500 = {'gui': '#7cbe8c', 'cterm': '108'}
let s:green600 = {'gui': '#6aaf7c', 'cterm': '72'}
let s:green700 = {'gui': '#599a69', 'cterm': '65'}
let s:green800 = {'gui': '#477f55', 'cterm': '65'}
let s:green900 = {'gui': '#356140', 'cterm': '238'}

" Usual semantic colors.
let s:mainfg = s:gray400
let s:mainbg = s:gray800

" editor
call s:h('Normal', {'fg': s:mainfg, 'bg': s:mainbg})
call s:h('NonText', {'fg': s:gray600})
call s:h('LineNr', {'fg': s:gray700})
call s:h('SignColumn', {})
call s:h('FoldColumn', {})
call s:h('Folded', {'fg': s:purple800, 'bg': s:gray700})
call s:h('VertSplit', {'fg': s:purple900})
call s:h('Conceal', {}) " TODO
call s:h('Cursor', {'fg': s:gray100, 'bg': s:purple400})
call s:h('CursorIM', {'fg': s:gray100, 'bg': s:purple400})
call s:h('CursorColumn', {}) " TODO
call s:h('CursorLine', {}) " TODO
call s:h('CursorLineNr', {}) " TODO
call s:h('Search', {'fg': s:gray100, 'bg': s:purple800})
call s:h('IncSearch', {'fg': s:gray100, 'bg': s:purple800})
call s:h('Directory', {'fg': s:yellow200})
call s:h('MatchParen', {'bg': s:gray600, 'gui': 'underline'})
call s:h('Error', {'fg': s:red400, 'bg': s:gray700, 'gui': 'bold'})
call s:h('ErrorMsg', {'fg': s:pink600, 'gui': 'bold'}) " TODO
call s:h('DiffAdd', {'fg': s:gray100, 'bg': s:green800, 'gui': 'bold'})
call s:h('DiffChange', {'fg': s:gray100, 'bg': s:yellow800, 'gui': 'bold'})
call s:h('DiffDelete', {'fg': s:mainfg, 'bg': s:pink800, 'gui': 'bold'})
call s:h('DiffText', {'bg': s:mainbg})
call s:h('WarningMsg', {'fg': s:yellow500, 'gui': 'bold'})
call s:h('Underlined', {'gui': 'underline'})
call s:h('Todo', {'fg': s:yellow100, 'gui': 'italic'})
call s:h('Ignore', {}) " TODO
call s:h('Debug', {}) " TODO
call s:h('Terminal', {}) " TODO
call s:h('ModeMsg', {}) " TODO
call s:h('MoreMsg', {}) " TODO
call s:h('WildMenu', {}) " TODO
call s:h('Pmenu', {'fg': s:gray100, 'bg': s:gray700})
call s:h('PmenuSel', {'fg': s:gray200, 'bg': s:purple800})
call s:h('PmenuSbar', {'bg': s:gray600})
call s:h('PmenuThumb', {'bg': s:purple600})
call s:h('Question', {'fg': s:purple600})
call s:h('QuickFixLine', {'bg': s:purple900})
call s:h('StatusLine', {'fg': s:gray500, 'bg': s:gray900})
call s:h('StatusLineNC', {'fg': s:gray500, 'bg': s:gray900})
call s:h('StatusLineTerm', {'fg': s:gray500, 'bg': s:gray900})
call s:h('StatusLineTermNC', {'fg': s:gray500, 'bg': s:gray900})
call s:h('TabLine', {}) " TODO
call s:h('TabLineFill', {}) " TODO
call s:h('TabLineSel', {'bg': s:gray900}) " TODO
call s:h('EndOfBuffer', {'fg': s:mainfg, 'bg': s:mainbg})

" mode
call s:h('Visual', {'fg': s:gray100, 'bg': s:purple800})

" language
call s:h('Comment', {'fg': s:gray600, 'gui': 'italic'})
call s:h('Boolean', {'fg': s:teal500})
call s:h('String', {'fg': s:green400})
call s:h('Character', {'fg': s:green500})
call s:h('Statement', {'fg': s:purple400})
call s:h('Type', {'fg': s:yellow500})
call s:h('Typedef', {'fg': s:yellow500})
call s:h('Structure', {'fg': s:yellow500})
call s:h('Constant', {'fg': s:teal500})
call s:h('Number', {'fg': s:teal500})
call s:h('Float', {'fg': s:teal500})
call s:h('Keyword', {'fg': s:orange400})
call s:h('Identifier', {'fg': s:orange300})
call s:h('Conditional', {'fg': s:pink500})
call s:h('Label', {'fg': s:purple500})
call s:h('Exception', {'fg': s:yellow500})
call s:h('Operater', {'fg': s:pink700})
call s:h('Function', {'fg': s:purple400})
call s:h('Title', {'fg': s:yellow500})
call s:h('Special', {'fg': s:pink500})
call s:h('SpecialKey', {'fg': s:pink500})
call s:h('SpecialChar', {'fg': s:pink500})
call s:h('SpecialComment', {'fg': s:pink500})
call s:h('PreProc', {'fg': s:pink500})
call s:h('qfFileName', {'fg': s:yellow200})
call s:h('qfLineNr', {'fg': s:purple300})

if has('nvim')
  let g:terminal_color_0 = s:gray800.gui
  let g:terminal_color_1 = s:pink500.gui
  let g:terminal_color_2 = s:green500.gui
  let g:terminal_color_3 = s:yellow500.gui
  let g:terminal_color_4 = s:teal500.gui
  let g:terminal_color_5 = s:purple500.gui
  let g:terminal_color_6 = s:teal200.gui
  let g:terminal_color_7 = s:gray100.gui
  let g:terminal_color_8 = s:gray600.gui
  let g:terminal_color_9 = s:red400.gui
  let g:terminal_color_10 = s:green300.gui
  let g:terminal_color_11 = s:yellow300.gui
  let g:terminal_color_12 = s:teal200.gui
  let g:terminal_color_13 = s:purple200.gui
  let g:terminal_color_15 = s:gray600.gui
  let g:terminal_color_background = g:terminal_color_0
  let g:terminal_color_foreground = g:terminal_color_7
endif

" markdown
call s:h('mkdHeading', {'fg': s:purple300})
call s:h('mkdLink', {'fg': s:purple400})
call s:h('mkdCode', {'fg': s:purple400})
hi link mkdCodeStart mkdCode
hi link mkdCodeEnd mkdCode
hi link mkdCodeDelimiter mkdCode

" vim-gitgutter
" https://github.com/airblade/vim-gitgutter
call s:h('GitGutterAdd', {'fg': s:green500})
call s:h('GitGutterChange', {'fg': s:yellow500})
call s:h('GitGutterDelete', {'fg': s:pink500})
call s:h('GitGutterChangeDelete', {'fg': s:yellow300})

" ale
" https://github.com/dense-analysis/ale
call s:h('ALEErrorSign', {'fg': s:red400, 'gui': 'bold'})
call s:h('ALEWarningSign', {'fg': s:orange400, 'gui': 'bold'})
call s:h('ALEInfoSign', {'fg': s:orange500})
call s:h('ALEError', {'fg': s:red400, 'bg': s:gray700})
call s:h('ALEWarning', {'fg': s:orange600, 'bg': s:gray700})

" clever-f.vim
" https://github.com/rhysd/clever-f.vim
call s:h('CleverFChar', {'fg': s:mainbg, 'bg': s:teal700})

" fzf.vim
" https://github.com/junegunn/fzf.vim
" TODO
hi default DogrunFzfFg guifg=s:mainfg.gui
hi default DogrunFzfBg guibg=s:gray700.gui
hi default DogrunFzfCursorLine guibg=s:gray800.gui
hi default DogrunFzfCursorColumn guibg=s:gray800.gui

let g:fzf_colors = {
      \ 'fg':      ['fg', 'DogrunFzfFg'],
      \ 'bg':      ['bg', 'DogrunFzfBg'],
      \ 'hl':      ['fg', 'DogrunFzfBg'],
      \ 'fg+':     ['fg', 'DogrunFzfCursorLine', 'DogrunFzfCursorColumn', 'Normal'],
      \ 'bg+':     ['bg', 'DogrunFzfCursorLine'],
      \ 'hl+':     ['fg', 'Statement'],
      \ 'info':    ['fg', 'PreProc'],
      \ 'border':  ['fg', 'Ignore'],
      \ 'prompt':  ['fg', 'Conditional'],
      \ 'pointer': ['fg', 'Exception'],
      \ 'marker':  ['fg', 'Keyword'],
      \ 'spinner': ['fg', 'Label'],
      \ 'header':  ['fg', 'Title'],
      \ }
