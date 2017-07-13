# mpvc

An mpc-like cli tool for mpv which connects to existing mpv instances through sockets, written in Rust

This tool is inspired by mpc, a command line interface for the Music Player Daemon.
It uses subcommands to control any mpv instance over a given socket.

Make sure mpv is started with the following option:
`
$ mpv --input-ipc-server=/tmp/mpvsocket ...
`

## Dependencies

- `mpv`
- `cargo` (makedep)

## Install

- [Arch](https://aur.archlinux.org/packages/mpvc-rs) - `yaourt mpvc-rs-git`

If you have packaged mpvc for your distribution, let me know so I can add it here.

#### Manual Install

Use "cargo build --release" to build the program.
The output binary will be found in 'target/release/'

## Usage

Make sure mpv is started with the following option:
`
$ mpv --input-ipc-server=/tmp/mpvsocket --idle
`

At the moment mpvc does not launch mpv instances, so the instances have to be launched beforehand.
I'm not sure yet where to go with this project so this might change in the future.

#### mps-youtube

mpvc can be used with mps-youtube. In order to use mpvc with mpsyt you have to set the following option in mpsyt:

`set download_command mpvc playlist add --mode=append-play https://youtu.be/%i#%f`

Then use it as follows:

`da 1-5`

## BUGS / IDEAS

Check out the [Issue Tracker](https://gitlab.com/mpv-ipc/mpvc-rs/issues)