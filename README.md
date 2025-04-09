# dotman [WIP]

A CLI tool for managing dotfiles written in Rust.

## Installation

```bash
cargo install --git https://github.com/k-kuroguro/dotman
```

## Usage

```bash
$ dotman help
A dotfiles manager

Usage: dotman [OPTIONS] <COMMAND>

Commands:
  links       Manage symlinks for dotfiles
  update      Update dotfiles repository (alias for `git -C <dotfiles directory> pull origin master`)
  self        Modify the dotman installation
  completion  Generate shell completion scripts
  help        Print this message or the help of the given subcommand(s)

Options:
  -d, --dotfiles-dir <DOTFILES_DIR>  Path to the dotfiles directory [env: DOTFILES_DIR=] [default: ~/dotfiles]
  -h, --help                         Print help
  -V, --version                      Print version
```

### links install

```bash
$ dotman links install
```

Create symlinks for the dotfiles in the specified directory.

By default, you will be prompted for confirmation if existing files are overwritten. If you want to skip this confirmation, use the `--force` option.

Symlink mappings are specified in the `.dotman.yaml` file:
```yaml
# Symbolic link mappings
# Key: Source file (relative to dotfiles directory)
# Value: Destination path (absolute)
mappings:
  bash/.bashrc: ~/.bashrc
  bash/.bash_aliases: ~/.bash_aliases
  git/.gitconfig: ~/.gitconfig
```

### links remove

Remove all symlinks that were created by `links install`

```bash
$ dotman links remove
```

### links list

List all symlinks that were created by `links install`

```bash
$ dotman links list
```

### update

[WIP]

### self update

[WIP]

### completion

Generate shell completion scripts for the specified shell.

```bash
$ dotman completion -s <SHELL>
```

This command supports the bash, elvish, fish, powershell, and zsh shells via [clap_complete](https://docs.rs/clap_complete).
For installation, please refer to your shell's documentation.
