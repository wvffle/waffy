# waffy
Wayland compatible, touch friendly application launcher

![waffy-screenshot](https://i.imgur.com/iyEQVqo.png)

## Features
- wlroots dedicated launcher
- Touch support
- Read all application locations
- Respect hidden apps
- Fuzzy find apps

## Requirements
- [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell) - A library to write GTK applications that use Layer Shell

## Building
### Build with git and cmake
```shell script
git clone https://github.com/wvffle/waffy.git
cd waffy
cargo build --release
```
