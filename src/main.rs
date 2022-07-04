#[macro_use]
extern crate clap;

extern crate serde;
extern crate serde_json;
extern crate colored;
extern crate mpvipc;

#[macro_use]
mod macros;

use std::process::exit;

use clap::{AppSettings, Arg, SubCommand};
use colored::*;
use mpvipc::*;

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
            .subcommand(SubCommand::with_name("mute")
                .about("<ON|OFF|TOGGLE>\n\
                Controls whether audio output is muted.")
                .arg(Arg::with_name("arg")
                    .value_name("on|off|toggle")
                    .possible_values(&["on", "off", "toggle"])
                    .help("Defines if muting audio is enabled or disabled")
                    .required(true)))
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
                    .conflicts_with("increase")))
            .subcommand(SubCommand::with_name("speed")
                .about("[OPTIONS] <NUM>\n\
                Sets the playback speed to <NUM> (0.01-100). \
                Use with --increase or --decrease to relatively change the speed")
                .arg(Arg::with_name("num")
                    .value_name("NUM")
                    .required(true))
                .arg(Arg::with_name("increase")
                    .short("i")
                    .long("increase")
                    .help("If set will increase speed by <NUM>")
                    .takes_value(false))
                .arg(Arg::with_name("decrease")
                    .short("d")
                    .long("decrease")
                    .help("If set will decrease speed by <NUM>")
                    .takes_value(false)
                    .conflicts_with("increase")))
            .subcommand(SubCommand::with_name("loop-file")
                .about("<ON|OFF|TOGGLE>\n\
                Controls whether the current file should be repeatet after playback.")
                .arg(Arg::with_name("arg")
                    .value_name("on|off|toggle")
                    .possible_values(&["on", "off", "toggle"])
                    .help("Defines if looping current file is enabled or disabled")
                    .required(true)))
            .subcommand(SubCommand::with_name("loop-playlist")
                .about("<ON|OFF|TOGGLE>\n\
                Controls whether the playlist should be repeatet after the end is reached.")
                .arg(Arg::with_name("arg")
                    .value_name("on|off|toggle")
                    .possible_values(&["on", "off", "toggle"])
                    .help("Defines if looping playlist is enabled or disabled")
                    .required(true))))
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
        .subcommand(SubCommand::with_name("kill")
            .about("Kill the mpv process controlling the socket"))
        .subcommand(SubCommand::with_name("format")
                .about("Replaces the following arguments with their real-time values from mpv. \n\
                        Use with \"\" for multiple arguments.\n\n\
                        %file%, %path%, %title%, %artist%, %album%, %albumartist%, %composer%, %genre%, %year%, %comment%, \
                        %track%, %disc%, %playlistlength%, %position%\n\n\
                        EXAMPLE: format \"%artist% - %title% (%album%)\"")
                .arg(Arg::with_name("input")
                    .value_name("INPUT-STRING")
                    .help("%file%: \n\
                        %path%: \n\
                        %title%: \n\
                        %artist%: \n\
                        %album%: \n\
                        %albumartist%: \n\
                        %composer%: \n\
                        %genre%: \n\
                        %year%: \n\
                        %comment%: \n\
                        %track%: \n\
                        %disc%: \n\
                        %playlistlength%: \n\
                        %position%: ")
                    .required(true)))
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
            .subcommand(SubCommand::with_name("raw")
                .about("Prints all mpv events in real-time in raw output format (JSON)."))
            .subcommand(SubCommand::with_name("observe")
                .about("<PROPERTIES>\n\
                Observes a comma separated set of properties and informs about changes.")
                .arg(Arg::with_name("properties")
                .value_name("PROPERTIES")
                .required(true))
                .arg(Arg::with_name("hide-data")
                    .short("h")
                    .long("hide-data")
                    .help("Hides the new content of the observed property (useful for properties with a lot of data)")
                    .takes_value(false)))
            .subcommand(SubCommand::with_name("wait-for")
                .about("<EVENT>\n\
                Runs until the mpv event <EVENT> is triggered. See --help for possible events.")
                .arg(Arg::with_name("event")
                    .value_name("EVENT")
                    .possible_values(&["Shutdown",
                        "StartFile",
                        "EndFile",
                        "FileLoaded",
                        "TracksChanged",
                        "TrackSwitched",
                        "Idle",
                        "Pause",
                        "Unpause",
                        "Tick",
                        "VideoReconfig",
                        "AudioReconfig",
                        "MetadataUpdate",
                        "Seek",
                        "PlaybackRestart",
                        "ChapterChange",
                        "Unimplemented"])
                    .required(true))))
        .subcommand(SubCommand::with_name("stop")
            .about("Stop playback and clear playlist."))
        .subcommand(SubCommand::with_name("playlist")
            .about("Playlist related commands. See --help for available subcommands")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(SubCommand::with_name("add")
                .about("[OPTIONS] <FILE|PLAYLIST>\n\
                Load the given file or playlist and play it. See -h for options.")
                .visible_alias("load")
                .arg(Arg::with_name("file")
                    .value_name("FILE")
                    .required(true))
                .arg(Arg::with_name("mode")
                    .short("m")
                    .long("mode")
                    .possible_values(&["replace", "append"])
                    .hide_possible_values(true)
                    .default_value("replace")
                    .help("<replace|append>\n\
                    <replace>: Stop playback of the current file, and play the new file immediately.\n\
                    <append>: Append the file to the playlist.\n\n")
                    .takes_value(true))
                .arg(Arg::with_name("type")
                    .short("t")
                    .long("type")
                    .possible_values(&["file", "playlist"])
                    .default_value("file")))
            .subcommand(SubCommand::with_name("show")
                .about("Prints the 0-based playlist."))
            .subcommand(SubCommand::with_name("clear")
                .about("Clear the playlist, except the currently played file."))
            .subcommand(SubCommand::with_name("shuffle")
                .about("Shuffles the playlist"))
            .subcommand(SubCommand::with_name("remove-id")
                .about("Removes <ID> from the 0-based playlist. If <ID> is currently playing, playback will stop")
                .visible_alias("remove")
                .arg(Arg::with_name("id")
                    .value_name("ID")
                    .help("Defines the id that should be removed from the 0-based playlist")
                    .required(true)))
            .subcommand(SubCommand::with_name("move-id")
                .about("Moves the playlist entry at position <FROM> to position <TO>")
                .visible_alias("move")
                .arg(Arg::with_name("from")
                    .value_name("FROM-ID")
                    .help("Defines the id that should be moved to position <TO>")
                    .required(true))
                .arg(Arg::with_name("to")
                    .value_name("TO-ID")
                    .help("Defines the id where the entry at <FROM> should be moved to")
                    .required(true)))
            .subcommand(SubCommand::with_name("play-id-next")
                .about("Moves the playlist entry at position <ID> after the currently playing ID")
                .visible_alias("next")
                .visible_alias("play-next")
                .arg(Arg::with_name("id")
                    .value_name("ID")
                    .help("Defines the id that should be played next")
                    .required(true)))
            .subcommand(SubCommand::with_name("play-id")
                .about("Plays the file at <ID> in the playlist")
                .visible_alias("play")
                .arg(Arg::with_name("id")
                    .value_name("ID")
                    .help("Defines the id that should be played next")
                    .required(true)))
            .subcommand(SubCommand::with_name("reverse")
                .about("Reverses the playlist")
                .visible_alias("rev")))
        .get_matches();

    //Input socket is always present, therefore unwrap
    let mpv;
    match Mpv::connect(matches.value_of("socket").unwrap()) {
        Ok(instance) => {
            mpv = instance;
        }
        Err(msg) => error!("Error: {}", msg),
    }

    // The user used the sub-command `pause`
    match matches.subcommand() {
        ("pause", _) => {
            if let Err(msg) = mpv.pause() {
                error!("Error: {}", msg);
            }
        }

        // The user used the sub-command `toggle`
        ("toggle", _) => {
            if let Err(msg) = mpv.toggle() {
                error!("Error: {}", msg);
            }
        }

        // The user used the sub-command `pause`
        ("next", _) => {
            if let Err(msg) = mpv.next() {
                error!("Error: {}", msg);
            }
        }

        // The user used the sub-command `prev`
        ("prev", _) => {
            if let Err(msg) = mpv.prev() {
                error!("Error: {}", msg);
            }
        }

        // The user used the sub-command `restart`
        ("restart", _) => {
            if let Err(msg) = mpv.restart() {
                error!("Error: {}", msg);
            }
        }

        // The user used the sub-command `stop`
        ("stop", _) => {
            if let Err(msg) = mpv.stop() {
                error!("Error: {}", msg);
            }
        }

        // The user used the sub-command `kill`
        ("kill", _) => {
            if let Err(msg) = mpv.kill() {
                error!("Error: {}", msg);
            }
        }

        // The user used the sub-command `format`
        ("format", Some(input)) => {
            let input_str = input.value_of("input").unwrap();
            let mut output_string = String::from(input_str);

            match mpv.get_metadata() {
                Ok(metadata) => {
                    if input_str.contains("%title%") {
                        if metadata.contains_key("title") {
                            if let MpvDataType::String(ref s) = metadata["title"] {
                                output_string = output_string.replace("%title%", s);
                            }
                        } else {
                            match mpv.get_property::<String>("media-title") {
                                Ok(media_title) => {
                                    output_string = output_string.replace("%title%", &media_title);
                                }
                                Err(msg) => error!("Error: {}", msg),
                            }
                        }
                    }

                    if input_str.contains("%artist%") {
                        if metadata.contains_key("artist") {
                            if let MpvDataType::String(ref s) = metadata["artist"] {
                                output_string = output_string.replace("%artist%", s);
                            }
                        } else {
                            output_string = output_string.replace("%artist%", "");
                        }
                    }

                    if input_str.contains("%album%") {
                        if metadata.contains_key("album") {
                            if let MpvDataType::String(ref s) = metadata["album"] {
                                output_string = output_string.replace("%album%", s);
                            }
                        } else {
                            output_string = output_string.replace("%album%", "");
                        }
                    }

                    if input_str.contains("%albumartist%") {
                        if metadata.contains_key("album_artist") {
                            if let MpvDataType::String(ref s) = metadata["album_artist"] {
                                output_string = output_string.replace("%albumartist%", s);
                            }
                        } else {
                            output_string = output_string.replace("%albumartist%", "");
                        }
                    }

                    if input_str.contains("%date%") {
                        if metadata.contains_key("date") {
                            if let MpvDataType::String(ref s) = metadata["date"] {
                                output_string = output_string.replace("%date%", s);
                            }
                        } else {
                            output_string = output_string.replace("%date%", "");
                        }
                    }

                    if input_str.contains("%track%") {
                        if metadata.contains_key("track") {
                            if let MpvDataType::String(ref s) = metadata["track"] {
                                output_string = output_string.replace("%track%", s);
                            }
                        } else {
                            output_string = output_string.replace("%track%", "");
                        }
                    }

                    if input_str.contains("%genre%") {
                        if metadata.contains_key("genre") {
                            if let MpvDataType::String(ref s) = metadata["genre"] {
                                output_string = output_string.replace("%genre%", s);
                            }
                        } else {
                            output_string = output_string.replace("%genre%", "");
                        }
                    }

                    if input_str.contains("%composer%") {
                        if metadata.contains_key("composer") {
                            if let MpvDataType::String(ref s) = metadata["composer"] {
                                output_string = output_string.replace("%composer%", s);
                            }
                        } else {
                            output_string = output_string.replace("%composer%", "");
                        }
                    }

                    if input_str.contains("%comment%") {
                        if metadata.contains_key("comment") {
                            if let MpvDataType::String(ref s) = metadata["comment"] {
                                output_string = output_string.replace("%comment%", s);
                            }
                        } else {
                            output_string = output_string.replace("%comment%", "");
                        }
                    }

                    if input_str.contains("%disc%") {
                        if metadata.contains_key("disc") {
                            if let MpvDataType::String(ref s) = metadata["disc"] {
                                output_string = output_string.replace("%disc%", s);
                            }
                        } else {
                            output_string = output_string.replace("%disc%", "");
                        }
                    }

                    if input_str.contains("%path%") {
                        match mpv.get_property::<String>("path") {
                            Ok(path) => {
                                output_string = output_string.replace("%path%", &path);
                            }
                            Err(msg) => error!("Error: {}", msg),
                        }
                    }

                    if input_str.contains("%file%") {
                        match mpv.get_property::<String>("filename") {
                            Ok(filename) => {
                                output_string = output_string.replace("%file%", &filename);
                            }
                            Err(msg) => error!("Error: {}", msg),
                        }
                    }

                    if input_str.contains("%position%") {
                        match mpv.get_property::<String>("playlist-pos") {
                            Ok(position) => {
                                output_string = output_string.replace("%position%", &position);
                            }
                            Err(msg) => error!("Error: {}", msg),
                        }
                    }

                    if input_str.contains("%playlistlength%") {
                        match mpv.get_property::<String>("playlist-count") {
                            Ok(playlist_count) => {
                                output_string =
                                    output_string.replace("%playlistlength%", &playlist_count);
                            }
                            Err(msg) => error!("Error: {}", msg),
                        }
                    }
                }
                Err(msg) => error!("Error: {}", msg),
            }
            println!("{}", output_string);
        }

        // The user used the sub-command `get`
        ("get", Some(get_matches)) => {
            match get_matches.subcommand() {
                ("property", Some(property_matches)) => {
                    let property = property_matches.value_of("property").unwrap();
                    match mpv.get_property_string(property) {
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
                                    if let MpvDataType::String(ref v) = *value {
                                        println!("{}: {}", key, v);
                                    } else {
                                        println!("{}: {:?}", key, value);
                                    }
                                }
                            }
                        }
                        Err(why) => error!("Error: {}", why),
                    }
                }

                (_, _) => unreachable!(),
            }
        }

        // The user used the sub-command `set`
        ("set", Some(set_matches)) => {
            match set_matches.subcommand() {
                ("mute", Some(mute_matches)) => {
                    match mute_matches.value_of("arg").unwrap() {
                        "on" => {
                            if let Err(msg) = mpv.set_mute(Switch::On) {
                                error!("Error: {}", msg);
                            }
                        }
                        "off" => {
                            if let Err(msg) = mpv.set_mute(Switch::Off) {
                                error!("Error: {}", msg);
                            }
                        }
                        "toggle" => {
                            if let Err(msg) = mpv.set_mute(Switch::Toggle) {
                                error!("Error: {}", msg);
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                ("property", Some(property_matches)) => {
                    let property = property_matches.value_of("property").unwrap();
                    let value = property_matches.value_of("value").unwrap();
                    if let Err(error_msg) = mpv.set_property(property, value.to_string()) {
                        error!("Error: {}", error_msg);
                    }
                }

                ("volume", Some(volume_matches)) => {
                    let num = volume_matches.value_of("num").unwrap();
                    if volume_matches.is_present("increase") {
                        if let Err(msg) = mpv.set_volume(
                            num.parse::<f64>().unwrap(),
                            NumberChangeOptions::Increase,
                        )
                        {
                            error!("Error: {}", msg);
                        }
                    } else if volume_matches.is_present("decrease") {
                        if let Err(msg) = mpv.set_volume(
                            num.parse::<f64>().unwrap(),
                            NumberChangeOptions::Decrease,
                        )
                        {
                            error!("Error: {}", msg);
                        }
                    } else {
                        if let Err(msg) = mpv.set_volume(
                            num.parse::<f64>().unwrap(),
                            NumberChangeOptions::Absolute,
                        )
                        {
                            error!("Error: {}", msg);
                        }
                    }
                }

                ("speed", Some(speed_matches)) => {
                    let num = speed_matches.value_of("num").unwrap();
                    if speed_matches.is_present("increase") {
                        if let Err(msg) = mpv.set_speed(
                            num.parse::<f64>().unwrap(),
                            NumberChangeOptions::Increase,
                        )
                        {
                            error!("Error: {}", msg);
                        }
                    } else if speed_matches.is_present("decrease") {
                        if let Err(msg) = mpv.set_speed(
                            num.parse::<f64>().unwrap(),
                            NumberChangeOptions::Decrease,
                        )
                        {
                            error!("Error: {}", msg);
                        }
                    } else {
                        if let Err(msg) = mpv.set_speed(
                            num.parse::<f64>().unwrap(),
                            NumberChangeOptions::Absolute,
                        )
                        {
                            error!("Error: {}", msg);
                        }
                    }
                }

                ("loop-file", Some(loop_playlist_matches)) => {
                    match loop_playlist_matches.value_of("arg").unwrap() {
                        "on" => {
                            if let Err(msg) = mpv.set_loop_file(Switch::On) {
                                error!("Error: {}", msg);
                            }
                        }
                        "off" => {
                            if let Err(msg) = mpv.set_loop_file(Switch::Off) {
                                error!("Error: {}", msg);
                            }
                        }
                        "toggle" => {
                            if let Err(msg) = mpv.set_loop_file(Switch::Toggle) {
                                error!("Error: {}", msg);
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                ("loop-playlist", Some(loop_playlist_matches)) => {
                    match loop_playlist_matches.value_of("arg").unwrap() {
                        "on" => {
                            if let Err(msg) = mpv.set_loop_playlist(Switch::On) {
                                error!("Error: {}", msg);
                            }
                        }
                        "off" => {
                            if let Err(msg) = mpv.set_loop_playlist(Switch::Off) {
                                error!("Error: {}", msg);
                            }
                        }
                        "toggle" => {
                            if let Err(msg) = mpv.set_loop_playlist(Switch::Toggle) {
                                error!("Error: {}", msg);
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                (_, _) => unreachable!(),
            }
        }

        // The user used the sub-command `seek`
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

        // The user used the sub-command `events`
        ("events", Some(events_matches)) => {
            match events_matches.subcommand() {
                ("wait-for", Some(wait_for_matches)) => {
                    let watched_event = wait_for_matches.value_of("event").unwrap();
                    let mut mpv = mpv;
                    loop {
                        match mpv.event_listen() {
                            Ok(event) => {
                                let event_str = &format!("{:?}", event);
                                if event_str == watched_event {
                                    break;
                                }
                            }
                            Err(msg) => {
                                error!("Error: {}", msg);
                            }
                        }
                    }
                }
                ("show", _) => {
                    let mut mpv = mpv;
                    loop {
                        match mpv.event_listen() {
                            Ok(event) => {
                                println!("{:?}", event);
                            }
                            Err(msg) => {
                                error!("Error: {}", msg);
                            }
                        }
                    }
                }

                ("observe", Some(observe_matches)) => {
                    let observed_properties = observe_matches.value_of("properties").unwrap();
                    let props: Vec<&str> = observed_properties.split(',').collect();
                    for (i, property) in props.iter().enumerate() {
                        mpv.observe_property(&(i as isize + 1), property).unwrap();
                    }
                    let mut mpv = mpv;
                    loop {
                        match mpv.event_listen() {
                            Ok(event) => {
                                if let Event::PropertyChange { id, property } = event {
                                    if observe_matches.is_present("hide-data") {
                                        match property {
                                            Property::Duration(_) => {
                                                println!("PropertyChange (name=duration, id={}", id)
                                            }
                                            Property::Metadata(_) => {
                                                println!("PropertyChange (name=metadata, id={}", id)
                                            }
                                            Property::Path(_) => {
                                                println!("PropertyChange (name=property, id={}", id)
                                            }
                                            Property::Pause(_) => {
                                                println!("PropertyChange (name=pause, id={}", id)
                                            }
                                            Property::PlaybackTime(_) => {
                                                println!("PropertyChange (name=pause, id={}", id)
                                            }
                                            Property::Unknown { name, data: _ } => {
                                                println!("PropertyChange (name={}, id={}", name, id)
                                            }
                                        }
                                    } else {
                                        match property {
                                            Property::Duration(duration) => println!(
                                                "PropertyChange (name=duration, id={}, data={:?}",
                                                id,
                                                duration
                                            ),
                                            Property::Metadata(metadata) => println!(
                                                "PropertyChange (name=metadata, id={}, data={:?}",
                                                id,
                                                metadata
                                            ),
                                            Property::Path(path) => println!(
                                                "PropertyChange (name=property, id={}, data={:?}",
                                                id,
                                                path
                                            ),
                                            Property::Pause(paused) => println!(
                                                "PropertyChange (name=pause, id={}, data={}",
                                                id, paused
                                            ),
                                            Property::PlaybackTime(playback_time) => println!(
                                                "PropertyChange (name=playback-time, id={}, data={:?}",
                                                id,
                                                playback_time
                                            ),
                                            Property::Unknown { name, data } => println!(
                                                "PropertyChange (name={}, id={}, data={:?}",
                                                name, id, data
                                            ),
                                        }
                                    }
                                }
                            }
                            Err(msg) => {
                                error!("Error: {}", msg);
                            }
                        }

                    }
                }

                ("raw", _) => {
                    mpv.observe_property(&99isize, "duration").unwrap();
                    let mut mpv = mpv;
                    loop {
                        let event = mpv.event_listen_raw();
                        //print!("{}", event);
                        println!("{}", event);
                    }
                }

                (_, _) => unreachable!(),
            }
        }

        // The user used the sub-command `playlist`
        ("playlist", Some(playlist_matches)) => {
            match playlist_matches.subcommand() {
                ("add", Some(add_matches)) => {
                    let file = add_matches.value_of("file").unwrap();
                    let file_type = match add_matches.value_of("type").unwrap() {
                        "file" => PlaylistAddTypeOptions::File,
                        "playlist" => PlaylistAddTypeOptions::Playlist,
                        _ => unreachable!(),
                    };
                    match add_matches.value_of("mode").unwrap() {
                        "replace" => {
                            if let Err(msg) = mpv.playlist_add(
                                file,
                                file_type,
                                PlaylistAddOptions::Replace,
                            )
                            {
                                error!("Error: {}", msg);
                            }
                        }

                        "append" => {
                            if let Err(msg) = mpv.playlist_add(
                                file,
                                file_type,
                                PlaylistAddOptions::Append,
                            )
                            {
                                error!("Error: {}", msg);
                            }
                        }
                        _ => unreachable!(),
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
                    if let Err(msg) = mpv.playlist_remove_id(
                        remove_id_matches
                            .value_of("id")
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    )
                    {
                        error!("Error: {}", msg);
                    }
                }

                ("move-id", Some(move_id_matches)) => {
                    if let Err(msg) = mpv.playlist_move_id(
                        move_id_matches
                            .value_of("from")
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                        move_id_matches
                            .value_of("to")
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    )
                    {
                        error!("Error: {}", msg);
                    }
                }

                ("play-id-next", Some(play_next_matches)) => {
                    if let Err(msg) = mpv.playlist_play_next(
                        play_next_matches
                            .value_of("id")
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    )
                    {
                        error!("Error: {}", msg);
                    }
                }

                ("play-id", Some(play_id_matches)) => {
                    if let Err(msg) = mpv.playlist_play_id(
                        play_id_matches
                            .value_of("id")
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    )
                    {
                        error!("Error: {}", msg);
                    }
                }

                ("reverse", _) => {
                    if let Ok(playlist) = mpv.get_playlist() {
                        let Playlist(entries) = playlist;
                        let mut i = 0usize;
                        while i < entries.len() {
                            if let Err(msg) = mpv.playlist_move_id((entries.len() -1) as usize, i)
                            {
                                error!("Error: {}", msg);
                            }
                            i += 1;
                        }
                    }
                }

                ("show", _) => {
                    //Show the playlist
                    if let Ok(playlist) = mpv.get_playlist() {
                        let Playlist(entries) = playlist;
                        for entry in entries.iter() {
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

    //mpv.disconnect();
}
