# mpvipc

An mpc-like cli tool for mpv which connects to existing mpv instances through sockets, written in Rust

This tool is inspired by mpc, a command line interface for the Music Player Daemon.
It uses subcommands to control any mpv instance over a given socket.

Make sure mpv is started with the following option:
`
$ mpv --input-ipc-server=/tmp/mpvsocket song.flac
`

## Dependencies

- `mpv`
- `socat` 
- `cargo` (makedep)

## Install

This is work in progress. Therefore there are no packages for any distribution at the moment

#### Manual Install

Use "cargo build --release" to build the program.
The output binary will be found in 'target/release/mpvipc'

## Usage

Make sure mpv is started with the following option:
`
$ mpv --input-ipc-server=/tmp/mpvsocket
`

At the moment mpvipc does not launch mpv instances, so the instances have to be launched beforehand.
I'm not sure yet where to go with this project so this might change in the future.

## BUGS / IDEAS

Check out the [Issue Tracker](https://github.com/freijon/mpvipc/issues)