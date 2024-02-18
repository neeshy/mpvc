#compdef mpvc

autoload -U is-at-least

_mpvc() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-S+[]:/path/to/socket: ' \
'--socket=[]:/path/to/socket: ' \
'-h[Print help]' \
'--help[Print help]' \
":: :_mpvc_commands" \
"*::: :->mpvc" \
&& ret=0
    case $state in
    (mpvc)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:mpvc-command-$line[1]:"
        case $line[1] in
            (play)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(pause)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(toggle)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(next)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(prev)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(seek)
_arguments "${_arguments_options[@]}" \
'-m+[<relative|relative-percent|absolute|absolute-percent> <relative>\: Seek relative to the current position (a negative value seeks backwards) <absolute>\: Seek to a given position (a negative value seeks starting from the end of the file) <absolute-percent>\: Seek to a given position in percentage <relative-percent>\: Seek relative to the current position in percentage ]: :(relative relative-percent absolute absolute-percent)' \
'--mode=[<relative|relative-percent|absolute|absolute-percent> <relative>\: Seek relative to the current position (a negative value seeks backwards) <absolute>\: Seek to a given position (a negative value seeks starting from the end of the file) <absolute-percent>\: Seek to a given position in percentage <relative-percent>\: Seek relative to the current position in percentage ]: :(relative relative-percent absolute absolute-percent)' \
'-h[Print help]' \
'--help[Print help]' \
':target:' \
&& ret=0
;;
(restart)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(kill)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" \
'-m+[<replace|append|append-play|append-next> <replace>\: Stop playback of the current file, and play the new file immediately <append>\: Append the file to the playlist <append-play>\: Append the file, and if nothing is currently playing, start playback <append-next>\: Append the file to the playlist, and place it in the next position ]: :(replace append append-play append-next)' \
'--mode=[<replace|append|append-play|append-next> <replace>\: Stop playback of the current file, and play the new file immediately <append>\: Append the file to the playlist <append-play>\: Append the file, and if nothing is currently playing, start playback <append-next>\: Append the file to the playlist, and place it in the next position ]: :(replace append append-play append-next)' \
'-t+[]: :(file playlist)' \
'--type=[]: :(file playlist)' \
'-h[Print help]' \
'--help[Print help]' \
'*::file:' \
&& ret=0
;;
(playlist)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(clear)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':id:' \
&& ret=0
;;
(move)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':from:' \
':to:' \
&& ret=0
;;
(play-next)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':id:' \
&& ret=0
;;
(position)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':id:' \
&& ret=0
;;
(shuffle)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(loop-file)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':arg:(on off toggle)' \
&& ret=0
;;
(loop-playlist)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':arg:(on off toggle)' \
&& ret=0
;;
(volume)
_arguments "${_arguments_options[@]}" \
'-m+[<absolute|relative> <absolute>\: Set the volume <relative>\: Change the volume relative to the current level (a negative value decreases the level) ]: :(absolute relative)' \
'--mode=[<absolute|relative> <absolute>\: Set the volume <relative>\: Change the volume relative to the current level (a negative value decreases the level) ]: :(absolute relative)' \
'-h[Print help]' \
'--help[Print help]' \
':num:' \
&& ret=0
;;
(mute)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':arg:(on off toggle)' \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" \
'-j[Parse <value> as JSON]' \
'--json[Parse <value> as JSON]' \
'-h[Print help]' \
'--help[Print help]' \
':property:' \
':value:' \
&& ret=0
;;
(get)
_arguments "${_arguments_options[@]}" \
'-j[Print property as JSON]' \
'--json[Print property as JSON]' \
'-h[Print help]' \
'--help[Print help]' \
':property:' \
&& ret=0
;;
(run)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':command:' \
'*::args -- String arguments:' \
&& ret=0
;;
(metadata)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':attribute:' \
&& ret=0
;;
(format)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
':format-string -- Possible format specifiers are\:
%filename%
%path%
%title%
%artist%
%album%
%album_artist%
%composer%
%genre%
%date%
%year%
%comment%
%track%
%disc%
%time%
%duration%
%percentage%
%position%
%playlist-count%
%n% (newline)
%% (escaped percent)
%\[% (escaped left bracket)
%\]% (escaped right bracket)

