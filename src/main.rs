#[macro_use]
extern crate clap;

extern crate serde;
extern crate serde_json;
extern crate colored;

#[macro_use]
mod macros;
mod ipc;

use std::process::exit;
use std::collections::HashMap;

use clap::{AppSettings, Arg, SubCommand};
use colored::*;

use ipc::*;

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
        .arg(Arg::with_name("list_options")
            .long("list_options")
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
                    .value_name("num")
                    .required(true))
                .arg(Arg::with_name("increase")
                    .short("i")
                    .long("increase")
                    .help("If set will increase volume by <num>")
                    .takes_value(false))
                .arg(Arg::with_name("decrease")
                    .short("d")
                    .long("decrease")
                    .help("If set will decrease volume by <num>")
                    .takes_value(false))))
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
                .takes_value(false))
            .arg(Arg::with_name("absolute-percent")
                .long("absolute-percent")
                .help("Seek to a given percent position.")
                .takes_value(false))
            .arg(Arg::with_name("relative-percent")
                .long("relative-percent")
                .help("Seek relative to current position in percent.")
                .takes_value(false))
            .arg(Arg::with_name("negative")
                .short("n")
                .long("negative")
                .help("Use with negative values of <num>")
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
                .arg(Arg::with_name("id")
                    .value_name("ID")
                    .help("Defines the id that should be removed from the 0-based playlist")
                    .required(true)))
            .subcommand(SubCommand::with_name("move-id")
                .about("Moves the playlist entry at position <FROM> to position <TO>")
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
                .arg(Arg::with_name("id")
                    .value_name("ID")
                    .help("Defines the id that should be played next")
                    .required(true))))
        .get_matches();

    //Input socket is always present, therefore unwrap
    let socket = matches.value_of("socket").unwrap();

    if let Some(submatches) = matches.subcommand_matches("get") {
        if let Some(ssm) = submatches.subcommand_matches("property") {
            let property = ssm.value_of("property").unwrap();
            match get_mpv_property_string(socket, property) {
                Ok(value) => {
                    println!("{}", value);
                    exit(0);
                }
                Err(msg) => error!("Error: {}", msg),
            }
        } else if let Some(_) = submatches.subcommand_matches("metadata") {
            let metadata: HashMap<String, String> = get_mpv_property(socket, "metadata").unwrap();
            for (key, value) in metadata.iter() {
                println!("{}: {}", key, value);
            }

        }
        exit(0);
    }

    if let Some(submatches) = matches.subcommand_matches("set") {
        if let Some(ssm) = submatches.subcommand_matches("property") {
            let property = ssm.value_of("property").unwrap();
            let value = ssm.value_of("value").unwrap();
            if let Err(error_msg) = set_mpv_property(socket, property, value.to_string()) {
                error!("Error: {}", error_msg);
            }
        } else if let Some(ssm) = submatches.subcommand_matches("volume") {
            let num = ssm.value_of("num").unwrap();
            if ssm.is_present("increase") || ssm.is_present("decrease") {
                match get_mpv_property::<f64>(socket, "volume") {
                    Ok(volume) => {
                        if ssm.is_present("increase") {
                            if let Err(error_msg) = set_mpv_property(socket,
                                                                     "volume",
                                                                     volume +
                                                                     num.parse::<f64>().unwrap()) {
                                error!("Error: {}", error_msg);
                            }
                        } else {
                            if let Err(error_msg) = set_mpv_property(socket,
                                                                     "volume",
                                                                     volume -
                                                                     num.parse::<f64>().unwrap()) {
                                error!("Error: {}", error_msg);
                            }
                        }
                    }
                    Err(msg) => error!("Error: {}", msg),
                }
            } else {
                if let Err(error_msg) = set_mpv_property(socket, "volume", num.to_string()) {
                    error!("Error: {}", error_msg);
                }
            }
        }
        exit(0);
    }

    if let Some(_) = matches.subcommand_matches("pause") {
        if let Err(error_msg) = set_mpv_property(socket, "pause", true) {
            error!("Error: {}", error_msg);
        }
    }

    if let Some(_) = matches.subcommand_matches("toggle") {
        match get_mpv_property::<bool>(socket, "pause") {
            Ok(paused) => {
                if let Err(error_msg) = set_mpv_property(socket, "pause", !paused) {
                    error!("Error: {}", error_msg);
                }
            }
            Err(msg) => error!("Error: {}", msg),
        }
        exit(0);
    }

    if let Some(_) = matches.subcommand_matches("next") {
        if let Err(error_msg) = run_mpv_command(socket, "playlist-next", &vec![]) {
            error!("Error: {}", error_msg);
        }
        exit(0);
    }

    if let Some(_) = matches.subcommand_matches("prev") {
        if let Err(error_msg) = run_mpv_command(socket, "playlist-prev", &vec![]) {
            error!("Error: {}", error_msg);
        }
        exit(0);
    }

    if let Some(_) = matches.subcommand_matches("restart") {
        if let Err(error_msg) = run_mpv_command(socket, "seek", &vec!["0", "absolute"]) {
            error!("Error: {}", error_msg);
        }
        exit(0);
    }

    if let Some(submatches) = matches.subcommand_matches("seek") {
        let num = submatches.value_of("num").unwrap();
        let mut n = num.to_string();
        if submatches.is_present("negative") {
            n = format!("-{}", num);
        }
        n = n;
        if submatches.is_present("absolute") {
            if let Err(error_msg) = run_mpv_command(socket, "seek", &vec![&n, "absolute"]) {
                error!("Error: {}", error_msg);
            }
            exit(0);
        }
        if submatches.is_present("absolute-percent") {
            if let Err(error_msg) = run_mpv_command(socket, "seek", &vec![&n, "absolute-percent"]) {
                error!("Error: {}", error_msg);
            }
            exit(0);
        }
        if submatches.is_present("relative-percent") {
            if let Err(error_msg) = run_mpv_command(socket, "seek", &vec![&n, "relative-percent"]) {
                error!("Error: {}", error_msg);
            }
            exit(0);
        }
        if let Err(error_msg) = run_mpv_command(socket, "seek", &vec![&n, "relative"]) {
            error!("Error: {}", error_msg);
        }
        exit(0);
    }

    if let Some(submatches) = matches.subcommand_matches("events") {
        if let Some(ssm) = submatches.subcommand_matches("wait-for") {
            let event = ssm.value_of("event").unwrap();
            wait_for_event(socket, event);
        } else if let Some(_) = submatches.subcommand_matches("show") {
            listen(socket);
        }
        exit(0);
    }

    if let Some(_) = matches.subcommand_matches("stop") {
        if let Err(error_msg) = run_mpv_command(socket, "stop", &vec![]) {
            error!("Error: {}", error_msg);
        }
        exit(0);
    }

    if let Some(submatches) = matches.subcommand_matches("wait-for-event") {
        let event = submatches.value_of("event").unwrap();
        wait_for_event(socket, event);
        exit(0);
    }

    if let Some(submatches) = matches.subcommand_matches("playlist") {
        if let Some(ssm) = submatches.subcommand_matches("add") {
            let file = ssm.value_of("file").unwrap();
            if let Err(error_msg) = run_mpv_command(socket,
                                                    "loadfile",
                                                    &vec![file, ssm.value_of("mode").unwrap()]) {
                error!("Error: {}", error_msg);
            }
        } else if let Some(_) = submatches.subcommand_matches("shuffle") {
            if let Err(error_msg) = run_mpv_command(socket, "playlist-shuffle", &vec![]) {
                error!("Error: {}", error_msg);
            }
        } else if let Some(_) = matches.subcommand_matches("clear") {
            if let Err(error_msg) = run_mpv_command(socket, "playlist-clear", &vec![]) {
                error!("Error: {}", error_msg);
            }
        } else if let Some(ssm) = submatches.subcommand_matches("remove") {
            if let Err(error_msg) = run_mpv_command(socket,
                                                    "playlist-remove",
                                                    &vec![ssm.value_of("num").unwrap()]) {
                error!("Error: {}", error_msg);
            }
        } else if let Some(ssm) = submatches.subcommand_matches("move") {
            if let Err(error_msg) = run_mpv_command(socket,
                                                    "playlist-move",
                                                    &vec![ssm.value_of("from").unwrap(),
                                                          ssm.value_of("to").unwrap()]) {
                error!("Error: {}", error_msg);
            }
        } else if let Some(ssm) = submatches.subcommand_matches("play-next") {
            match Playlist::get_from(socket) {
                Ok(playlist) => {
                    if let Some(current_id) = playlist.current_id() {
                        if let Err(error_msg) = run_mpv_command(socket,
                                                                "playlist-move",
                                                                &vec![ssm.value_of("id")
                                                                          .unwrap(),
                                                                      &format!("{}",
                                                                               current_id + 1)]) {
                            error!("Error: {}", error_msg);
                        }
                    }
                }
                Err(why) => error!("Error: Could not set play-next: {}", why),
            }
        } else if let Some(_) = submatches.subcommand_matches("show") {
            //Show the playlist
            if let Ok(playlist) = Playlist::get_from(socket) {
                for entry in playlist.entries.iter() {
                    if &entry.title == "" {
                        let mut output = format!("{}\t{}", entry.id + 1, entry.filename);
                        if entry.current {
                            output = format!("{}", output.reverse());
                        }
                        println!("{}", output);
                    } else {
                        let mut output = format!("{}\t{}", entry.id + 1, entry.title);
                        if entry.current {
                            output = format!("{}", output.reverse());
                        }
                        println!("{}", output);
                    }
                }
            }
        }
        exit(0);
    }
}