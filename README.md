# SDMW - A Simple Dotfiles Manager Written in Rust

![workflow_build](https://github.com/p1486/sdmw/actions/workflows/build.yml/badge.svg)
![workflow_test](https://github.com/p1486/sdmw/actions/workflows/test.yml/badge.svg)
![Crates.io Version](https://img.shields.io/crates/v/sdmw)

## v0.4.0
Changed behavior of a subcommand `add`

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

1. Create a new repository:
```
sdmw new /path/to/repository && cd /path/to/repository
```

2. Edit `settings.json`:
```diff
{
    "path": [
+       "~/.config/alacritty",
+       "~/.zshrc"
    ]
}
```
And run:
```
sdmw add
```
Alternatively:
```
sdmw add ~/.config/alacritty ~/.zshrc
```

3. Install:
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

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