Additionally, any valid property may be used.

The format specifier may also appear in the form\:
	%property?consequent\:alternative%
where the property evaluates to a boolean.

Brackets may be used to group output such that if any
format specifiers contained within fail to be retrieved,
then none of the characters between the brackets are outputted.:' \
&& ret=0
;;
(observe)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
'*::property:' \
&& ret=0
;;
(wait)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
'*::event:(start-file end-file file-loaded seek playback-restart shutdown video-reconfig audio-reconfig)' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_mpvc__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:mpvc-help-command-$line[1]:"
        case $line[1] in
            (play)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(pause)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(toggle)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(next)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(prev)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(seek)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(restart)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(kill)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(playlist)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(clear)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(move)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(play-next)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(position)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(shuffle)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(reverse)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(loop-file)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(loop-playlist)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(volume)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(mute)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(get)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(run)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(metadata)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(format)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(observe)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(wait)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_mpvc_commands] )) ||
_mpvc_commands() {
    local commands; commands=(
'play:Resume playback' \
'pause:Pause playback' \
'toggle:Toggle between play and pause' \
'next:Start playing the next entry in the playlist' \
'prev:Start playing the previous entry in the playlist' \
'seek:Set the playback position. By default seeks by a relative amount of seconds. You may use negative values.' \
'restart:Restart playback of the current file (equivalent to '\''seek -m absolute 0'\'')' \
'kill:Kill the mpv process controlling the socket' \
'add:Load the given file or playlist and play it' \
'load:Load the given file or playlist and play it' \
'playlist:Print playlist entries' \
'stop:Stop playback and clear the playlist' \
'clear:Clear the playlist, except the currently playing file' \
'remove:Remove the given entry from the playlist (0-indexed). If the entry is currently playing, playback will stop.' \
'rm:Remove the given entry from the playlist (0-indexed). If the entry is currently playing, playback will stop.' \
'move:Move the given playlist entry to a new position' \
'mv:Move the given playlist entry to a new position' \
'play-next:Move the given playlist entry to be after the currently playing file' \
'position:Play the given entry in the playlist' \
'pos:Play the given entry in the playlist' \
'shuffle:Shuffle the playlist' \
'reverse:Reverse the playlist' \
'rev:Reverse the playlist' \
'loop-file:Control whether the current file should be repeated after playback' \
'loop-playlist:Control whether the playlist should be repeated after the end is reached' \
'volume:Control the volume level' \
'mute:Control whether audio output is muted' \
'set:Set a property to the given value' \
'get:Retrieve a property (see property '\''property-list'\'' for possible values)' \
'run:Run an mpv command' \
'metadata:Retrieve a metadata attribute from the currently playing file (see property '\''metadata'\'' for possible values)' \
'format:Replace the given specifiers in the format string with their real-time values from mpv' \
'observe:Print all mpv events in real-time. Additionally, observe a set of properties and inform about changes.' \
'wait:Block until one of the given events is triggered, or until one of the given properties is changed' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'mpvc commands' commands "$@"
}
(( $+functions[_mpvc__add_commands] )) ||
_mpvc__add_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc add commands' commands "$@"
}
(( $+functions[_mpvc__help__add_commands] )) ||
_mpvc__help__add_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help add commands' commands "$@"
}
(( $+functions[_mpvc__clear_commands] )) ||
_mpvc__clear_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc clear commands' commands "$@"
}
(( $+functions[_mpvc__help__clear_commands] )) ||
_mpvc__help__clear_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help clear commands' commands "$@"
}
(( $+functions[_mpvc__format_commands] )) ||
_mpvc__format_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc format commands' commands "$@"
}
(( $+functions[_mpvc__help__format_commands] )) ||
_mpvc__help__format_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help format commands' commands "$@"
}
(( $+functions[_mpvc__get_commands] )) ||
_mpvc__get_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc get commands' commands "$@"
}
(( $+functions[_mpvc__help__get_commands] )) ||
_mpvc__help__get_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help get commands' commands "$@"
}
(( $+functions[_mpvc__help_commands] )) ||
_mpvc__help_commands() {
    local commands; commands=(
'play:Resume playback' \
'pause:Pause playback' \
'toggle:Toggle between play and pause' \
'next:Start playing the next entry in the playlist' \
'prev:Start playing the previous entry in the playlist' \
'seek:Set the playback position. By default seeks by a relative amount of seconds. You may use negative values.' \
'restart:Restart playback of the current file (equivalent to '\''seek -m absolute 0'\'')' \
'kill:Kill the mpv process controlling the socket' \
'add:Load the given file or playlist and play it' \
'playlist:Print playlist entries' \
'stop:Stop playback and clear the playlist' \
'clear:Clear the playlist, except the currently playing file' \
'remove:Remove the given entry from the playlist (0-indexed). If the entry is currently playing, playback will stop.' \
'move:Move the given playlist entry to a new position' \
'play-next:Move the given playlist entry to be after the currently playing file' \
'position:Play the given entry in the playlist' \
'shuffle:Shuffle the playlist' \
'reverse:Reverse the playlist' \
'loop-file:Control whether the current file should be repeated after playback' \
'loop-playlist:Control whether the playlist should be repeated after the end is reached' \
'volume:Control the volume level' \
'mute:Control whether audio output is muted' \
'set:Set a property to the given value' \
'get:Retrieve a property (see property '\''property-list'\'' for possible values)' \
'run:Run an mpv command' \
'metadata:Retrieve a metadata attribute from the currently playing file (see property '\''metadata'\'' for possible values)' \
'format:Replace the given specifiers in the format string with their real-time values from mpv' \
'observe:Print all mpv events in real-time. Additionally, observe a set of properties and inform about changes.' \
'wait:Block until one of the given events is triggered, or until one of the given properties is changed' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'mpvc help commands' commands "$@"
}
(( $+functions[_mpvc__help__help_commands] )) ||
_mpvc__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help help commands' commands "$@"
}
(( $+functions[_mpvc__help__kill_commands] )) ||
_mpvc__help__kill_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help kill commands' commands "$@"
}
(( $+functions[_mpvc__kill_commands] )) ||
_mpvc__kill_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc kill commands' commands "$@"
}
(( $+functions[_mpvc__help__loop-file_commands] )) ||
_mpvc__help__loop-file_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help loop-file commands' commands "$@"
}
(( $+functions[_mpvc__loop-file_commands] )) ||
_mpvc__loop-file_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc loop-file commands' commands "$@"
}
(( $+functions[_mpvc__help__loop-playlist_commands] )) ||
_mpvc__help__loop-playlist_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help loop-playlist commands' commands "$@"
}
(( $+functions[_mpvc__loop-playlist_commands] )) ||
_mpvc__loop-playlist_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc loop-playlist commands' commands "$@"
}
(( $+functions[_mpvc__help__metadata_commands] )) ||
_mpvc__help__metadata_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help metadata commands' commands "$@"
}
(( $+functions[_mpvc__metadata_commands] )) ||
_mpvc__metadata_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc metadata commands' commands "$@"
}
(( $+functions[_mpvc__help__move_commands] )) ||
_mpvc__help__move_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help move commands' commands "$@"
}
(( $+functions[_mpvc__move_commands] )) ||
_mpvc__move_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc move commands' commands "$@"
}
(( $+functions[_mpvc__help__mute_commands] )) ||
_mpvc__help__mute_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help mute commands' commands "$@"
}
(( $+functions[_mpvc__mute_commands] )) ||
_mpvc__mute_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc mute commands' commands "$@"
}
(( $+functions[_mpvc__help__next_commands] )) ||
_mpvc__help__next_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help next commands' commands "$@"
}
(( $+functions[_mpvc__next_commands] )) ||
_mpvc__next_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc next commands' commands "$@"
}
(( $+functions[_mpvc__help__observe_commands] )) ||
_mpvc__help__observe_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help observe commands' commands "$@"
}
(( $+functions[_mpvc__observe_commands] )) ||
_mpvc__observe_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc observe commands' commands "$@"
}
(( $+functions[_mpvc__help__pause_commands] )) ||
_mpvc__help__pause_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help pause commands' commands "$@"
}
(( $+functions[_mpvc__pause_commands] )) ||
_mpvc__pause_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc pause commands' commands "$@"
}
(( $+functions[_mpvc__help__play_commands] )) ||
_mpvc__help__play_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help play commands' commands "$@"
}
(( $+functions[_mpvc__play_commands] )) ||
_mpvc__play_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc play commands' commands "$@"
}
(( $+functions[_mpvc__help__play-next_commands] )) ||
_mpvc__help__play-next_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help play-next commands' commands "$@"
}
(( $+functions[_mpvc__play-next_commands] )) ||
_mpvc__play-next_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc play-next commands' commands "$@"
}
(( $+functions[_mpvc__help__playlist_commands] )) ||
_mpvc__help__playlist_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help playlist commands' commands "$@"
}
(( $+functions[_mpvc__playlist_commands] )) ||
_mpvc__playlist_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc playlist commands' commands "$@"
}
(( $+functions[_mpvc__help__position_commands] )) ||
_mpvc__help__position_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help position commands' commands "$@"
}
(( $+functions[_mpvc__position_commands] )) ||
_mpvc__position_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc position commands' commands "$@"
}
(( $+functions[_mpvc__help__prev_commands] )) ||
_mpvc__help__prev_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help prev commands' commands "$@"
}
(( $+functions[_mpvc__prev_commands] )) ||
_mpvc__prev_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc prev commands' commands "$@"
}
(( $+functions[_mpvc__help__remove_commands] )) ||
_mpvc__help__remove_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help remove commands' commands "$@"
}
(( $+functions[_mpvc__remove_commands] )) ||
_mpvc__remove_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc remove commands' commands "$@"
}
(( $+functions[_mpvc__help__restart_commands] )) ||
_mpvc__help__restart_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help restart commands' commands "$@"
}
(( $+functions[_mpvc__restart_commands] )) ||
_mpvc__restart_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc restart commands' commands "$@"
}
(( $+functions[_mpvc__help__reverse_commands] )) ||
_mpvc__help__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help reverse commands' commands "$@"
}
(( $+functions[_mpvc__reverse_commands] )) ||
_mpvc__reverse_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc reverse commands' commands "$@"
}
(( $+functions[_mpvc__help__run_commands] )) ||
_mpvc__help__run_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help run commands' commands "$@"
}
(( $+functions[_mpvc__run_commands] )) ||
_mpvc__run_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc run commands' commands "$@"
}
(( $+functions[_mpvc__help__seek_commands] )) ||
_mpvc__help__seek_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help seek commands' commands "$@"
}
(( $+functions[_mpvc__seek_commands] )) ||
_mpvc__seek_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc seek commands' commands "$@"
}
(( $+functions[_mpvc__help__set_commands] )) ||
_mpvc__help__set_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help set commands' commands "$@"
}
(( $+functions[_mpvc__set_commands] )) ||
_mpvc__set_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc set commands' commands "$@"
}
(( $+functions[_mpvc__help__shuffle_commands] )) ||
_mpvc__help__shuffle_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help shuffle commands' commands "$@"
}
(( $+functions[_mpvc__shuffle_commands] )) ||
_mpvc__shuffle_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc shuffle commands' commands "$@"
}
(( $+functions[_mpvc__help__stop_commands] )) ||
_mpvc__help__stop_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help stop commands' commands "$@"
}
(( $+functions[_mpvc__stop_commands] )) ||
_mpvc__stop_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc stop commands' commands "$@"
}
(( $+functions[_mpvc__help__toggle_commands] )) ||
_mpvc__help__toggle_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help toggle commands' commands "$@"
}
(( $+functions[_mpvc__toggle_commands] )) ||
_mpvc__toggle_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc toggle commands' commands "$@"
}
(( $+functions[_mpvc__help__volume_commands] )) ||
_mpvc__help__volume_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help volume commands' commands "$@"
}
(( $+functions[_mpvc__volume_commands] )) ||
_mpvc__volume_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc volume commands' commands "$@"
}
(( $+functions[_mpvc__help__wait_commands] )) ||
_mpvc__help__wait_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc help wait commands' commands "$@"
}
(( $+functions[_mpvc__wait_commands] )) ||
_mpvc__wait_commands() {
    local commands; commands=()
    _describe -t commands 'mpvc wait commands' commands "$@"
}

if [ "$funcstack[1]" = "_mpvc" ]; then
    _mpvc "$@"
else
    compdef _mpvc mpvc
fi
