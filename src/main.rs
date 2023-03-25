extern crate mpvc;

extern crate clap;
extern crate colored;
extern crate serde_json;

#[macro_use]
mod macros;

use std::process::Command as Cmd;
use std::thread;
use std::time;

use mpvc::*;

use clap::{Arg, Command};
use colored::Colorize;
use serde_json::{Map, Value};

fn main() -> Result<(), Error> {
    let matches = Command::new(env!("CARGO_CRATE_NAME"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(Arg::new("socket")
            .short('S')
            .long("socket")
            .value_name("/path/to/socket")
            .help("Specifies the path to the socket")
            .default_value("/tmp/mpv.sock")
            .num_args(1))
        .subcommand(Command::new("get")
            .about("Gets information from mpv and prints them. See --help for available subcommands.")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("property")
                .about("<PROPERTY>\n\
                Retrieves a mpv property (see property 'property-list' for possible values)")
                .arg(Arg::new("property")
                    .help("Property that should be retrieved")
                    .required(true)))
            .subcommand(Command::new("metadata")
                .about("<METADATA>\n\
                Retrieves a metadata attribute from the currently playing file")
                .arg(Arg::new("metadata")
                    .help("Metadata attribute that should be retrieved")
                    .required(true))))
        .subcommand(Command::new("set")
            .about("Set settings for mpv. See --help for available subcommands.")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("property")
                .about("<PROPERTY> <VALUE>\n\
                Sets a mpv property to <VALUE>")
                .arg(Arg::new("property")
                    .help("Property that should be set")
                    .required(true))
                .arg(Arg::new("value")
                    .help("Value to be set")
                    .required(true)))
            .subcommand(Command::new("mute")
                .about("<ON|OFF|TOGGLE>\n\
                Controls whether audio output is muted.")
                .arg(Arg::new("arg")
                    .value_name("on|off|toggle")
                    .value_parser(["on", "off", "toggle"])
                    .help("Defines if muting audio is enabled or disabled")
                    .required(true)))
            .subcommand(Command::new("volume")
                .about("[OPTIONS] <NUM>\n\
                Sets the volume to <NUM> (0-100). \
                Use with --mode relative to increase or decrease the volume")
                .arg(Arg::new("num")
                    .value_name("NUM")
                    .required(true))
                .arg(Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .value_parser(["absolute", "relative"])
                    .hide_possible_values(true)
                    .default_value("absolute")
                    .help("<absolute|relative>\n\
                    <absolute>: Set the volume.\n\
                    <relative>: Change the volume relatively. To decrease, use negative values for <NUM>.\n\n")
                    .num_args(1)))
            .subcommand(Command::new("loop-file")
                .about("<ON|OFF|TOGGLE>\n\
                Controls whether the current file should be repeatet after playback.")
                .arg(Arg::new("arg")
                    .value_name("on|off|toggle")
                    .value_parser(["on", "off", "toggle"])
                    .help("Defines if looping current file is enabled or disabled")
                    .required(true)))
            .subcommand(Command::new("loop-playlist")
                .about("<ON|OFF|TOGGLE>\n\
                Controls whether the playlist should be repeatet after the end is reached.")
                .arg(Arg::new("arg")
                    .value_name("on|off|toggle")
                    .value_parser(["on", "off", "toggle"])
                    .help("Defines if looping playlist is enabled or disabled")
                    .required(true))))
        .subcommand(Command::new("play")
            .about("Resumes playing"))
        .subcommand(Command::new("pause")
            .about("Pauses playing"))
        .subcommand(Command::new("toggle")
            .about("Toggles between play and pause. \
            If stopped starts playing. Does not support start playing \
            at song number (use play)."))
        .subcommand(Command::new("next")
            .about("Starts playing next file on playlist"))
        .subcommand(Command::new("prev")
            .about("Starts playing previous file on playlist"))
        .subcommand(Command::new("restart")
            .about("Restarting playback of current file (same as 'seek -a 0')"))
        .subcommand(Command::new("kill")
            .about("Kill the mpv process controlling the socket"))
        .subcommand(Command::new("format")
                .about("Replaces the following arguments with their real-time values from mpv. \n\
                        Use with \"\" for multiple arguments.\n\n\
                        %filename%, %path%, %title%, %artist%, %album%, %album_artist%, %composer%, %genre%, %year%, %comment%, \
                        %track%, %disc%, %playlistlength%, %position%\n\n\
                        EXAMPLE: format \"%artist% - %title% (%album%)\"")
                .arg(Arg::new("input")
                    .value_name("INPUT-STRING")
                    .help("%filename%: \n\
                        %path%: \n\
                        %title%: \n\
                        %artist%: \n\
                        %album%: \n\
                        %album_artist%: \n\
                        %composer%: \n\
                        %genre%: \n\
                        %year%: \n\
                        %comment%: \n\
                        %track%: \n\
                        %disc%: \n\
                        %playlistlength%: \n\
                        %position%: ")
                    .required(true)))
        .subcommand(Command::new("seek")
            .about("[OPTIONS] <NUM>\n\
            Change the playback position. By default, \
            seeks by a relative amount of seconds. You may use negative values. \
            See -h for more options.")
            .arg(Arg::new("num")
                .value_name("NUM")
                .required(true))
            .arg(Arg::new("mode")
                .short('m')
                .long("mode")
                .value_parser(["relative", "relative-percent", "absolute", "absolute-percent"])
                .hide_possible_values(true)
                .default_value("relative")
                .help("<relative|relative-percent|absolute|absolute-percent>\n\
                <relative>: Seek relative to current position (a negative value seeks backwards).\n\
                <absolute>: Seek to a given time (a negataive value starts from the end of the file).\n\
                <absolute-percent>: Seek to a given percent position.\n\
                <relative-percent>: Seek relative to current position in percent.\n\n")
                .num_args(1)))
        .subcommand(Command::new("events")
            .about("Event related commands. See --help for available subcommands.")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("observe")
                .about("<PROPERTIES>\n\
                Prints all mpv events in real-time. Additionally, observes a set of properties and informs about changes.")
                .arg(Arg::new("property")
                    .value_name("PROPERTY")
                    .num_args(0..)))
            .subcommand(Command::new("wait-for")
                .about("<EVENT>\n\
                Runs until the mpv event <EVENT> is triggered, or until the mpv property <PROPERTY> is changed.
                See --help for possible events.")
                .arg_required_else_help(true)
                .arg(Arg::new("event")
                    .value_name("EVENT")
                    .value_parser([
                        "start-file",
                        "end-file",
                        "file-loaded",
                        "seek",
                        "playback-restart",
                        "shutdown",
                        "video-reconfig",
                        "audio-reconfig"])
                    .num_args(1..))
                .arg(Arg::new("property")
                    .value_name("PROPERTY")
                    .num_args(1..)
                    .last(true))))
        .subcommand(Command::new("stop")
            .about("Stop playback and clear playlist."))
        .subcommand(Command::new("playlist")
            .about("Playlist related commands. See --help for available subcommands")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("add")
                .about("[OPTIONS] <FILE|PLAYLIST>\n\
                Load the given file or playlist and play it. See -h for options.")
                .visible_alias("load")
                .arg(Arg::new("file")
                    .value_name("FILE")
                    .num_args(1..)
                    .required(true))
                .arg(Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .value_parser(["replace", "append", "append-play"])
                    .hide_possible_values(true)
                    .default_value("append-play")
                    .help("<replace|append|append-play>\n\
                    <replace>: Stop playback of the current file, and play the new file immediately.\n\
                    <append>: Append the file to the playlist.\n\
                    <append-play>: Append the file, and if nothing is currently playing, start playback.\n\n")
                    .num_args(1))
                .arg(Arg::new("type")
                    .short('t')
                    .long("type")
                    .value_parser(["file", "playlist"])
                    .default_value("file")))
            .subcommand(Command::new("show")
                .about("Prints the playlist."))
            .subcommand(Command::new("clear")
                .about("Clear the playlist, except the currently played file."))
            .subcommand(Command::new("shuffle")
                .about("Shuffles the playlist"))
            .subcommand(Command::new("remove")
                .about("Removes <ID> from the 0-based playlist. If <ID> is currently playing, playback will stop")
                .arg(Arg::new("id")
                    .value_name("ID")
                    .help("Defines the id that should be removed from the 0-based playlist")
                    .required(true)))
            .subcommand(Command::new("move")
                .about("Moves the playlist entry at position <FROM> to position <TO>")
                .arg(Arg::new("from")
                    .value_name("FROM")
                    .help("Defines the id that should be moved to position <TO>")
                    .required(true))
                .arg(Arg::new("to")
                    .value_name("TO")
                    .help("Defines the id where the entry at <FROM> should be moved to")
                    .required(true)))
            .subcommand(Command::new("play-next")
                .about("Moves the playlist entry at position <ID> after the currently playing ID")
                .visible_alias("next")
                .arg(Arg::new("id")
                    .value_name("ID")
                    .help("Defines the id that should be played next")
                    .required(true)))
            .subcommand(Command::new("play")
                .about("Plays the file at <ID> in the playlist")
                .arg(Arg::new("id")
                    .value_name("ID")
                    .help("Defines the id that should be played next")
                    .required(true)))
            .subcommand(Command::new("reverse")
                .about("Reverses the playlist")
                .visible_alias("rev")))
        .get_matches();

    // Input socket is always present, therefore unwrap
    let socket = matches.get_one::<String>("socket").unwrap();
    let mut mpv = match Mpv::connect(socket) {
        Ok(instance) => instance,
        Err(msg) => match matches.subcommand() {
            Some(("playlist", playlist_matches)) => match playlist_matches.subcommand() {
                Some(("add", _)) => {
                    Cmd::new("mpv")
                        .args(["--really-quiet",
                               "--idle=once",
                               "--vid=no",
                               &("--input-ipc-server=".to_string() + socket)])
                        .spawn()
                        .expect("mpv failed to start");
                    thread::sleep(time::Duration::from_millis(500));
                    Mpv::connect(socket)?
                }
                _ => error!("Error: {}", msg),
            },
            _ => error!("Error: {}", msg),
        },
    };

    match matches.subcommand() {
        Some(("play", _)) => mpv.set_property("pause", false)?,
        Some(("pause", _)) => mpv.set_property("pause", true)?,
        Some(("toggle", _)) => {
            let pause = mpv.get_property("pause")?.as_bool().unwrap();
            mpv.set_property("pause", !pause)?;
        }
        Some(("next", _)) => mpv.command("playlist-next")?,
        Some(("prev", _)) => mpv.command("playlist-prev")?,
        Some(("restart", _)) => mpv.command_arg("seek", &["0", "absolute"])?,
        Some(("stop", _)) => mpv.command("stop")?,
        Some(("kill", _)) => mpv.command("quit")?,

        Some(("format", format_matches)) => {
            fn eval_format(mpv: &mut Mpv, metadata: &Map<String, Value>, key: &str) -> Option<String> {
                macro_rules! fmt_metadata {
                    ($key:expr) => {
                        if key == $key {
                            return Some(metadata.get($key)?.as_str()?.to_string());
                        }
                    };
                }

                macro_rules! fmt_property {
                    ($property:expr) => {
                        if key == $property {
                            return Some(mpv.get_property($property).ok()?.as_str()?.to_string());
                        }
                    };
                }

                if key == "title" {
                    if metadata.contains_key("title") {
                        return Some(metadata["title"].as_str()?.to_string());
                    } else {
                        return Some(mpv.get_property("media-title").ok()?.as_str()?.to_string());
                    }
                }

                fmt_metadata!("artist");
                fmt_metadata!("album");
                fmt_metadata!("album_artist");
                fmt_metadata!("date");
                fmt_metadata!("track");
                fmt_metadata!("genre");
                fmt_metadata!("composer");
                fmt_metadata!("comment");
                fmt_metadata!("disc");
                fmt_property!("path");
                fmt_property!("filename");

                if key == "position" {
                    let position = mpv.get_property("playlist-pos").ok()?.as_u64()?;
                    return Some((position + 1).to_string());
                }

                if key == "playlistlength" {
                    return mpv.get_property_string("playlist-count").ok();
                }

                None
            }

            let input = format_matches.get_one::<String>("input").unwrap();
            let property = mpv.get_property("metadata")?;
            let metadata = property.as_object().unwrap();
            // Manually parse the format string instead of naively doing repeated search
            // and replace operations. This is (most likely) more performant (the entire
            // string can be parsed in one pass instead of multiple), and issues with
            // "double replacements" are avoided. e.g. If the format string is "%title%"
            // and the title metadata in turn contains a valid format string (say "%path%",
            // unlikely but possible), the resulting output will be incorrect.
            let mut output = String::with_capacity(input.len());
            let mut s = 0usize;
            loop {
                let sub = &input[s..];
                match sub.find('%') {
                    Some(f) => {
                        let subf = &sub[f + 1..];
                        match subf.find('%') {
                            Some(e) => {
                                output += &sub[..f];
                                let fmt = &subf[..e];
                                match eval_format(&mut mpv, metadata, fmt) {
                                    Some(m) => {
                                        output += &m;
                                        // If the format string is valid, the
                                        // starting index should be iterated past
                                        // the ending '%'
                                        s += f + e + 2;
                                    }
                                    None => {
                                        // If this was not a valid format string, set the index to
                                        // the ending '%'. This is needed in case of unbalanced %'s
                                        // e.g. "100% Orange Juice %filename%"
                                        // This string will produce these iterations:
                                        //   sub[..f] == "100", fmt == " Orange Juice "
                                        //   fmt == "filename"
                                        output += "%";
                                        output += fmt;
                                        s += f + e + 1;
                                    }
                                }
                            }
                            None => {
                                output += sub;
                                break;
                            }
                        }
                    }
                    None => {
                        output += sub;
                        break;
                    }
                }
            }
            println!("{}", output);
        }

        Some(("get", get_matches)) => match get_matches.subcommand() {
            Some(("property", property_matches)) => {
                let property = property_matches.get_one::<String>("property").unwrap();
                let value = mpv.get_property_string(property)?;
                println!("{}", value);
            }

            Some(("metadata", metadata_matches)) => {
                let attribute = metadata_matches.get_one::<String>("metadata").unwrap();
                let property = mpv.get_property("metadata")?;
                let metadata = property.as_object().unwrap();
                if let Some(value) = metadata.get(attribute) {
                    if let Value::String(ref v) = value {
                        println!("{}", v);
                    } else {
                        println!("{:?}", value);
                    }
                } else {
                    error!("Error: MpvError: metadata attribute not found");
                }
            }

            _ => unreachable!(),
        },

        Some(("set", set_matches)) => match set_matches.subcommand() {
            Some(("property", property_matches)) => {
                let property = property_matches.get_one::<String>("property").unwrap();
                let value = property_matches.get_one::<String>("value").unwrap();
                mpv.set_property(property, value)?;
            }

            Some(("mute", mute_matches)) => {
                let switch_type = match mute_matches.get_one::<String>("arg").unwrap().as_str() {
                    "on" => true,
                    "off" => false,
                    "toggle" => !mpv.get_property("mute")?.as_bool().unwrap(),
                    _ => unreachable!(),
                };
                mpv.set_property("mute", switch_type)?;
            }

            Some(("volume", volume_matches)) => {
                let num = volume_matches.get_one::<String>("num").unwrap().parse::<f64>().unwrap();
                match volume_matches.get_one::<String>("mode").unwrap().as_str() {
                    "absolute" => mpv.set_property("volume", num)?,
                    "relative" => {
                        let volume = mpv.get_property("volume")?.as_f64().unwrap();
                        mpv.set_property("volume", volume + num)?;
                    }
                    _ => unreachable!(),
                }
            }

            Some(("loop-file", loop_file_matches)) => {
                let switch_type = match loop_file_matches.get_one::<String>("arg").unwrap().as_str() {
                    "on" => true,
                    "off" => false,
                    "toggle" => matches!(mpv.get_property_string("loop-file")?.as_str(), "false"),
                    _ => unreachable!(),
                };
                mpv.set_property("loop-file", switch_type)?;
            }

            Some(("loop-playlist", loop_playlist_matches)) => {
                let switch_type = match loop_playlist_matches.get_one::<String>("arg").unwrap().as_str() {
                    "on" => true,
                    "off" => false,
                    "toggle" => matches!(mpv.get_property_string("loop-playlist")?.as_str(), "false"),
                    _ => unreachable!(),
                };
                mpv.set_property("loop-playlist", switch_type)?;
            }

            _ => unreachable!(),
        },

        Some(("seek", seek_matches)) => {
            let num = seek_matches.get_one::<String>("num").unwrap();
            let seek_option = seek_matches.get_one::<String>("mode").unwrap().as_str();
            mpv.command_arg("seek", &[num, seek_option])?
        }

        Some(("events", events_matches)) => match events_matches.subcommand() {
            Some(("wait-for", wait_for_matches)) => {
                let watched_events = match wait_for_matches.get_many::<String>("event") {
                    Some(event_values) => event_values.collect::<Vec<&String>>(),
                    None => Vec::new(),
                };
                let watched_properties = match wait_for_matches.get_many::<String>("property") {
                    Some(property_values) => property_values.collect::<Vec<&String>>(),
                    None => Vec::new(),
                };
                for (i, property) in watched_properties.iter().enumerate() {
                    mpv.observe_property(i as isize + 1, property)?;
                }
                // Needed since the observe_property command itself emits a property-change event
                let mut watched_properties_first = Vec::<&String>::new();
                loop {
                    match mpv.listen() {
                        Ok(event) => {
                            if let Some(Value::String(e)) = event.get("event") {
                                if e == "property-change" {
                                    if let Some(Value::String(property)) = event.get("name") {
                                        if let Some(idx) = watched_properties.iter().position(|v| v == &property) {
                                            if watched_properties_first.contains(&property) {
                                                break;
                                            } else {
                                                watched_properties_first.push(watched_properties[idx]);
                                            }
                                        }
                                    }
                                } else if watched_events.contains(&e) {
                                    break;
                                }
                            }
                        }
                        Err(msg) => {
                            error!("Error: {}", msg);
                        }
                    }
                }
                for i in 1..=watched_properties.len() {
                    mpv.unobserve_property(i as isize)?;
                }
            }

            Some(("observe", observe_matches)) => {
                let watched_properties = match observe_matches.get_many::<String>("property") {
                    Some(property_values) => property_values.collect::<Vec<&String>>(),
                    None => Vec::new(),
                };
                for (i, property) in watched_properties.iter().enumerate() {
                    mpv.observe_property(i as isize + 1, property)?;
                }
                loop {
                    println!("{}", mpv.listen_raw());
                }
            }

            _ => unreachable!(),
        },

        Some(("playlist", playlist_matches)) => match playlist_matches.subcommand() {
            Some(("add", add_matches)) => {
                for file in add_matches.get_many::<String>("file").unwrap() {
                    let command = match add_matches.get_one::<String>("type").unwrap().as_str() {
                        "file" => "loadfile",
                        "playlist" => "loadlist",
                        _ => unreachable!(),
                    };
                    let add_option = add_matches.get_one::<String>("mode").unwrap().as_str();
                    mpv.command_arg(command, &[file, add_option])?
                }
            }

            Some(("shuffle", _)) => mpv.command("playlist-shuffle")?,
            Some(("clear", _)) => mpv.command("playlist-clear")?,

            Some(("remove", remove_matches)) => {
                let id = remove_matches.get_one::<String>("id").unwrap();
                mpv.command_arg("playlist-remove", &[id])?;
            }

            Some(("move", move_matches)) => {
                let from = move_matches.get_one::<String>("from").unwrap();
                let to = move_matches.get_one::<String>("to").unwrap();
                mpv.command_arg("playlist-move", &[from, to])?
            }

            Some(("play-next", play_next_matches)) => {
                let current_id = mpv.get_property("playlist-pos")?.as_u64().unwrap();
                let id = play_next_matches.get_one::<String>("id").unwrap();
                mpv.command_arg("playlist-move", &[id, &(current_id + 1).to_string()])?
            }

            Some(("play", play_matches)) => {
                let id = play_matches.get_one::<String>("id").unwrap().parse::<usize>().unwrap();
                mpv.set_property("playlist-pos", id)?
            }

            Some(("reverse", _)) => {
                let count = (mpv.get_property("playlist-count")?.as_u64().unwrap() as usize) - 1;
                for i in 0..count {
                    mpv.command_arg("playlist-move", &[&count.to_string(), &i.to_string()])?;
                }
            }

            Some(("show", _)) => {
                let playlist = mpv.get_property("playlist")?;
                let p = playlist.as_array().unwrap();
                for (i, entry) in p.iter().enumerate() {
                    let e = entry.as_object().unwrap();
                    let title = if e.contains_key("title") {
                        e["title"].as_str().unwrap()
                    } else {
                        e["filename"].as_str().unwrap()
                    };
                    let mut output = format!("{}\t{}", i + 1, title);
                    if e.contains_key("current") {
                        output = format!("{}", output.reverse());
                    }
                    println!("{}", output);
                }
            }

            _ => unreachable!(),
        },

        _ => unreachable!(),
    }

    mpv.disconnect();

    Ok(())
}
