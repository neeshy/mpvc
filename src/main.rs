#[macro_use]
extern crate clap;

extern crate serde;
extern crate serde_json;
extern crate colored;

#[macro_use]
mod macros;
mod ipc;
mod commands;

use std::process::exit;

use clap::{AppSettings, Arg, SubCommand};
use colored::*;

use ipc::*;
use commands::*;

fn main() {

    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("socket")
            .short("S")
            .long("socket")
            .value_name("/path/to/socket")
            .help("Specifies the path to the socket")
            .default_value("/tmp/mpvsocket")
            .takes_value(true))
        .arg(Arg::with_name("list-options")
            .long("list-options")
            .help("Dummy option needed for mps-youtube")
            .hidden(true))
        .subcommand(SubCommand::with_name("get")
            .about("Gets information from mpv and prints them. See --help for available subcommands.")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("metadata")
                .about("Prints all metadata attributes of the currently playing file"))
            .subcommand(SubCommand::with_name("property")
                .about("<PROPERTY>\n\
                Retrieves a mpv property (see property 'property-list' for possible values)")
                .arg(Arg::with_name("property")
                    .help("Property that should be retrieved")
                    .takes_value(false)
                    .required(true))))
        .subcommand(SubCommand::with_name("set")
            .about("Set settings for mpv. See --help for available subcommands.")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("property")
                .about("<PROPERTY> <VALUE>\n\
                Sets a mpv property to <VALUE>")
                .arg(Arg::with_name("property")
                    .help("Property that should be set")
                    .takes_value(false)
                    .required(true))
                .arg(Arg::with_name("value")
                    .help("Value to be set")
                    .takes_value(false)
                    .required(true)))
            .subcommand(SubCommand::with_name("volume")
                .about("[OPTIONS] <NUM>\n\
                Sets the volume to <NUM> (0-100). \
                Use with --increase or --decrease to relatively change the volume")
                .arg(Arg::with_name("num")
                    .value_name("NUM")
                    .required(true))
                .arg(Arg::with_name("increase")
                    .short("i")
                    .long("increase")
                    .help("If set will increase volume by <NUM>")
                    .takes_value(false))
                .arg(Arg::with_name("decrease")
                    .short("d")
                    .long("decrease")
                    .help("If set will decrease volume by <NUM>")
                    .takes_value(false)
                    .conflicts_with("increase"))))
        .subcommand(SubCommand::with_name("pause")
            .about("Pauses playing"))
        .subcommand(SubCommand::with_name("toggle")
            .about("Toggles between play and pause. \
            If stopped starts playing. Does not support start playing \
            at song number (use play)."))
        .subcommand(SubCommand::with_name("next")
            .about("Starts playing next file on playlist"))
        .subcommand(SubCommand::with_name("prev")
            .about("Starts playing previous file on playlist"))
        .subcommand(SubCommand::with_name("restart")
            .about("Restarting playback of current file (same as 'seek -a 0')"))
        .subcommand(SubCommand::with_name("seek")
            .about("[OPTIONS] <NUM>\n\
            Change the playback position. By default, \
            seeks by a relative amount of seconds. Use -n for negative values. \
            See -h for more options.")
            .arg(Arg::with_name("num")
                .value_name("NUM")
                .required(true))
            .arg(Arg::with_name("relative")
                .short("r")
                .long("relative")
                .help("Seek relative to current position (a value with -n seeks backwards).")
                .takes_value(false))
            .arg(Arg::with_name("absolute")
                .short("a")
                .long("absolute")
                .help("Seek to a given time (a value with -n starts from the end of the file).")
                .takes_value(false)
                .conflicts_with_all(&["relative", "absolute-percent", "relative-percent"]))
            .arg(Arg::with_name("absolute-percent")
                .long("absolute-percent")
                .help("Seek to a given percent position.")
                .takes_value(false)
                .conflicts_with_all(&["relative", "absolute", "relative-percent"]))
            .arg(Arg::with_name("relative-percent")
                .long("relative-percent")
                .help("Seek relative to current position in percent.")
                .takes_value(false)
                .conflicts_with_all(&["relative", "absolute", "absolute-percent"]))
            .arg(Arg::with_name("negative")
                .short("n")
                .long("negative")
                .help("Use with negative values of <NUM>")
                .takes_value(false)))
        .subcommand(SubCommand::with_name("events")
            .about("Event related commands. See --help for available subcommands.")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("show")
                .about("Prints all mpv events in real-time."))
            .subcommand(SubCommand::with_name("wait-for")
                .about("<EVENT>\n\
                Runs until the mpv event <EVENT> is triggered.")
                .arg(Arg::with_name("event")
                    .value_name("event")
                    .required(true))))
        .subcommand(SubCommand::with_name("stop")
            .about("Stop playback and clear playlist."))
        .subcommand(SubCommand::with_name("playlist")
            .about("Playlist related commands. See --help for available subcommands")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("add")
                .about("[OPTIONS] <FILE>\n\
                Load the given file or playlist and play it. See -h for options.")
                .arg(Arg::with_name("file")
                    .value_name("FILE")
                    .required(true))
                .arg(Arg::with_name("mode")
                    .short("m")
                    .long("mode")
                    .possible_values(&["replace", "append", "append-play"])
                    .hide_possible_values(true)
                    .default_value("replace")
                    .help("<replace|append|append-play>\n\
                    <replace>: Stop playback of the current file, and play the new file immediately.\n\
                    <append>: Append the file to the playlist.\n\
                    <append-play>: Append the file, and if nothing is currently playing, start playback.\n\n")
                    .takes_value(true)))
            .subcommand(SubCommand::with_name("show")
                .about("Prints the 1-based playlist."))
            .subcommand(SubCommand::with_name("clear")
                .about("Clear the playlist, except the currently played file."))
            .subcommand(SubCommand::with_name("shuffle")
                .about("Shuffles the playlist"))
            .subcommand(SubCommand::with_name("remove-id")
                .about("Removes <ID> from the 0-based playlist. If <ID> is currently playing, playback will stop")
                .alias("remove")
                .arg(Arg::with_name("id")
                    .value_name("ID")
                    .help("Defines the id that should be removed from the 0-based playlist")
                    .required(true)))
            .subcommand(SubCommand::with_name("move-id")
                .about("Moves the playlist entry at position <FROM> to position <TO>")
                .alias("move")
                .arg(Arg::with_name("from")
                    .value_name("FROM-ID")
                    .help("Defines the id that should be moved to position <TO>")
                    .required(true))
                .arg(Arg::with_name("to")
                    .value_name("TO-ID")
                    .help("Defines the id where the entry at <FROM> should be moved to")
                    .required(true)))
            .subcommand(SubCommand::with_name("play-next")
                .about("Moves the playlist entry at position <ID> after the currently playing ID")
                .alias("next")
                .arg(Arg::with_name("id")
                    .value_name("ID")
                    .help("Defines the id that should be played next")
                    .required(true)))
            .subcommand(SubCommand::with_name("play-id")
                .about("Plays the file at <ID> in the playlist")
                .alias("play")
                .arg(Arg::with_name("id")
                    .value_name("ID")
                    .help("Defines the id that should be played next")
                    .required(true))))
        .get_matches();

    //Input socket is always present, therefore unwrap
    let mpv: Socket = String::from(matches.value_of("socket").unwrap());

    match matches.subcommand() {
        ("pause", _) => {
            if let Err(msg) = mpv.pause() {
                error!("Error: {}", msg);
            }
        }

        ("toggle", _) => {
            if let Err(msg) = mpv.toggle() {
                error!("Error: {}", msg);
            }
        }

        ("next", _) => {
            if let Err(msg) = mpv.next() {
                error!("Error: {}", msg);
            }
        }

        ("prev", _) => {
            if let Err(msg) = mpv.prev() {
                error!("Error: {}", msg);
            }
        }

        ("restart", _) => {
            if let Err(msg) = mpv.restart() {
                error!("Error: {}", msg);
            }
        }

        ("stop", _) => {
            if let Err(msg) = mpv.stop() {
                error!("Error: {}", msg);
            }
        }

        ("get", Some(get_matches)) => {
            match get_matches.subcommand() {
                ("property", Some(property_matches)) => {
                    let property = property_matches.value_of("property").unwrap();
                    match get_mpv_property_string(&mpv, property) {
                        Ok(value) => {
                            println!("{}", value);
                            exit(0);
                        }
                        Err(msg) => error!("Error: {}", msg),
                    }
                }

                ("metadata", _) => {
                    match mpv.get_metadata() {
                        Ok(metadata) => {
                            if metadata.len() == 0 {
                                println!("File has no metadata");
                            } else {
                                for (key, value) in metadata.iter() {
                                    println!("{}: {}", key, value);
                                }
                            }
                        }
                        Err(why) => error!("Error: {}", why),
                    }
                }

                (_, _) => unreachable!(),
            }
        }

        ("set", Some(set_matches)) => {
            match set_matches.subcommand() {
                ("property", Some(property_matches)) => {
                    let property = property_matches.value_of("property").unwrap();
                    let value = property_matches.value_of("value").unwrap();
                    if let Err(error_msg) = set_mpv_property(&mpv, property, value.to_string()) {
                        error!("Error: {}", error_msg);
                    }
                }

                ("volume", Some(volume_matches)) => {
                    let num = volume_matches.value_of("num").unwrap();
                    if volume_matches.is_present("increase") {
                        if let Err(msg) =
                            mpv.set_volume(num.parse::<f64>().unwrap(),
                                           VolumeChangeOptions::Increase) {
                            error!("Error: {}", msg);
                        }
                    } else if volume_matches.is_present("decrease") {
                        if let Err(msg) =
                            mpv.set_volume(num.parse::<f64>().unwrap(),
                                           VolumeChangeOptions::Decrease) {
                            error!("Error: {}", msg);
                        }
                    } else {
                        if let Err(msg) =
                            mpv.set_volume(num.parse::<f64>().unwrap(),
                                           VolumeChangeOptions::Absolute) {
                            error!("Error: {}", msg);
                        }
                    }
                }

                (_, _) => unreachable!(),
            }
        }

        ("seek", Some(seek_matches)) => {
            let num = seek_matches.value_of("num").unwrap();
            let mut n: f64 = num.parse().expect("Parse f64");
            if seek_matches.is_present("negative") {
                n = n * -1.0f64;
            }
            let n = n;
            if seek_matches.is_present("absolute") {
                if let Err(msg) = mpv.seek(n, SeekOptions::Absolute) {
                    error!("Error: {}", msg);
                }
            } else if seek_matches.is_present("absolute-percent") {
                if let Err(msg) = mpv.seek(n, SeekOptions::AbsolutePercent) {
                    error!("Error: {}", msg);
                }
            } else if seek_matches.is_present("relative-percent") {
                if let Err(msg) = mpv.seek(n, SeekOptions::RelativePercent) {
                    error!("Error: {}", msg);
                }
            } else if let Err(msg) = mpv.seek(n, SeekOptions::Relative) {
                error!("Error: {}", msg);
            }
        }

        ("events", Some(events_matches)) => {
            match events_matches.subcommand() {
                ("wait-for", Some(wait_for_matches)) => {
                    let event = wait_for_matches.value_of("event").unwrap();
                    wait_for_event(&mpv, event);
                }

                ("show", _) => {
                    listen(&mpv);
                }

                (_, _) => unreachable!(),
            }
        }

        ("playlist", Some(playlist_matches)) => {
            match playlist_matches.subcommand() {
                ("add", Some(add_matches)) => {
                    let file = add_matches.value_of("file").unwrap();
                    if add_matches.is_present("mode") {
                        match add_matches.value_of("mode").unwrap() {
                            "replace" => {
                                if let Err(msg) =
                                    mpv.playlist_add(file, PlaylistAddOptions::Replace) {
                                    error!("Error: {}", msg);
                                }
                            }

                            "append" => {
                                if let Err(msg) =
                                    mpv.playlist_add(file, PlaylistAddOptions::Append) {
                                    error!("Error: {}", msg);
                                }
                            }

                            "append-play" => {
                                if let Err(msg) =
                                    mpv.playlist_add(file, PlaylistAddOptions::AppendPlay) {
                                    error!("Error: {}", msg);
                                }
                            }

                            _ => unreachable!(),
                        }
                    }
                }

                ("shuffle", _) => {
                    if let Err(msg) = mpv.playlist_shuffle() {
                        error!("Error: {}", msg);
                    }
                }

                ("clear", _) => {
                    if let Err(msg) = mpv.playlist_clear() {
                        error!("Error: {}", msg);
                    }
                }

                ("remove-id", Some(remove_id_matches)) => {
                    if let Err(msg) = mpv.playlist_remove_id(remove_id_matches
                                                                 .value_of("id")
                                                                 .unwrap()
                                                                 .parse::<usize>()
                                                                 .unwrap()) {
                        error!("Error: {}", msg);
                    }
                }

                ("move-id", Some(move_id_matches)) => {
                    if let Err(msg) = mpv.playlist_move_id(move_id_matches
                                                               .value_of("from")
                                                               .unwrap()
                                                               .parse::<usize>()
                                                               .unwrap(),
                                                           move_id_matches
                                                               .value_of("to")
                                                               .unwrap()
                                                               .parse::<usize>()
                                                               .unwrap()) {
                        error!("Error: {}", msg);
                    }
                }

                ("play-next", Some(play_next_matches)) => {
                    if let Err(msg) = mpv.playlist_play_next(play_next_matches
                                                                 .value_of("id")
                                                                 .unwrap()
                                                                 .parse::<usize>()
                                                                 .unwrap()) {
                        error!("Error: {}", msg);
                    }
                }

                ("play-id", Some(play_id_matches)) => {
                    if let Err(msg) = mpv.playlist_play_id(play_id_matches
                                                               .value_of("id")
                                                               .unwrap()
                                                               .parse::<usize>()
                                                               .unwrap()) {
                        error!("Error: {}", msg);
                    }
                }

                ("show", _) => {
                    //Show the playlist
                    if let Ok(playlist) = mpv.get_playlist() {
                        for entry in playlist.entries.iter() {
                            if &entry.title == "" {
                                let mut output = format!("{}\t{}", entry.id, entry.filename);
                                if entry.current {
                                    output = format!("{}", output.reverse());
                                }
                                println!("{}", output);
                            } else {
                                let mut output = format!("{}\t{}", entry.id, entry.title);
                                if entry.current {
                                    output = format!("{}", output.reverse());
                                }
                                println!("{}", output);
                            }
                        }
                    }
                }

                (_, _) => unreachable!(),
            }
        }

        (_, _) => unreachable!(),
    }
}