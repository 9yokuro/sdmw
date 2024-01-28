# SDMW - A Simple Dotfiles Manager Written in Rust

## Installation
Run the following Cargo command:
```
cargo install sdmw
```
Or download prebuilt binary from the [GitHub release page](https://github.com/p1486/sdmw/releases)

## Usage

01. Create a new repository:
```
sdmw new new_repository_name && cd new_repository_name
```

02. Edit `settings.json`:
```diff
{
    "path": [
        + "~/.config/alacritty",
        + "~/.zshrc"
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
```diff
{
    "path": [
        "~/.config/alacritty",
        - "~/.zshrc"
    ]
}
```
and move files to where they came from:
```
mv repository_name/.zshrc ~/.zshrc
```

### Options

## License
This project is licensed under the MIT License and the Apache-2.0 license.
