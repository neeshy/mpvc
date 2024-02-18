use builtin;
use str;

set edit:completion:arg-completer[mpvc] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'mpvc'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'mpvc'= {
            cand -S 'S'
            cand --socket 'socket'
            cand -h 'Print help'
            cand --help 'Print help'
            cand play 'Resume playback'
            cand pause 'Pause playback'
            cand toggle 'Toggle between play and pause'
            cand next 'Start playing the next entry in the playlist'
            cand prev 'Start playing the previous entry in the playlist'
            cand seek 'Set the playback position. By default seeks by a relative amount of seconds. You may use negative values.'
            cand restart 'Restart playback of the current file (equivalent to ''seek -m absolute 0'')'
            cand kill 'Kill the mpv process controlling the socket'
            cand add 'Load the given file or playlist and play it'
            cand playlist 'Print playlist entries'
            cand stop 'Stop playback and clear the playlist'
            cand clear 'Clear the playlist, except the currently playing file'
            cand remove 'Remove the given entry from the playlist (0-indexed). If the entry is currently playing, playback will stop.'
            cand move 'Move the given playlist entry to a new position'
            cand play-next 'Move the given playlist entry to be after the currently playing file'
            cand position 'Play the given entry in the playlist'
            cand shuffle 'Shuffle the playlist'
            cand reverse 'Reverse the playlist'
            cand loop-file 'Control whether the current file should be repeated after playback'
            cand loop-playlist 'Control whether the playlist should be repeated after the end is reached'
            cand volume 'Control the volume level'
            cand mute 'Control whether audio output is muted'
            cand set 'Set a property to the given value'
            cand get 'Retrieve a property (see property ''property-list'' for possible values)'
            cand run 'Run an mpv command'
            cand metadata 'Retrieve a metadata attribute from the currently playing file (see property ''metadata'' for possible values)'
            cand format 'Replace the given specifiers in the format string with their real-time values from mpv'
            cand observe 'Print all mpv events in real-time. Additionally, observe a set of properties and inform about changes.'
            cand wait 'Block until one of the given events is triggered, or until one of the given properties is changed'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'mpvc;play'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;pause'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;toggle'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;next'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;prev'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;seek'= {
            cand -m '<relative|relative-percent|absolute|absolute-percent> <relative>: Seek relative to the current position (a negative value seeks backwards) <absolute>: Seek to a given position (a negative value seeks starting from the end of the file) <absolute-percent>: Seek to a given position in percentage <relative-percent>: Seek relative to the current position in percentage '
            cand --mode '<relative|relative-percent|absolute|absolute-percent> <relative>: Seek relative to the current position (a negative value seeks backwards) <absolute>: Seek to a given position (a negative value seeks starting from the end of the file) <absolute-percent>: Seek to a given position in percentage <relative-percent>: Seek relative to the current position in percentage '
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;restart'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;kill'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;add'= {
            cand -m '<replace|append|append-play|append-next> <replace>: Stop playback of the current file, and play the new file immediately <append>: Append the file to the playlist <append-play>: Append the file, and if nothing is currently playing, start playback <append-next>: Append the file to the playlist, and place it in the next position '
            cand --mode '<replace|append|append-play|append-next> <replace>: Stop playback of the current file, and play the new file immediately <append>: Append the file to the playlist <append-play>: Append the file, and if nothing is currently playing, start playback <append-next>: Append the file to the playlist, and place it in the next position '
            cand -t 't'
            cand --type 'type'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;playlist'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;stop'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;clear'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;remove'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;move'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;play-next'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;position'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;shuffle'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;reverse'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;loop-file'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;loop-playlist'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;volume'= {
            cand -m '<absolute|relative> <absolute>: Set the volume <relative>: Change the volume relative to the current level (a negative value decreases the level) '
            cand --mode '<absolute|relative> <absolute>: Set the volume <relative>: Change the volume relative to the current level (a negative value decreases the level) '
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;mute'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;set'= {
            cand -j 'Parse <value> as JSON'
            cand --json 'Parse <value> as JSON'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;get'= {
            cand -j 'Print property as JSON'
            cand --json 'Print property as JSON'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;run'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;metadata'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;format'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;observe'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;wait'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'mpvc;help'= {
            cand play 'Resume playback'
            cand pause 'Pause playback'
            cand toggle 'Toggle between play and pause'
            cand next 'Start playing the next entry in the playlist'
            cand prev 'Start playing the previous entry in the playlist'
            cand seek 'Set the playback position. By default seeks by a relative amount of seconds. You may use negative values.'
            cand restart 'Restart playback of the current file (equivalent to ''seek -m absolute 0'')'
            cand kill 'Kill the mpv process controlling the socket'
            cand add 'Load the given file or playlist and play it'
            cand playlist 'Print playlist entries'
            cand stop 'Stop playback and clear the playlist'
            cand clear 'Clear the playlist, except the currently playing file'
            cand remove 'Remove the given entry from the playlist (0-indexed). If the entry is currently playing, playback will stop.'
            cand move 'Move the given playlist entry to a new position'
            cand play-next 'Move the given playlist entry to be after the currently playing file'
            cand position 'Play the given entry in the playlist'
            cand shuffle 'Shuffle the playlist'
            cand reverse 'Reverse the playlist'
            cand loop-file 'Control whether the current file should be repeated after playback'
            cand loop-playlist 'Control whether the playlist should be repeated after the end is reached'
            cand volume 'Control the volume level'
            cand mute 'Control whether audio output is muted'
            cand set 'Set a property to the given value'
            cand get 'Retrieve a property (see property ''property-list'' for possible values)'
            cand run 'Run an mpv command'
            cand metadata 'Retrieve a metadata attribute from the currently playing file (see property ''metadata'' for possible values)'
            cand format 'Replace the given specifiers in the format string with their real-time values from mpv'
            cand observe 'Print all mpv events in real-time. Additionally, observe a set of properties and inform about changes.'
            cand wait 'Block until one of the given events is triggered, or until one of the given properties is changed'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'mpvc;help;play'= {
        }
        &'mpvc;help;pause'= {
        }
        &'mpvc;help;toggle'= {
        }
        &'mpvc;help;next'= {
        }
        &'mpvc;help;prev'= {
        }
        &'mpvc;help;seek'= {
        }
        &'mpvc;help;restart'= {
        }
        &'mpvc;help;kill'= {
        }
        &'mpvc;help;add'= {
        }
        &'mpvc;help;playlist'= {
        }
        &'mpvc;help;stop'= {
        }
        &'mpvc;help;clear'= {
        }
        &'mpvc;help;remove'= {
        }
        &'mpvc;help;move'= {
        }
        &'mpvc;help;play-next'= {
        }
        &'mpvc;help;position'= {
        }
        &'mpvc;help;shuffle'= {
        }
        &'mpvc;help;reverse'= {
        }
        &'mpvc;help;loop-file'= {
        }
        &'mpvc;help;loop-playlist'= {
        }
        &'mpvc;help;volume'= {
        }
        &'mpvc;help;mute'= {
        }
        &'mpvc;help;set'= {
        }
        &'mpvc;help;get'= {
        }
        &'mpvc;help;run'= {
        }
        &'mpvc;help;metadata'= {
        }
        &'mpvc;help;format'= {
        }
        &'mpvc;help;observe'= {
        }
        &'mpvc;help;wait'= {
        }
        &'mpvc;help;help'= {
        }
    ]
    $completions[$command]
}
