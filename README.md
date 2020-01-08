# waffy
Wayland compatible, touch friendly application launcher

![waffy-screenshot](https://i.imgur.com/iyEQVqo.png)

## Features
- sway dedicated launcher
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
cmake -Bcmake-build-release -H. -DCMAKE_BUILD_TYPE=Release
cmake --build cmake-build-release --target all
```

### Add to path
```shell script
echo "export PATH=$HOME/.local/share/waffy:$PATH" >> $HOME/.bash_profile
# or
sudo ln -s $HOME/.local/share/waffy/waffy /usr/local/bin/waffy
```



## Other information
- [Studies project info](/docs/project.md)
