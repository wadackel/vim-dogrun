" lightline
" https://github.com/itchyny/lightline.vim
let s:gray   = [s:gray900.gui, s:gray900.cterm]
let s:white  = [s:gray500.gui, s:gray500.cterm]
let s:blue   = [s:purple300.gui, s:purple300.cterm]
let s:green  = [s:green300.gui, s:green300.cterm]
let s:purple = [s:purple400.gui, s:purple400.cterm]
let s:red    = [s:pink600.gui, s:pink600.cterm]
let s:pink   = [s:pink400.gui, s:pink400.cterm]
let s:yellow = [s:yellow500.gui, s:yellow500.cterm]

let s:lightline = {'normal': {}, 'inactive': {}, 'insert': {}, 'replace': {}, 'visual': {}, 'tabline': {}}
let s:lightline.normal.left     = [[s:gray, s:purple], [s:purple, s:gray]]
let s:lightline.normal.right    = [[s:gray, s:purple], [s:purple, s:gray]]
let s:lightline.inactive.left   = [[s:gray, s:blue], [s:blue, s:gray]]
let s:lightline.inactive.right  = [[s:gray, s:blue], [s:blue, s:gray]]
let s:lightline.insert.left     = [[s:gray, s:green], [s:green, s:gray]]
let s:lightline.insert.right    = [[s:gray, s:green], [s:green, s:gray]]
let s:lightline.replace.left    = [[s:gray, s:red], [s:red, s:gray]]
let s:lightline.replace.right   = [[s:gray, s:red], [s:red, s:gray]]
let s:lightline.visual.left     = [[s:gray, s:pink], [s:pink, s:gray]]
let s:lightline.visual.right    = [[s:gray, s:pink], [s:pink, s:gray]]
let s:lightline.normal.middle   = [[s:white, s:gray]]
let s:lightline.inactive.middle = [[s:white, s:gray]]
let s:lightline.tabline.left    = [[s:purple, s:gray]]
let s:lightline.tabline.tabsel  = [[s:gray, s:purple]]
let s:lightline.tabline.middle  = [[s:purple, s:gray]]
let s:lightline.tabline.right   = [[s:gray, s:purple]]
let s:lightline.normal.error    = [[s:red, s:gray]]
let s:lightline.normal.warning  = [[s:yellow, s:gray]]

let g:lightline#colorscheme#dogrun#palette = lightline#colorscheme#flatten(s:lightline)
