# SDMW - A Simple Dotfiles Manager Written in Rust

![workflow_build](https://github.com/p1486/sdmw/actions/workflows/build.yml/badge.svg)
![workflow_test](https://github.com/p1486/sdmw/actions/workflows/test.yml/badge.svg)
![Crates.io Version](https://img.shields.io/crates/v/sdmw)

## v0.3.0
Add a new subcommand "restore".

## Requirements
- Unix-like operating system
- installed `git`

## Installation
Run the following Cargo command:
```
cargo install sdmw
```
Or download prebuilt binary from the [GitHub release page](https://github.com/p1486/sdmw/releases)

## Usage

01. Create a new repository:
```
sdmw new /path/to/repository && cd /path/to/repository
```

02. Edit `settings.json`:
```diff
{
    "path": [
+       "~/.config/alacritty",
+       "~/.zshrc"
    ]
}
```

03. Add files to repository:
```
sdmw add
```

04. Install:
Create symbolic links.
```
sdmw install
```

To restore files:
```
cd /path/to/repository && sdmw restore alacritty/
```

### Options
- `-q`, `--quiet` - Do not print log messages.
- `-p`, `--pretend` - Prit what it would do but not actually change anything.
- `-h`, `--help` - Print help.
- `-V`, `--version` - Print version.

## License
This project is licensed under the MIT License and the Apache-2.0 license.
