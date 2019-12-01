![vim-dogrun](./docs/images/repo-banner.png)

[![Actions Status](https://github.com/wadackel/vim-dogrun/workflows/CI/badge.svg)](https://github.com/wadackel/vim-dogrun/actions)

> A dark Vim/Neovim colorscheme for the GUI and 16/256/true-color terminals.

## Screenshots

### Neovim (GUI)

![Neovim](./docs/images/neovim.png)

### Vim (with 256 colors)

![Vim](./docs/images/vim.png)

## Features

- No settings are required to start using it.
- Support 256 colors terminal.
- Supports [lightline](https://github.com/itchyny/lightline.vim) theme.

From now on, we will gradually support file types and plug-ins :dog:

## Installation

This is an example of installation using [vim-plug](https://github.com/junegunn/vim-plug).

```vim
Plug 'wadackel/vim-dogrun'
```

## Usage

Add the following settings to your `$MYVIMRC` (`.vimrc` or `init.vim`).

```vim
colorscheme dogrun
```

### Options

#### lightline theme

If you want, specify `dogrun` for` colorscheme`.

```vim
let g:lightline = {
  \ 'colorscheme': 'dogrun',
  \ }
```

## License

[MIT License © wadackel](./LICENSE)
