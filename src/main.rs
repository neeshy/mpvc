use core::time::Duration;
use std::io;
use std::process::Command as Cmd;
use std::thread;

use mpvc::{Error, Mpv};

use clap::{CommandFactory as _, Parser, Subcommand, ValueEnum, ValueHint};
use clap_complete::Shell;
use colored::Colorize as _;
use serde_json::{Map, Value};

fn value_to_string(v: &Value) -> Result<String, Error> {
    match *v {
        Value::Bool(ref b) => Ok(b.to_string()),
        Value::Number(ref n) => Ok(n.to_string()),
        Value::String(ref s) => Ok(s.to_owned()),
        Value::Array(_) | Value::Object(_) => Ok(v.to_string()),
        Value::Null => Err(Error::MissingValue),
    }
}

#[derive(Parser)]
#[command(
    name = env!("CARGO_CRATE_NAME"),
    about = "An mpc-like CLI tool for mpv",
    subcommand_required = true,
    arg_required_else_help = true,
)]
struct Cli {
    #[arg(
        short = 'S',
        long,
        value_name = "/path/to/socket",
        value_hint = ValueHint::AnyPath,
        default_value = "/tmp/mpv.sock"
    )]
    socket: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Resume playback
    Play,

    /// Pause playback
    Pause,

    /// Toggle between play and pause
    Toggle,

    /// Start playing the next entry in the playlist
    Next,

    /// Start playing the previous entry in the playlist
    Prev,

    /// Set the playback position.
    ///
    /// By default seeks by a relative amount of seconds. You may use negative values.
    Seek {
        target: String,

        /// relative: Seek relative to the current position (a negative value seeks backwards)
        /// absolute: Seek to a given position (a negative value seeks starting from the end of the file)
        /// absolute-percent: Seek to a given position in percentage
        /// relative-percent: Seek relative to the current position in percentage
        #[arg(
            short,
            long,
            value_parser = ["relative", "relative-percent", "absolute", "absolute-percent"],
            default_value = "relative",
            help = "",
            long_help,
            verbatim_doc_comment,
        )]
        mode: String,
    },

    /// Restart playback of the current file (equivalent to 'seek -m absolute 0')
    Restart,

    /// Kill the mpv process controlling the socket
    Kill,

    /// Load the given file or playlist and play it
    #[command(visible_alias = "load")]
    Add {
        #[arg(value_hint = ValueHint::AnyPath, num_args = 1.., required = true)]
        file: Vec<String>,

        /// replace: Stop playback of the current file and play the new file immediately
        /// append: Append the file to the playlist
        /// append-play: Append the file and if nothing is currently playing, start playback
        /// insert-next: Insert the file into the playlist directly after the current entry
        /// insert-next-play: Insert the file into the next position and if nothing is currently playing, start playback
        #[arg(
            short,
            long,
            value_parser = ["replace", "append", "append-play", "append-next", "insert-next", "insert-next-play"],
            default_value = "append-play",
            help = "",
            long_help,
            verbatim_doc_comment,
        )]
        mode: String,

        #[arg(short, long, value_enum, default_value = "file")]
        r#type: AddType,
    },

    /// Print playlist entries
    #[command(visible_alias = "list")]
    Playlist,

    /// Stop playback and clear the playlist
    Stop,

    /// Clear the playlist, except the currently playing file
    Clear,

    /// Remove the given entry from the playlist
    ///
    /// The currently playing entry will be removed if the argument is omitted (0-indexed).
    /// If the entry is currently playing, playback will stop.
    #[command(visible_alias = "rm")]
    Remove {
        id: Option<u64>,
    },

    /// Move the given playlist entry to a new position
    #[command(visible_alias = "mv")]
    Move {
        from: u64,
        to: u64,
    },

    /// Move the given playlist entry to be after the currently playing file
    PlayNext {
        id: u64,
    },

    /// Play the given entry in the playlist
    #[command(visible_alias = "pos")]
    Position {
        id: u64,
    },

    /// Shuffle the playlist
    #[command(visible_alias = "shuf")]
    Shuffle,

    /// Reverse the playlist
    #[command(visible_alias = "rev")]
    Reverse,

    /// Control whether the current file should be repeated after playback
    ///
    /// Toggle by omitting the argument
    LoopFile {
        #[arg(value_name = "on|off", value_enum)]
        arg: Option<Toggle>,
    },

    /// Control whether the playlist should be repeated after the end is reached
    ///
    /// Toggle by omitting the argument
    LoopPlaylist {
        #[arg(value_name = "on|off", value_enum)]
        arg: Option<Toggle>,
    },

    /// Control the volume level
    Volume {
        num: f64,

        /// absolute: Set the volume
        /// relative: Change the volume relative to the current level (a negative value decreases the level)
        #[arg(
            short,
            long,
            value_enum,
            default_value = "absolute",
            help = "",
            long_help,
            verbatim_doc_comment,
        )]
        mode: VolumeMode,
    },

    /// Control whether audio output is muted
    ///
    /// Toggle by omitting the argument
    Mute {
        #[arg(value_name = "on|off", value_enum)]
        arg: Option<Toggle>,
    },

    /// Set a property to the given value
    Set {
        /// Parse <VALUE> as JSON
        #[arg(short, long)]
        json: bool,

        property: String,
        value: String,
    },

    /// Retrieve a property
    ///
    /// See property 'property-list' for possible values
    Get {
        /// Print <PROPERTY> as JSON
        #[arg(short, long)]
        json: bool,

        property: String,
    },

    /// Run an mpv command
    Run {
        command: String,

        /// String arguments
        args: Vec<String>,

        /// JSON arguments
        #[arg(num_args = 1.., last = true)]
        json: Vec<String>,
    },

    /// Retrieve a metadata attribute from the currently playing file
    ///
    /// See property 'metadata' for possible values
    Metadata {
        attribute: String,
    },

    /// Replace the given specifiers in the format string with their real-time values from mpv
    Format {
        /// Possible format specifiers are:
        /// %filename%
        /// %path%
        /// %title%
        /// %artist%
        /// %album%
        /// %album_artist%
        /// %composer%
        /// %genre%
        /// %date%
        /// %year%
        /// %comment%
        /// %track%
        /// %disc%
        /// %time%
        /// %duration%
        /// %percentage%
        /// %position%
        /// %playlist-count%
        /// %n% (newline)
        /// %% (escaped percent)
        /// %[% (escaped left bracket)
        /// %]% (escaped right bracket)
        ///
        /// Additionally, any valid property may be used.
        ///
        /// The format specifier may also appear in the form:
        /// 	%property?consequent:alternative%
        /// where the property evaluates to a boolean.
        ///
        /// Brackets may be used to group output such that if any
        /// format specifiers contained within fail to be retrieved,
        /// then none of the characters between the brackets are outputted.
        #[arg(
            help = "",
            long_help,
            verbatim_doc_comment,
        )]
        format_string: String,
    },

    /// Print all mpv events in real-time
    ///
    /// Additionally, observe a set of properties and inform about changes
    Observe {
        property: Vec<String>,
    },

    /// Block until one of the given events is triggered, or until one of the given properties is changed
    #[command(arg_required_else_help = true)]
    Wait {
        /// start-file: Happens right before a new file is loaded. When you receive this, the player is loading the file (or possibly already done with it).
        /// end-file: Happens after a file was unloaded. Typically, the player will load the next file right away, or quit if this was the last file.
        /// file-loaded: Happens after a file was loaded and begins playback.
        /// seek: Happens on seeking. (This might include cases when the player seeks internally, even without user interaction. This includes e.g. segment changes when playing ordered chapters Matroska files.)
        /// playback-restart: Start of playback after seek or after file was loaded.
        /// shutdown: Sent when the player quits, and the script should terminate. Normally handled automatically. See Details on the script initialization and lifecycle.
        /// video-reconfig: Happens on video output or filter reconfig.
        /// audio-reconfig: Happens on audio output or filter reconfig.
        #[arg(
            value_parser = [
                "start-file",
                "end-file",
                "file-loaded",
                "seek",
                "playback-restart",
                "shutdown",
                "video-reconfig",
                "audio-reconfig",
            ],
            num_args = 1..,
            help = "",
            long_help,
            verbatim_doc_comment,
        )]
        event: Vec<String>,

        #[arg(num_args = 1.., last = true)]
        property: Vec<String>,
    },

    /// Generate a shell completion script
    #[command(hide = true)]
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Clone, ValueEnum)]
enum AddType {
    File,
    Playlist,
}

