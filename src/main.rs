use std::io;
use std::process::Command as Cmd;
use std::thread;
use std::time::Duration;

use mpvc::{Error, Mpv};

use clap::{Arg, ArgAction, Command, ValueHint, builder::EnumValueParser};
use clap_complete::Shell;
use colored::Colorize;
use serde_json::{Map, Value};

fn value_to_string(v: &Value) -> Result<String, Error> {
    match v {
        Value::Bool(b) => Ok(b.to_string()),
        Value::Number(n) => Ok(n.to_string()),
        Value::String(s) => Ok(s.to_owned()),
        Value::Array(_) => Ok(v.to_string()),
        Value::Object(_) => Ok(v.to_string()),
        Value::Null => Err(Error::MissingValue),
    }
}

fn main() -> Result<(), Error> {
    let mut cli = Command::new(env!("CARGO_CRATE_NAME"))
        .about("An mpc-like CLI tool for mpv")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(Arg::new("socket")
            .short('S')
            .long("socket")
            .value_name("/path/to/socket")
            .value_hint(ValueHint::AnyPath)
            .default_value("/tmp/mpv.sock"))
        .subcommand(Command::new("play")
            .about("Resume playback"))
        .subcommand(Command::new("pause")
            .about("Pause playback"))
        .subcommand(Command::new("toggle")
            .about("Toggle between play and pause"))
        .subcommand(Command::new("next")
            .about("Start playing the next entry in the playlist"))
        .subcommand(Command::new("prev")
            .about("Start playing the previous entry in the playlist"))
        .subcommand(Command::new("seek")
            .about("Set the playback position. By default seeks by a relative amount of seconds. \
                   You may use negative values.")
            .arg(Arg::new("target")
                .required(true))
            .arg(Arg::new("mode")
                .help("relative: Seek relative to the current position (a negative value seeks backwards)\n\
                    absolute: Seek to a given position (a negative value seeks starting from the end of the file)\n\
                    absolute-percent: Seek to a given position in percentage\n\
                    relative-percent: Seek relative to the current position in percentage\n")
                .short('m')
                .long("mode")
                .value_parser(["relative", "relative-percent", "absolute", "absolute-percent"])
                .default_value("relative")))
        .subcommand(Command::new("restart")
            .about("Restart playback of the current file (equivalent to 'seek -m absolute 0')"))
        .subcommand(Command::new("kill")
            .about("Kill the mpv process controlling the socket"))
        .subcommand(Command::new("add")
            .about("Load the given file or playlist and play it")
            .visible_alias("load")
            .arg(Arg::new("file")
                .value_hint(ValueHint::AnyPath)
                .num_args(1..)
                .required(true))
            .arg(Arg::new("mode")
                .help("replace: Stop playback of the current file and play the new file immediately\n\
                    append: Append the file to the playlist\n\
                    append-play: Append the file and if nothing is currently playing, start playback\n\
                    insert-next: Insert the file into the playlist directly after the current entry\n\
                    insert-next-play: Insert the file into the next position and if nothing is currently playing, start playback\n")
                .short('m')
                .long("mode")
                .value_parser(["replace", "append", "append-play", "append-next", "insert-next", "insert-next-play"])
                .default_value("append-play"))
            .arg(Arg::new("type")
                .short('t')
                .long("type")
                .value_parser(["file", "playlist"])
                .default_value("file")))
        .subcommand(Command::new("playlist")
            .about("Print playlist entries")
            .visible_alias("list"))
        .subcommand(Command::new("stop")
            .about("Stop playback and clear the playlist"))
        .subcommand(Command::new("clear")
            .about("Clear the playlist, except the currently playing file"))
        .subcommand(Command::new("remove")
            .about("Remove the given entry from the playlist, or the currently playing entry if the argument is omitted (0-indexed). \
                   If the entry is currently playing, playback will stop.")
            .visible_alias("rm")
            .arg(Arg::new("id")
                .value_parser(str::parse::<usize>)))
        .subcommand(Command::new("move")
            .about("Move the given playlist entry to a new position")
            .visible_alias("mv")
            .arg(Arg::new("from")
                .value_parser(str::parse::<usize>)
                .required(true))
            .arg(Arg::new("to")
                .value_parser(str::parse::<usize>)
                .required(true)))
        .subcommand(Command::new("play-next")
            .about("Move the given playlist entry to be after the currently playing file")
            .arg(Arg::new("id")
                .value_parser(str::parse::<usize>)
                .required(true)))
        .subcommand(Command::new("position")
            .about("Play the given entry in the playlist")
            .visible_alias("pos")
            .arg(Arg::new("id")
                .value_parser(str::parse::<usize>)
                .required(true)))
        .subcommand(Command::new("shuffle")
            .about("Shuffle the playlist")
            .visible_alias("shuf"))
        .subcommand(Command::new("reverse")
            .about("Reverse the playlist")
            .visible_alias("rev"))
        .subcommand(Command::new("loop-file")
            .about("Control whether the current file should be repeated after playback. Toggle by omitting the argument.")
            .arg(Arg::new("arg")
                .value_name("on|off")
                .value_parser(["on", "off"])))
        .subcommand(Command::new("loop-playlist")
            .about("Control whether the playlist should be repeated after the end is reached. Toggle by omitting the argument.")
            .arg(Arg::new("arg")
                .value_name("on|off")
                .value_parser(["on", "off"])))
        .subcommand(Command::new("volume")
            .about("Control the volume level")
            .arg(Arg::new("num")
                .value_parser(str::parse::<f64>)
                .required(true))
            .arg(Arg::new("mode")
                .help("absolute: Set the volume\n\
                    relative: Change the volume relative to the current level (a negative value decreases the level)\n")
                .short('m')
                .long("mode")
                .value_parser(["absolute", "relative"])
                .default_value("absolute")))
        .subcommand(Command::new("mute")
            .about("Control whether audio output is muted. Toggle by omitting the argument.")
            .arg(Arg::new("arg")
                .value_name("on|off")
                .value_parser(["on", "off"])))
        .subcommand(Command::new("set")
            .about("Set a property to the given value")
            .arg(Arg::new("json")
                .help("Parse <value> as JSON")
                .short('j')
                .long("json")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("property")
                .required(true))
            .arg(Arg::new("value")
                .required(true)))
        .subcommand(Command::new("get")
            .about("Retrieve a property (see property 'property-list' for possible values)")
            .arg(Arg::new("json")
                .help("Print property as JSON")
                .short('j')
                .long("json")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("property")
                .required(true)))
        .subcommand(Command::new("run")
            .about("Run an mpv command")
            .arg(Arg::new("command")
                .required(true))
            .arg(Arg::new("args")
                .help("String arguments")
                .num_args(0..))
            .arg(Arg::new("json")
                .help("JSON arguments")
                .num_args(1..)
                .last(true)))
        .subcommand(Command::new("metadata")
            .about("Retrieve a metadata attribute from the currently playing file (see property 'metadata' for possible values)")
            .arg(Arg::new("attribute")
                .required(true)))
        .subcommand(Command::new("format")
            .about("Replace the given specifiers in the format string with their real-time values from mpv")
            .arg(Arg::new("format-string")
                .help("Possible format specifiers are:\n\
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
                    %n% (newline)\n\
                    %% (escaped percent)\n\
                    %[% (escaped left bracket)\n\
                    %]% (escaped right bracket)\n\n\
                    Additionally, any valid property may be used.\n\n\
                    The format specifier may also appear in the form:\n\
                    \t%property?consequent:alternative%\n\
                    where the property evaluates to a boolean.\n\n\
                    Brackets may be used to group output such that if any\n\
                    format specifiers contained within fail to be retrieved,\n\
                    then none of the characters between the brackets are outputted.")
                .required(true)))
        .subcommand(Command::new("observe")
            .about("Print all mpv events in real-time. Additionally, observe a set of properties and inform about changes.")
            .arg(Arg::new("property")
                .num_args(0..)))
        .subcommand(Command::new("wait")
            .about("Block until one of the given events is triggered, or until one of the given properties is changed")
            .arg_required_else_help(true)
            .arg(Arg::new("event")
                .help("start-file: Happens right before a new file is loaded. When you receive this, the player is loading the file (or possibly already done with it).\n\
                    end-file: Happens after a file was unloaded. Typically, the player will load the next file right away, or quit if this was the last file.\n\
                    file-loaded: Happens after a file was loaded and begins playback.\n\
                    seek: Happens on seeking. (This might include cases when the player seeks internally, even without user interaction. This includes e.g. segment changes when playing ordered chapters Matroska files.)\n\
                    playback-restart: Start of playback after seek or after file was loaded.\n\
                    shutdown: Sent when the player quits, and the script should terminate. Normally handled automatically. See Details on the script initialization and lifecycle.\n\
                    video-reconfig: Happens on video output or filter reconfig.\n\
                    audio-reconfig: Happens on audio output or filter reconfig.\n")
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
                .num_args(1..)
                .last(true)))
        .subcommand(Command::new("completion")
            .about("Generate a shell completion script")
            .hide(true)
            .arg(Arg::new("shell")
                .value_parser(EnumValueParser::<Shell>::new())
                .required(true)));
    let matches = cli.get_matches_mut();

    if let Some(("completion", completion_matches)) = matches.subcommand() {
        let shell = *completion_matches.get_one::<Shell>("shell").unwrap();
        clap_complete::generate(shell, &mut cli, env!("CARGO_CRATE_NAME"), &mut io::stdout());
        return Ok(());
    }

    let socket = matches.get_one::<String>("socket").unwrap();
    let mut mpv = match Mpv::connect(socket) {
        Ok(instance) => instance,
        Err(e) => {
            if let Some(("add", _)) = matches.subcommand() {
                #[allow(clippy::zombie_processes)]
                Cmd::new("mpv")
                    .args([
                        "--no-terminal",
                        "--idle=once",
                        "--vid=no",
                        ("--input-ipc-server=".to_owned() + socket).as_str(),
                    ])
                    .spawn()
                    .expect("mpv failed to start");
                thread::sleep(Duration::from_millis(500));
                Mpv::connect(socket)?
            } else {
                return Err(e);
            }
        }
    };

    match matches.subcommand() {
        Some(("play", _)) => mpv.set_property("pause", false)?,
        Some(("pause", _)) => mpv.set_property("pause", true)?,
        Some(("toggle", _)) => mpv.command_arg("cycle", ["pause"])?,
        Some(("next", _)) => mpv.command("playlist-next")?,
        Some(("prev", _)) => mpv.command("playlist-prev")?,
        Some(("seek", seek_matches)) => {
            let target = seek_matches.get_one::<String>("target").unwrap().as_str();
            let mode = seek_matches.get_one::<String>("mode").unwrap().as_str();
            mpv.command_arg("seek", [target, mode])?;
        }
        Some(("restart", _)) => mpv.command_arg("seek", ["0", "absolute"])?,
        Some(("kill", _)) => mpv.command("quit")?,

        Some(("add", add_matches)) => {
            let command = match add_matches.get_one::<String>("type").unwrap().as_str() {
                "file" => "loadfile",
                "playlist" => "loadlist",
                _ => unreachable!(),
            };
            let mode = add_matches.get_one::<String>("mode").unwrap().as_str();
            for file in add_matches.get_many::<String>("file").unwrap() {
                mpv.command_arg(command, [file.as_str(), mode])?;
            }
        }

        Some(("playlist", _)) => {
            let property = mpv.get_property("playlist")?;
            let playlist = property.as_array().ok_or(Error::UnexpectedValue)?;
            for (i, e) in playlist.iter().enumerate() {
                let entry = e.as_object().ok_or(Error::UnexpectedValue)?;
                let title = if let Some(title) = entry.get("title") {
                    title
                } else if let Some(filename) = entry.get("filename") {
                    filename
                } else {
                    return Err(Error::MissingValue);
                }.as_str().ok_or(Error::UnexpectedValue)?;
                let mut output = format!("{}\t{}", i + 1, title);
                if entry.contains_key("current") {
                    output = output.reversed().to_string();
                }
                println!("{}", output);
            }
        }

        Some(("stop", _)) => mpv.command("stop")?,
        Some(("clear", _)) => mpv.command("playlist-clear")?,

        Some(("remove", remove_matches)) => {
            let value: Value = if let Some(id) = remove_matches.get_one::<usize>("id") {
                (*id).into()
            } else {
                "current".into()
            };
            mpv.command_arg("playlist-remove", [value])?;
        }

        Some(("move", move_matches)) => {
            let from = *move_matches.get_one::<usize>("from").unwrap();
            let to = *move_matches.get_one::<usize>("to").unwrap();
            mpv.command_arg("playlist-move", [from, to])?;
        }

        Some(("play-next", play_next_matches)) => {
            let pos = mpv.get_property("playlist-pos")?.as_u64().ok_or(Error::UnexpectedValue)? as usize;
            let id = *play_next_matches.get_one::<usize>("id").unwrap();
            mpv.command_arg("playlist-move", [id, pos + 1])?;
        }

        Some(("position", position_matches)) => {
            let id = *position_matches.get_one::<usize>("id").unwrap();
            mpv.set_property("playlist-pos", id)?;
        }

        Some(("shuffle", _)) => mpv.command("playlist-shuffle")?,

        Some(("reverse", _)) => {
            let count = mpv.get_property("playlist-count")?.as_u64().ok_or(Error::UnexpectedValue)? as usize - 1;
            for i in 0..count {
                mpv.command_arg("playlist-move", [count, i])?;
            }
        }

        Some(("loop-file", loop_file_matches)) => {
            match loop_file_matches.get_one::<String>("arg") {
                Some(arg) => match arg.as_str() {
                    "on" => mpv.set_property("loop-file", "inf")?,
                    "off" => mpv.set_property("loop-file", false)?,
                    _ => unreachable!(),
                },
                None => mpv.command_arg("cycle-values", ["loop-file".into(), Value::Bool(false), "inf".into()])?,
            }
        }

        Some(("loop-playlist", loop_playlist_matches)) => {
            match loop_playlist_matches.get_one::<String>("arg") {
                Some(arg) => match arg.as_str() {
                    "on" => mpv.set_property("loop-playlist", "inf")?,
                    "off" => mpv.set_property("loop-playlist", false)?,
                    _ => unreachable!(),
                },
                None => mpv.command_arg("cycle-values", ["loop-playlist".into(), Value::Bool(false), "inf".into()])?,
            }
        }

        Some(("volume", volume_matches)) => {
            let num = *volume_matches.get_one::<f64>("num").unwrap();
            match volume_matches.get_one::<String>("mode").unwrap().as_str() {
                "absolute" => mpv.set_property("volume", num)?,
                "relative" => mpv.add_property("volume", num)?,
                _ => unreachable!(),
            }
        }

        Some(("mute", mute_matches)) => {
            match mute_matches.get_one::<String>("arg") {
                Some(arg) => match arg.as_str() {
                    "on" => mpv.set_property("mute", true)?,
                    "off" => mpv.set_property("mute", false)?,
                    _ => unreachable!(),
                },
                None => mpv.command_arg("cycle", ["mute"])?,
            }
        }

        Some(("set", set_matches)) => {
            let property = set_matches.get_one::<String>("property").unwrap();
            let value = set_matches.get_one::<String>("value").unwrap();
            let json = *set_matches.get_one::<bool>("json").unwrap();
            let value = if json {
                value.parse::<Value>().map_err(Error::JsonError)?
            } else {
                value.as_str().into()
            };
            mpv.set_property(property, value)?;
        }

        Some(("get", get_matches)) => {
            let property = get_matches.get_one::<String>("property").unwrap();
            let json = *get_matches.get_one::<bool>("json").unwrap();
            let value = mpv.get_property(property)?;
            if json {
                println!("{}", value);
            } else {
                println!("{}", value_to_string(&value)?);
            }
        }

        Some(("run", run_matches)) => {
            let command = run_matches.get_one::<String>("command").unwrap();
            let args = run_matches.get_many::<String>("args").unwrap_or_default()
                .map(|v| v.as_str().into());
            let json = if let Some(json) = run_matches.get_many::<String>("json") {
                json.map(|v| v.parse::<Value>())
                    .collect::<Result<Vec<_>, _>>().map_err(Error::JsonError)?
            } else {
                Vec::new()
            };
            mpv.command_arg(command, args.chain(json))?;
        }

        Some(("metadata", metadata_matches)) => {
            let attribute = metadata_matches.get_one::<String>("attribute").unwrap().as_str();
            let metadata = mpv.get_property(("metadata/by-key/".to_owned() + attribute).as_str())?;
            println!("{}", value_to_string(&metadata)?);
        }

        Some(("format", format_matches)) => {
            fn eval_format(mpv: &mut Mpv, metadata: &Map<String, Value>, spec: &str) -> Option<String> {
                fn format_duration(d: u64) -> String {
                    match (d % 60, (d / 60) % 60, d / 3600) {
                        (s, m, 0) => format!("{:02}:{:02}", m, s),
                        (s, m, h) => format!("{:02}:{:02}:{:02}", h, m, s),
                    }
                }

                match spec {
                    "" => Some("%".to_owned()),
                    "n" => Some("\n".to_owned()),
                    "[" | "]" => Some(spec.to_owned()),
                    "title" => {
                        if let Some(title) = metadata.get("title") {
                            Some(title.as_str()?.to_owned())
                        } else {
                            Some(mpv.get_property("media-title").ok()?.as_str()?.to_owned())
                        }
                    }
                    "time" => {
                        let time = mpv.get_property("time-pos").ok()?.as_f64()?;
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
                        let position = mpv.get_property("playlist-pos-1").ok()?.as_u64()?;
                        Some(position.to_string())
                    }
                    _ => {
                        if let Some(i) = spec.find('?') {
                            let property = &spec[..i];
                            let pair = &spec[i + 1..];
                            let j = pair.find(':')?;
                            if mpv.get_property(property).ok()?.as_bool()? {
                                Some(pair[..j].to_owned())
                            } else {
                                Some(pair[j + 1..].to_owned())
                            }
                        } else if let Some(metadata) = metadata.get(spec) {
                            Some(value_to_string(metadata).ok()?)
                        } else {
                            value_to_string(&mpv.get_property(spec).ok()?).ok()
                        }
                    }
                }
            }

            let mut input = format_matches.get_one::<String>("format-string").unwrap().as_str();
            let mut output = String::with_capacity(input.len());
            let metadata = if let Ok(metadata) = mpv.get_property("metadata") {
                metadata.as_object().ok_or(Error::UnexpectedValue)?
                    .iter().map(|(k, v)| (k.to_lowercase(), v.clone())).collect()
            } else {
                Map::<String, Value>::new()
            };

            enum State {
                Raw,
                Spec,
                Skip(usize, bool),
            }
            use State::*;

            let mut state = Raw;
            let mut stack = Vec::new();

            loop {
                match state {
                    Raw => {
                        let Some(i) = input.find(['%', '[', ']']) else {
                            // No further format specifiers or groups
                            output += input;
                            break;
                        };
                        if let Some(last) = stack.last_mut() {
                            *last += &input[..i];
                        } else {
                            output += &input[..i];
                        }
                        match input.as_bytes()[i] {
                            b'%' => state = Spec,
                            b'[' => stack.push(String::new()),
                            b']' => {
                                if let Some(pop) = stack.pop() {
                                    if let Some(last) = stack.last_mut() {
                                        // Collapse the last two elements
                                        *last += pop.as_str();
                                    } else {
                                        output += pop.as_str()
                                    }
                                } else {
                                    output.push(']'); // XXX
                                }
                            }
                            _ => unreachable!(),
                        }
                        input = &input[i + 1..];
                    }
                    Spec => {
                        let Some(i) = input.find('%') else {
                            // Unterminated format specifier
                            break;
                        };
                        if let Some(s) = eval_format(&mut mpv, &metadata, &input[..i]) {
                            if let Some(last) = stack.last_mut() {
                                *last += s.as_str();
                            } else {
                                output += s.as_str();
                            }
                            state = Raw;
                        } else if stack.is_empty() {
                            state = Raw;
                        } else {
                            stack.pop();
                            state = Skip(0, false);
                        }
                        input = &input[i + 1..];
                    }
                    Skip(ref mut nesting, ref mut spec) => {
                        let i = if *spec {
                            input.find('%')
                        } else {
                            input.find(['%', '[', ']'])
                        };
                        let Some(i) = i else {
                            // Unterminated group or format specifier
                            break;
                        };
                        match input.as_bytes()[i] {
                            b'%' => *spec = !*spec,
                            b'[' => *nesting += 1,
                            b']' => {
                                if *nesting == 0 {
                                    state = Raw;
                                } else {
                                    *nesting -= 1;
                                }
                            }
                            _ => unreachable!(),
                        }
                        input = &input[i + 1..];
                    }
                }
            }
            print!("{}", output);
        }

        Some(("observe", observe_matches)) => {
            let properties = match observe_matches.get_many::<String>("property") {
                Some(properties) => properties.collect(),
                None => Vec::new(),
            };
            for (i, property) in properties.into_iter().enumerate() {
                mpv.observe_property(i as isize + 1, property)?;
            }
            while let Ok(response) = mpv.listen_raw() {
                println!("{}", response);
            }
        }

        Some(("wait", wait_matches)) => {
            let events = match wait_matches.get_many::<String>("event") {
                Some(events) => events.collect(),
                None => Vec::new(),
            };
            let properties = match wait_matches.get_many::<String>("property") {
                Some(properties) => properties.collect(),
                None => Vec::new(),
            };
            for (i, property) in properties.iter().enumerate() {
                mpv.observe_property(i as isize + 1, property)?;
            }
            // Needed since the observe_property command itself emits a property-change event
            let mut seen = Vec::new();
            loop {
                let event = mpv.listen()?;
                if let Some(Value::String(e)) = event.get("event") {
                    if e == "property-change" {
                        if let Some(Value::String(property)) = event.get("name") {
                            if let Some(i) = properties.iter().position(|v| v == &property) {
                                if seen.contains(&property) {
                                    break;
                                } else {
                                    seen.push(properties[i]);
                                }
                            }
                        }
                    } else if events.contains(&e) {
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
