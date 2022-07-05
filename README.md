![mpvc screenshot](https://gitlab.com/mpv-ipc/mpvc/raw/master/res/logo.png "logo")
# mpvc

An mpc-like cli tool for mpv which connects to existing mpv instances through sockets, written in Rust

This tool is inspired by mpc, a command line interface for the Music Player Daemon.
It uses subcommands to control any mpv instance over a given socket.

Make sure mpv is started with the following option:
`
$ mpv --input-ipc-server=/tmp/mpv.sock ...
`

## Dependencies

- `mpv`
- `cargo` (makedep)

## Install

- Arch ([AUR](https://aur.archlinux.org/packages/mpvc-rs)) - `yay mpvc-rs`
- Gentoo ([GURU](https://github.com/gentoo/guru/tree/master/media-sound/mpvc) - `emerge -av media-sound/mpvc`

If you have packaged mpvc for your distribution, let me know so I can add it here.

#### Manual Install

Use "cargo build --release" to build the program.
The output binary will be found in 'target/release/'

## Usage

Make sure mpv is started with the following option:
`
$ mpv --input-ipc-server=/tmp/mpv.sock --idle
`

At the moment mpvc does not launch mpv instances, so the instances have to be launched beforehand.
I'm not sure yet where to go with this project so this might change in the future.

## BUGS / IDEAS

Check out the [Issue Tracker](https://gitlab.com/mpv-ipc/mpvc/issues)
