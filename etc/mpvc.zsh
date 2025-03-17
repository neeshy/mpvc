#compdef _mpvc mpvc

_mpvc() {
    local -a commands=(
        play'\:"Resume playback"'
        pause'\:"Pause playback"'
        toggle'\:"Toggle between play and pause"'
        next'\:"Start playing the next entry in the playlist"'
        prev'\:"Start playing the previous entry in the playlist"'
        seek'\:"Set the playback position. By default seeks by a relative amount of seconds. You may use negative values."'
        restart'\:"Restart playback of the current file (equivalent to '\''seek -m absolute 0'\'')"'
        kill'\:"Kill the mpv process controlling the socket"'
        {add,load}'\:"Load the given file or playlist and play it"'
        {playlist,list}'\:"Print playlist entries"'
        stop'\:"Stop playback and clear the playlist"'
        clear'\:"Clear the playlist, except the currently playing file"'
        {remove,rm}'\:"Remove the given entry from the playlist, or the currently playing entry if the argument is omitted (0-indexed). If the entry is currently playing, playback will stop."'
        {move,mv}'\:"Move the given playlist entry to a new position"'
        play-next'\:"Move the given playlist entry to be after the currently playing file"'
        {position,pos}'\:"Play the given entry in the playlist"'
        {shuffle,shuf}'\:"Shuffle the playlist"'
        {reverse,rev}'\:"Reverse the playlist"'
        loop-file'\:"Control whether the current file should be repeated after playback. Toggle by omitting the argument."'
        loop-playlist'\:"Control whether the playlist should be repeated after the end is reached. Toggle by omitting the argument."'
        volume'\:"Control the volume level"'
        mute'\:"Control whether audio output is muted. Toggle by omitting the argument."'
        set'\:"Set a property to the given value"'
        get'\:"Retrieve a property (see property '\''property-list'\'' for possible values)"'
        run'\:"Run an mpv command"'
        metadata'\:"Retrieve a metadata attribute from the currently playing file (see property '\''metadata'\'' for possible values)"'
        format'\:"Replace the given specifiers in the format string with their real-time values from mpv"'
        observe'\:"Print all mpv events in real-time. Additionally, observe a set of properties and inform about changes."'
        wait'\:"Block until one of the given events is triggered, or until one of the given properties is changed"'
        help'\:"Print this message or the help of the given subcommand(s)"'
    )

    # TODO don't sort matches
    local state line
    _arguments -s -S : \
        '(-S --socket)'{-S+,--socket=}'[Path to mpv socket]:/path/to/socket:_files' \
        '(- *)'{-h,--help}'[Print help]' \
        ":mpvc commands:((${commands[*]}))" \
        '*:: :->args'

    case "$state" in args) case "$line[1]" in
        play|pause|toggle|next|prev|restart|kill|playlist|list|stop|clear|shuffle|shuf|reverse|rev)
            _arguments -s -S : '(- *)'{-h,--help}'[Print help]';;
        seek)
            _arguments -s -S : \
                '(-m --mode)'{-m+,--mode=}'[Seeking mode]:mode:((
                    relative\:"Seek relative to the current position (a negative value seeks backwards)"
                    absolute\:"Seek to a given position (a negative value seeks starting from the end of the file)"
                    absolute-percent\:"Seek to a given position in percentage"
                    relative-percent\:"Seek relative to the current position in percentage"
                ))' \
                '(- *)'{-h,--help}'[Print help]' \
                ':target:';;
        add|load)
            _arguments -s -S : \
                '(-m --mode)'{-m+,--mode=}'[Loading mode]:mode:((
                    replace\:"Stop playback of the current file and play the new file immediately"
                    append\:"Append the file to the playlist"
                    append-play\:"Append the file and if nothing is currently playing, start playback"
                    insert-next\:"Insert the file into the playlist directly after the current entry"
                    insert-next-play\:"Insert the file into the next position and if nothing is currently playing, start playback"
                ))' \
                '(-t --type)'{-t+,--type=}'[Entry type]:type:(file playlist)' \
                '(- *)'{-h,--help}'[Print help]' \
                '*::file:_files';;
        remove|rm)
            _arguments -s -S : \
                '(- *)'{-h,--help}'[Print help]' \
                '::id:_mpvc__playlist';;
        move|mv)
            _arguments -s -S : \
                '(- *)'{-h,--help}'[Print help]' \
                ':from:_mpvc__playlist' \
                ':to:_mpvc__playlist';;
        play-next|position|pos)
            _arguments -s -S : \
                '(- *)'{-h,--help}'[Print help]' \
                ':id:_mpvc__playlist';;
        loop-file|loop-playlist|mute)
            _arguments -s -S : \
                '(- *)'{-h,--help}'[Print help]' \
                '::arg:(on off)';;
        volume)
            _arguments -s -S : \
                '(-m --mode)'{-m+,--mode=}'[Volume mode]:mode:((absolute\:"Set the volume" relative\:"Change the volume relative to the current level (a negative value decreases the level)"))' \
                '(- *)'{-h,--help}'[Print help]' \
                ':num:';;
        set)
            _arguments -s -S : \
                '(-j --json)'{-j,--json}'[Parse <value> as JSON]' \
                '(- *)'{-h,--help}'[Print help]' \
                ':property:_mpvc__property' \
                ':value:';;
        get)
            _arguments -s -S : \
                '(-j --json)'{-j,--json}'[Print property as JSON]' \
                '(- *)'{-h,--help}'[Print help]' \
                ':property:_mpvc__property';;
        run)
            _arguments -s -S : \
                '(- *)'{-h,--help}'[Print help]' \
                ':command:_mpvc__command' \
                '*::args:';;
        metadata)
            _arguments -s -S : \
                '(- *)'{-h,--help}'[Print help]' \
                ':attribute:_mpvc__metadata';;
        format)
            _arguments -s -S : \
                '(- *)'{-h,--help}'[Print help]' \
                ':format-string:';;
        observe)
            _arguments -s -S : \
                '(- *)'{-h,--help}'[Print help]' \
                '*::property:_mpvc__property';;
        wait)
            # TODO match on properties after '--'
            _arguments -s -S : \
                '(- *)'{-h,--help}'[Print help]' \
                '*::event:((
                    start-file\:"Happens right before a new file is loaded. When you receive this, the player is loading the file (or possibly already done with it)."
                    end-file\:"Happens after a file was unloaded. Typically, the player will load the next file right away, or quit if this was the last file."
                    file-loaded\:"Happens after a file was loaded and begins playback."
                    seek\:"Happens on seeking. (This might include cases when the player seeks internally, even without user interaction. This includes e.g. segment changes when playing ordered chapters Matroska files.)"
                    playback-restart\:"Start of playback after seek or after file was loaded."
                    shutdown\:"Sent when the player quits, and the script should terminate. Normally handled automatically. See Details on the script initialization and lifecycle."
                    video-reconfig\:"Happens on video output or filter reconfig."
                    audio-reconfig\:"Happens on audio output or filter reconfig."
                ))';;
        help)
            _arguments -s -S : ":mpvc help commands:((${commands[*]}))";;
    esac;; esac
}

_mpvc__describe() {
    whence jq &>/dev/null || return
    mpvc get idle-active &>/dev/null || return
    local name="$1"
    local property="$2"
    local filter="$3"
    shift 3
    local -a completions=("${(@0)$(mpvc get -- "$property" | jq --raw-output0 -- "$filter")}")
    shift -p completions
    _describe -V completions "$name" "$@"
}

_mpvc__playlist() {
    _mpvc__describe playlist playlist 'to_entries | .[] | (.key | tostring) + ":" + .value.filename' "$@"
}

_mpvc__property() {
    _mpvc__describe property property-list '.[]' "$@"
}

_mpvc__command() {
    _mpvc__describe command command-list '.[] | .name' "$@"
}

_mpvc__metadata() {
    _mpvc__describe metadata metadata 'keys | .[]' "$@"
}
