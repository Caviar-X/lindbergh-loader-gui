## Lindbergh Loader GUI

This is a GUI for [lindbergh-loader](https://github.com/lindbergh-loader/lindbergh-loader)

**Main Page**

![](./main_look.png)

**Game Configuration Page**

![](./config_game_look.png)

**Keymap Configuration Page**

![](./config_keymap_look.png)

## Pre-Install Requirements

Rust nightly toolchain is needed to compile this project (specified in [rust-toolchain.toml](./rust-toolchain.toml))

for how to install rust toolchain on your computer,see [rustup](https://rustup.rs/)

For graphical dependecies:

- **Desktop Users**: Basically nothing is required


-  **WSL2 Users / Users without desktop environment**: at least one implementation of `xdg-desktop-portal` (for file picker) and X11/Wayland with some desktop environment package (such as libadwaita,etc.)

NOTE: I'm not very sure about non-desktop users' dependencies,but basically it'll work?

## Installation

1. [follow the guide on lindbergh-loader](https://github.com/lindbergh-loader/lindbergh-loader) and build the dynamic library.

2. clone or download this project,`cd` into the project's directory and run
```sh
cargo build --release
```
then you can found the executable named `loader-gui` in `./target/release`,move it into anywhere you like.

3. move the `assets` directory into the same-level directory as the executable

4. create a directory named `dynlibs` in the same-level directory as the executable,move everything from `/libs` and `/build` in lindbergh-loader directory into `dynlibs`.

## Troubleshooting

Find your game's stdout/stderr output in `./log` directory

## Known Issue

```
Io error: Broken pipe (os error 32)
Io error: Broken pipe (os error 32)
Io error: Broken pipe (os error 32)
Error: WinitEventLoop(ExitFailure(1))
```
NOTE: So far this only happens on wsl2

**Recreation:** No idea how to trigger this.

**Solution:** just rerun a few more times and it works???

## TODOs

- [ ] identitfy games from .elf
- [ ] Import from existing lindbergh.conf
- [ ] i18n
- [ ] more themes
- [ ] Automatically scaling
- [ ] Automatic fetcher / updater of lindbergh-loader

**If you have any ideas,open an issue.**