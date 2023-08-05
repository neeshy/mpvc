extern crate mpvc;

extern crate clap;
extern crate colored;
extern crate serde_json;

use std::process::Command as Cmd;
use std::thread;
use std::time::Duration;

use mpvc::{Error, Mpv};

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
            .default_value("/tmp/mpv.sock")
            .num_args(1))
        .subcommand(Command::new("play")
            .about("Resume playback"))
        .subcommand(Command::new("pause")
            .about("Pause playback"))
        .subcommand(Command::new("toggle")
            .about("Toggles between play and pause"))
        .subcommand(Command::new("next")
            .about("Starts playing the next file in the playlist"))
        .subcommand(Command::new("prev")
            .about("Starts playing the previous file in the playlist"))
        .subcommand(Command::new("seek")
            .about("Set the playback position. By default seeks by a relative amount of seconds. \
                   You may use negative values.")
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
                <relative-percent>: Seek relative to current position in percent.\n")
                .num_args(1)))
        .subcommand(Command::new("restart")
            .about("Restart playback of the current file (equivalent to 'seek -m absolute 0')"))
        .subcommand(Command::new("kill")
            .about("Kill the mpv process controlling the socket"))
        .subcommand(Command::new("add")
            .about("Load the given file or playlist and play it")
            .visible_alias("load")
            .arg(Arg::new("file")
                .value_name("FILE")
                .num_args(1..)
                .required(true))
            .arg(Arg::new("mode")
                .short('m')
                .long("mode")
                .value_parser(["replace", "append", "append-play", "append-next"])
                .hide_possible_values(true)
                .default_value("append-play")
                .help("<replace|append|append-play|append-next>\n\
                <replace>: Stop playback of the current file, and play the new file immediately.\n\
                <append>: Append the file to the playlist.\n\
                <append-play>: Append the file, and if nothing is currently playing, start playback.\n\
                <append-next>: Append the file to the playlist, and place it in the next position.\n")
                .num_args(1))
            .arg(Arg::new("type")
                .short('t')
                .long("type")
                .value_parser(["file", "playlist"])
                .default_value("file")))
        .subcommand(Command::new("playlist")
            .about("Prints the playlist"))
        .subcommand(Command::new("stop")
            .about("Stop playback and clear playlist"))
        .subcommand(Command::new("clear")
            .about("Clear the playlist, except the currently playing file"))
        .subcommand(Command::new("remove")
            .about("Removes <ID> from the 0-indexed playlist. If <ID> is currently playing, playback will stop.")
            .visible_alias("rm")
            .arg(Arg::new("id")
                .value_name("ID")
                .required(true)))
        .subcommand(Command::new("move")
            .about("Moves the playlist entry at position <FROM> to position <TO>")
            .visible_alias("mv")
            .arg(Arg::new("from")
                .value_name("FROM")
                .required(true))
            .arg(Arg::new("to")
                .value_name("TO")
                .required(true)))
        .subcommand(Command::new("play-next")
            .about("Moves the playlist entry at position <ID> to be after the currently playing file")
            .arg(Arg::new("id")
                .value_name("ID")
                .help("Defines the id that should be played next")
                .required(true)))
        .subcommand(Command::new("position")
            .about("Plays the file at <ID> in the playlist")
            .arg(Arg::new("id")
                .value_name("ID")
                .required(true)))
        .subcommand(Command::new("shuffle")
            .about("Shuffles the playlist"))
        .subcommand(Command::new("reverse")
            .about("Reverses the playlist")
            .visible_alias("rev"))
        .subcommand(Command::new("loop-file")
            .about("Controls whether the current file should be repeated after playback")
            .arg(Arg::new("arg")
                .value_name("on|off|toggle")
                .value_parser(["on", "off", "toggle"])
                .required(true)))
        .subcommand(Command::new("loop-playlist")
            .about("Controls whether the playlist should be repeated after the end is reached")
            .arg(Arg::new("arg")
                .value_name("on|off|toggle")
                .value_parser(["on", "off", "toggle"])
                .required(true)))
        .subcommand(Command::new("volume")
            .about("Controls the volume level")
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
                <relative>: Change the volume relatively. To decrease, use negative values for <NUM>.\n")
                .num_args(1)))
        .subcommand(Command::new("mute")
            .about("Controls whether audio output is muted")
            .arg(Arg::new("arg")
                .value_name("on|off|toggle")
                .value_parser(["on", "off", "toggle"])
                .required(true)))
        .subcommand(Command::new("set")
            .about("Sets an mpv property to <VALUE>")
            .arg(Arg::new("property")
                .required(true))
            .arg(Arg::new("value")
                .required(true)))
        .subcommand(Command::new("get")
            .about("Retrieves an mpv property (see property 'property-list' for possible values)")
            .arg(Arg::new("property")
                .required(true)))
        .subcommand(Command::new("metadata")
            .about("Retrieves a metadata attribute from the currently playing file (see property 'metadata' for possible values)")
            .arg(Arg::new("attribute")
                .required(true)))
        .subcommand(Command::new("format")
            .about("Replaces the given arguments in the format string with their real-time values from mpv")
            .arg(Arg::new("input")
                .value_name("FORMAT-STRING")
                .help("Possible format string arguments are:\n\
                    %filename%\n\
                    %path%\n\
                    %title%\n\
                    %artist%\n\
                    %album%\n\
                    %album_artist%\n\
                    %composer%\n\
                    %genre%\n\
                    %date%\n\
                    %year%\n\
                    %comment%\n\
                    %track%\n\
                    %disc%\n\
                    %time%\n\
                    %duration%\n\
                    %percentage%\n\
                    %position%\n\
                    %playlist-count%\n\
                    %n% (newline)\n\n\
                    Additionally, any valid property may be used.\n\n\
                    The format string may also appear in the form:\n\
                    \t'%property?consequent:alternative%'\n\
                    where the property evaluates to a boolean.")
                .required(true)))
        .subcommand(Command::new("observe")
            .about("Prints all mpv events in real-time. Additionally, observes a set of properties and informs about changes")
            .arg(Arg::new("property")
                .value_name("PROPERTY")
                .num_args(0..)))
        .subcommand(Command::new("wait")
            .about("Blocks until the mpv event <EVENT> is triggered, or until the mpv property <PROPERTY> is changed")
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
                .last(true)))
        .get_matches();

    // Input socket is always present, therefore unwrap
    let socket = matches.get_one::<String>("socket").unwrap();
    let mut mpv = match Mpv::connect(socket) {
        Ok(instance) => instance,
        Err(msg) => match matches.subcommand() {
            Some(("add", _)) => {
                Cmd::new("mpv")
                    .args([
                        "--no-terminal",
                        "--idle=once",
                        "--vid=no",
                        &("--input-ipc-server=".to_string() + socket),
                    ])
                    .spawn()
                    .expect("mpv failed to start");
                thread::sleep(Duration::from_millis(500));
                Mpv::connect(socket)?
            }
            _ => return Err(msg),
        },
    };

    match matches.subcommand() {
        Some(("play", _)) => mpv.set_property("pause", false)?,
        Some(("pause", _)) => mpv.set_property("pause", true)?,
        Some(("toggle", _)) => mpv.command_arg("cycle", &["pause"])?,
        Some(("next", _)) => mpv.command("playlist-next")?,
        Some(("prev", _)) => mpv.command("playlist-prev")?,
        Some(("seek", seek_matches)) => {
            let num = seek_matches.get_one::<String>("num").unwrap();
            let mode = seek_matches.get_one::<String>("mode").unwrap().as_str();
            mpv.command_arg("seek", &[num, mode])?
        }
        Some(("restart", _)) => mpv.command_arg("seek", &["0", "absolute"])?,
        Some(("kill", _)) => mpv.command("quit")?,

        Some(("add", add_matches)) => {
            let command = match add_matches.get_one::<String>("type").unwrap().as_str() {
                "file" => "loadfile",
                "playlist" => "loadlist",
                _ => unreachable!(),
            };
            let mode = add_matches.get_one::<String>("mode").unwrap().as_str();
            match mode {
                "replace" | "append" | "append-play" => {
                    for file in add_matches.get_many::<String>("file").unwrap() {
                        mpv.command_arg(command, &[file, mode])?
                    }
                }
                "append-next" => {
                    let files = add_matches.get_many::<String>("file").unwrap();
                    let files_len = files.len();
                    let count = mpv.get_property("playlist-count")?.as_u64().unwrap() as usize;
                    for file in files {
                        mpv.command_arg(command, &[file, "append"])?;
                    }
                    let pos = mpv.get_property("playlist-pos")?.as_u64().unwrap() as usize + 1;
                    for i in 0..files_len {
                        mpv.command_arg("playlist-move", &[&(count + i).to_string(), &(pos + i).to_string()])?;
                    }
                }
                _ => unreachable!(),
            }
        }

        Some(("playlist", _)) => {
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
                    output = output.reversed().to_string();
                }
                println!("{}", output);
            }
        }

        Some(("stop", _)) => mpv.command("stop")?,
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
            let pos = mpv.get_property("playlist-pos")?.as_u64().unwrap();
            let id = play_next_matches.get_one::<String>("id").unwrap();
            mpv.command_arg("playlist-move", &[id, &(pos + 1).to_string()])?
        }

        Some(("position", position_matches)) => {
            let id = position_matches.get_one::<String>("id").unwrap().parse::<usize>().unwrap();
            mpv.set_property("playlist-pos", id)?
        }

        Some(("shuffle", _)) => mpv.command("playlist-shuffle")?,

        Some(("reverse", _)) => {
            let count = mpv.get_property("playlist-count")?.as_u64().unwrap() as usize - 1;
            let count_str = count.to_string();
            for i in 0..count {
                mpv.command_arg("playlist-move", &[&count_str, &i.to_string()])?;
            }
        }

        Some(("loop-file", loop_file_matches)) => {
            let arg = match loop_file_matches.get_one::<String>("arg").unwrap().as_str() {
                "on" => true,
                "off" => false,
                "toggle" => matches!(mpv.get_property_string("loop-file")?.as_str(), "false"),
                _ => unreachable!(),
            };
            mpv.set_property("loop-file", arg)?;
        }

        Some(("loop-playlist", loop_playlist_matches)) => {
            let arg = match loop_playlist_matches.get_one::<String>("arg").unwrap().as_str() {
                "on" => true,
                "off" => false,
                "toggle" => matches!(mpv.get_property_string("loop-playlist")?.as_str(), "false"),
                _ => unreachable!(),
            };
            mpv.set_property("loop-playlist", arg)?;
        }

        Some(("volume", volume_matches)) => {
            let num = volume_matches.get_one::<String>("num").unwrap().parse::<f64>().unwrap();
            match volume_matches.get_one::<String>("mode").unwrap().as_str() {
                "absolute" => mpv.set_property("volume", num)?,
                "relative" => mpv.add_property("volume", num)?,
                _ => unreachable!(),
            }
        }

        Some(("mute", mute_matches)) => {
            match mute_matches.get_one::<String>("arg").unwrap().as_str() {
                "on" => mpv.set_property("mute", true)?,
                "off" => mpv.set_property("mute", false)?,
                "toggle" => mpv.command_arg("cycle", &["mute"])?,
                _ => unreachable!(),
            }
        }

        Some(("set", set_matches)) => {
            let property = set_matches.get_one::<String>("property").unwrap();
            let value = set_matches.get_one::<String>("value").unwrap();
            mpv.set_property(property, value)?;
        }

        Some(("get", get_matches)) => {
            let property = get_matches.get_one::<String>("property").unwrap();
            let value = mpv.get_property(property)?;
            println!("{}", value);
        }

        Some(("metadata", metadata_matches)) => {
            let attribute = metadata_matches.get_one::<String>("attribute").unwrap();
            let property = mpv.get_property("metadata")?;
            let metadata = property.as_object().unwrap();
            let value = metadata.get(attribute).ok_or(Error::MpvError("metadata attribute not found".to_string()))?;
            if let Value::String(ref v) = value {
                println!("{}", v);
            } else {
                println!("{:?}", value);
            }
        }

        Some(("format", format_matches)) => {
            fn eval_format(mpv: &mut Mpv, metadata: &Map<String, Value>, key: &str) -> Option<String> {
                fn format_duration(d: u64) -> String {
                    match (d % 60, (d / 60) % 60, d / 3600) {
                        (s, m, 0) => format!("{:02}:{:02}", m, s),
                        (s, m, h) => format!("{:02}:{:02}:{:02}", h, m, s),
                    }
                }

                match key {
                    "n" => Some("\n".to_string()),
                    "title" => {
                        if metadata.contains_key("title") {
                            Some(metadata["title"].as_str()?.to_string())
                        } else {
                            Some(mpv.get_property("media-title").ok()?.as_str()?.to_string())
                        }
                    }
                    "artist" | "album" | "album_artist" | "date" | "year" | "track" | "genre" |
                    "composer" | "comment" | "disc" => {
                        Some(metadata.get(key)?.as_str()?.to_string())
                    }
                    "time" => {
                        let time = mpv.get_property("playback-time").ok()?.as_f64()?;
                        Some(format_duration(time as u64))
                    }
                    "duration" => {
                        let duration = mpv.get_property("duration").ok()?.as_f64()?;
                        Some(format_duration(duration as u64))
                    }
                    "percentage" => {
                        let percent = mpv.get_property("percent-pos").ok()?.as_f64()?;
                        Some((percent as u64).to_string())
                    }
                    "position" => {
                        let position = mpv.get_property("playlist-pos").ok()?.as_u64()?;
                        Some((position + 1).to_string())
                    }
                    _ => {
                        if let Some(i) = key.find("?") {
                            let property = &key[..i];
                            let pair = &key[i + 1..];
                            let j = pair.find(":")?;
                            if mpv.get_property(property).ok()?.as_bool()? {
                                Some(pair[..j].to_string())
                            } else {
                                Some(pair[j + 1..].to_string())
                            }
                        } else {
                            mpv.get_property_string(key).ok()
                        }
                    }
                }
            }

            let input = format_matches.get_one::<String>("input").unwrap();
            let property = mpv.get_property("metadata")?;
            let metadata = property.as_object().unwrap();
            // Manually parse the format string instead of doing repeated search
            // and replace operations. This avoids issues with "double replacements".
            // e.g. If the format string is "%title%" and the title metadata in
            // turn contains a valid format string (say "%path%", unlikely but possible),
            // the resulting output will be incorrect.
            // Despite processing the string in one pass, this is actually slower than the
            // alternative of repeatedly calling String::replace().
            let mut output = String::with_capacity(input.len());
            let mut i = 0usize;
            loop {
                let sub = &input[i..];
                match sub.find('%') {
                    Some(start) => {
                        let sub_fmt = &sub[start + 1..];
                        match sub_fmt.find('%') {
                            Some(end) => {
                                output += &sub[..start];
                                let fmt = &sub_fmt[..end];
                                match eval_format(&mut mpv, metadata, fmt) {
                                    Some(m) => {
                                        output += &m;
                                        // If the format string is valid, the
                                        // starting index should be iterated past
                                        // the ending '%'. Add two to account for
                                        // each delimiter.
                                        i += start + end + 2;
                                    }
                                    None => {
                                        // If this was not a valid format string, set the index to
                                        // the ending '%'. This is needed in case of unbalanced %'s
                                        // i.e. the string "100% Orange Juice: %percentage%" will
                                        // produce the following iterations:
                                        //   1: sub[..start] == "100", fmt == " Orange Juice: "
                                        //   2: sub[..start] == "",    fmt == "percentage"
                                        output += "%";
                                        output += fmt;
                                        i += start + end + 1;
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
            print!("{}", output);
        }

        Some(("observe", observe_matches)) => {
            let watched_properties = match observe_matches.get_many::<String>("property") {
                Some(property_values) => property_values.collect::<Vec<&String>>(),
                None => Vec::new(),
            };
            for (i, property) in watched_properties.iter().enumerate() {
                mpv.observe_property(i as isize + 1, property)?;
            }
            while let Ok(response) = mpv.listen_raw() {
                println!("{}", response);
            }
        }

        Some(("wait", wait_matches)) => {
            let watched_events = match wait_matches.get_many::<String>("event") {
                Some(event_values) => event_values.collect::<Vec<&String>>(),
                None => Vec::new(),
            };
            let watched_properties = match wait_matches.get_many::<String>("property") {
                Some(property_values) => property_values.collect::<Vec<&String>>(),
                None => Vec::new(),
            };
            for (i, property) in watched_properties.iter().enumerate() {
                mpv.observe_property(i as isize + 1, property)?;
            }
            // Needed since the observe_property command itself emits a property-change event
            let mut watched_properties_first = Vec::<&String>::new();
            loop {
                let event = mpv.listen()?;
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
        }

        _ => unreachable!(),
    }

    mpv.disconnect();

    Ok(())
}