#[derive(Clone, ValueEnum)]
enum VolumeMode {
    Absolute,
    Relative,
}

#[derive(Clone, ValueEnum)]
enum Toggle {
    On,
    Off,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    if let Commands::Completion { shell } = cli.command {
        let mut cmd = Cli::command();
        clap_complete::generate(shell, &mut cmd, env!("CARGO_CRATE_NAME"), &mut io::stdout());
        return Ok(());
    }

    let socket = &cli.socket;
    let mut mpv = match Mpv::connect(socket) {
        Ok(instance) => instance,
        Err(e) => {
            if matches!(cli.command, Commands::Add { .. }) {
                #[expect(clippy::zombie_processes)]
                Cmd::new("mpv")
                    .args([
                        "--no-terminal",
                        "--idle=once",
                        "--vid=no",
                        ("--input-ipc-server=".to_owned() + socket.as_str()).as_str(),
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

    match cli.command {
        Commands::Play => mpv.set_property("pause", false)?,
        Commands::Pause => mpv.set_property("pause", true)?,
        Commands::Toggle => mpv.command_arg("cycle", ["pause"])?,
        Commands::Next => mpv.command("playlist-next")?,
        Commands::Prev => mpv.command("playlist-prev")?,
        Commands::Seek { target, mode } => mpv.command_arg("seek", [target, mode])?,
        Commands::Restart => mpv.command_arg("seek", ["0", "absolute"])?,
        Commands::Kill => mpv.command("quit")?,

        Commands::Add { file, mode, r#type } => {
            let command = match r#type {
                AddType::File => "loadfile",
                AddType::Playlist => "loadlist",
            };
            for file in file {
                mpv.command_arg(command, [file.as_str(), mode.as_str()])?;
            }
        }

        Commands::Playlist => {
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
                let mut output = format!("{}\t{title}", i + 1);
                if entry.contains_key("current") {
                    output = output.reversed().to_string();
                }
                println!("{output}");
            }
        }

        Commands::Stop => mpv.command("stop")?,
        Commands::Clear => mpv.command("playlist-clear")?,

        Commands::Remove { id } => {
            let value: Value = id.map_or_else(|| "current".into(), Into::into);
            mpv.command_arg("playlist-remove", [value])?;
        }

        Commands::Move { from, to } => mpv.command_arg("playlist-move", [from, to])?,

        Commands::PlayNext { id } => {
            let pos = mpv.get_property("playlist-pos")?.as_u64().ok_or(Error::UnexpectedValue)?;
            mpv.command_arg("playlist-move", [id, pos + 1])?;
        }

        Commands::Position { id } => mpv.command_arg("playlist-play-index", [id])?,
        Commands::Shuffle => mpv.command("playlist-shuffle")?,

        Commands::Reverse => {
            let count = mpv.get_property("playlist-count")?.as_u64().ok_or(Error::UnexpectedValue)? - 1;
            for i in 0..count {
                mpv.command_arg("playlist-move", [count, i])?;
            }
        }

        Commands::LoopFile { arg } => {
            match arg {
                Some(arg) => mpv.set_property("loop-file", match arg {
                    Toggle::On => "inf".into(),
                    Toggle::Off => Value::Bool(false),
                })?,
                None => mpv.command_arg("cycle-values", ["loop-file".into(), Value::Bool(false), "inf".into()])?,
            }
        }

        Commands::LoopPlaylist { arg } => {
            match arg {
                Some(arg) => mpv.set_property("loop-playlist", match arg {
                    Toggle::On => "inf".into(),
                    Toggle::Off => Value::Bool(false),
                })?,
                None => mpv.command_arg("cycle-values", ["loop-playlist".into(), Value::Bool(false), "inf".into()])?,
            }
        }

        Commands::Volume { num, mode } => {
            match mode {
                VolumeMode::Absolute => mpv.set_property("volume", num)?,
                VolumeMode::Relative => mpv.add_property("volume", num)?,
            }
        }

        Commands::Mute { arg } => {
            match arg {
                Some(arg) => mpv.set_property("mute", matches!(arg, Toggle::On))?,
                None => mpv.command_arg("cycle", ["mute"])?,
            }
        }

        Commands::Set { json, property, value } => {
            let value = if json {
                value.parse::<Value>().map_err(Error::JsonError)?
            } else {
                value.as_str().into()
            };
            mpv.set_property(property.as_str(), value)?;
        }

        Commands::Get { json, property } => {
            let value = mpv.get_property(property.as_str())?;
            if json {
                println!("{value}");
            } else {
                println!("{}", value_to_string(&value)?);
            }
        }

        Commands::Run { command, args, json } => {
            let args = args.into_iter().map(|v| v.as_str().into());
            let json = json.into_iter().map(|v| v.parse::<Value>()).collect::<Result<Vec<_>, _>>().map_err(Error::JsonError)?;
            mpv.command_arg(command.as_str(), args.chain(json))?;
        }

        Commands::Metadata { attribute } => {
            let metadata = mpv.get_property(("metadata/by-key/".to_owned() + attribute.as_str()).as_str())?;
            println!("{}", value_to_string(&metadata)?);
        }

        Commands::Format { format_string } => {
            fn eval_format(mpv: &mut Mpv, metadata: &Map<String, Value>, spec: &str) -> Option<String> {
                fn format_duration(d: u64) -> String {
                    match (d % 60, (d / 60) % 60, d / 3600) {
                        (s, m, 0) => format!("{m:02}:{s:02}"),
                        (s, m, h) => format!("{h:02}:{m:02}:{s:02}"),
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
                    "time" => Some(format_duration(mpv.get_property("time-pos").ok()?.as_f64()? as u64)),
                    "duration" => Some(format_duration(mpv.get_property("duration").ok()?.as_f64()? as u64)),
                    "percentage" => Some((mpv.get_property("percent-pos").ok()?.as_f64()? as u64).to_string()),
                    "position" => Some(mpv.get_property("playlist-pos-1").ok()?.as_u64()?.to_string()),
                    _ if let Some(i) = spec.find('?') => {
                        let property = &spec[..i];
                        let pair = &spec[i + 1..];
                        let j = pair.find(':')?;
                        if mpv.get_property(property).ok()?.as_bool()? {
                            Some(pair[..j].to_owned())
                        } else {
                            Some(pair[j + 1..].to_owned())
                        }
                    }
                    _ if let Some(metadata) = metadata.get(spec) => value_to_string(metadata).ok(),
                    _ => value_to_string(&mpv.get_property(spec).ok()?).ok(),
                }
            }

            enum State {
                Raw,
                Spec,
                Skip(usize, bool),
            }
            use State::*;

            let mut input = format_string.as_str();
            let mut output = String::with_capacity(input.len());
            let metadata = if let Ok(metadata) = mpv.get_property("metadata") {
                metadata.as_object().ok_or(Error::UnexpectedValue)?
                    .iter().map(|(k, v)| (k.to_lowercase(), v.clone())).collect()
            } else {
                Map::<String, Value>::new()
            };

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
                                        output += pop.as_str();
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
            print!("{output}");
        }

        Commands::Observe { property: properties } => {
            for (i, property) in properties.into_iter().enumerate() {
                mpv.observe_property(i as isize + 1, property.as_str())?;
            }
            while let Ok(response) = mpv.listen_raw() {
                println!("{response}");
            }
        }

        Commands::Wait { event: events, property: properties } => {
            for (i, property) in properties.iter().enumerate() {
                mpv.observe_property(i as isize + 1, property)?;
            }
            // Needed since the observe_property command itself emits a property-change event
            let mut seen = Vec::new();
            loop {
                let event = mpv.listen()?;
                if let Some(Value::String(e)) = event.get("event") {
                    if e == "property-change"
                            && let Some(Value::String(property)) = event.get("name")
                            && let Some(i) = properties.iter().position(|v| v == property) {
                        if seen.contains(&property) {
                            break;
                        }
                        seen.push(&properties[i]);
                    } else if events.contains(e) {
                        break;
                    }
                }
            }
        }

        Commands::Completion { .. } => unreachable!(),
    }

    Ok(())
}
