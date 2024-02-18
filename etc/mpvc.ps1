using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'mpvc' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'mpvc'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'mpvc' {
            [CompletionResult]::new('-S', 'S ', [CompletionResultType]::ParameterName, 'S')
            [CompletionResult]::new('--socket', 'socket', [CompletionResultType]::ParameterName, 'socket')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('play', 'play', [CompletionResultType]::ParameterValue, 'Resume playback')
            [CompletionResult]::new('pause', 'pause', [CompletionResultType]::ParameterValue, 'Pause playback')
            [CompletionResult]::new('toggle', 'toggle', [CompletionResultType]::ParameterValue, 'Toggle between play and pause')
            [CompletionResult]::new('next', 'next', [CompletionResultType]::ParameterValue, 'Start playing the next entry in the playlist')
            [CompletionResult]::new('prev', 'prev', [CompletionResultType]::ParameterValue, 'Start playing the previous entry in the playlist')
            [CompletionResult]::new('seek', 'seek', [CompletionResultType]::ParameterValue, 'Set the playback position. By default seeks by a relative amount of seconds. You may use negative values.')
            [CompletionResult]::new('restart', 'restart', [CompletionResultType]::ParameterValue, 'Restart playback of the current file (equivalent to ''seek -m absolute 0'')')
            [CompletionResult]::new('kill', 'kill', [CompletionResultType]::ParameterValue, 'Kill the mpv process controlling the socket')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Load the given file or playlist and play it')
            [CompletionResult]::new('playlist', 'playlist', [CompletionResultType]::ParameterValue, 'Print playlist entries')
            [CompletionResult]::new('stop', 'stop', [CompletionResultType]::ParameterValue, 'Stop playback and clear the playlist')
            [CompletionResult]::new('clear', 'clear', [CompletionResultType]::ParameterValue, 'Clear the playlist, except the currently playing file')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove the given entry from the playlist (0-indexed). If the entry is currently playing, playback will stop.')
            [CompletionResult]::new('move', 'move', [CompletionResultType]::ParameterValue, 'Move the given playlist entry to a new position')
            [CompletionResult]::new('play-next', 'play-next', [CompletionResultType]::ParameterValue, 'Move the given playlist entry to be after the currently playing file')
            [CompletionResult]::new('position', 'position', [CompletionResultType]::ParameterValue, 'Play the given entry in the playlist')
            [CompletionResult]::new('shuffle', 'shuffle', [CompletionResultType]::ParameterValue, 'Shuffle the playlist')
            [CompletionResult]::new('reverse', 'reverse', [CompletionResultType]::ParameterValue, 'Reverse the playlist')
            [CompletionResult]::new('loop-file', 'loop-file', [CompletionResultType]::ParameterValue, 'Control whether the current file should be repeated after playback')
            [CompletionResult]::new('loop-playlist', 'loop-playlist', [CompletionResultType]::ParameterValue, 'Control whether the playlist should be repeated after the end is reached')
            [CompletionResult]::new('volume', 'volume', [CompletionResultType]::ParameterValue, 'Control the volume level')
            [CompletionResult]::new('mute', 'mute', [CompletionResultType]::ParameterValue, 'Control whether audio output is muted')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set a property to the given value')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Retrieve a property (see property ''property-list'' for possible values)')
            [CompletionResult]::new('run', 'run', [CompletionResultType]::ParameterValue, 'Run an mpv command')
            [CompletionResult]::new('metadata', 'metadata', [CompletionResultType]::ParameterValue, 'Retrieve a metadata attribute from the currently playing file (see property ''metadata'' for possible values)')
            [CompletionResult]::new('format', 'format', [CompletionResultType]::ParameterValue, 'Replace the given specifiers in the format string with their real-time values from mpv')
            [CompletionResult]::new('observe', 'observe', [CompletionResultType]::ParameterValue, 'Print all mpv events in real-time. Additionally, observe a set of properties and inform about changes.')
            [CompletionResult]::new('wait', 'wait', [CompletionResultType]::ParameterValue, 'Block until one of the given events is triggered, or until one of the given properties is changed')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'mpvc;play' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;pause' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;toggle' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;next' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;prev' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;seek' {
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, '<relative|relative-percent|absolute|absolute-percent> <relative>: Seek relative to the current position (a negative value seeks backwards) <absolute>: Seek to a given position (a negative value seeks starting from the end of the file) <absolute-percent>: Seek to a given position in percentage <relative-percent>: Seek relative to the current position in percentage ')
            [CompletionResult]::new('--mode', 'mode', [CompletionResultType]::ParameterName, '<relative|relative-percent|absolute|absolute-percent> <relative>: Seek relative to the current position (a negative value seeks backwards) <absolute>: Seek to a given position (a negative value seeks starting from the end of the file) <absolute-percent>: Seek to a given position in percentage <relative-percent>: Seek relative to the current position in percentage ')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;restart' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;kill' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;add' {
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, '<replace|append|append-play|append-next> <replace>: Stop playback of the current file, and play the new file immediately <append>: Append the file to the playlist <append-play>: Append the file, and if nothing is currently playing, start playback <append-next>: Append the file to the playlist, and place it in the next position ')
            [CompletionResult]::new('--mode', 'mode', [CompletionResultType]::ParameterName, '<replace|append|append-play|append-next> <replace>: Stop playback of the current file, and play the new file immediately <append>: Append the file to the playlist <append-play>: Append the file, and if nothing is currently playing, start playback <append-next>: Append the file to the playlist, and place it in the next position ')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 't')
            [CompletionResult]::new('--type', 'type', [CompletionResultType]::ParameterName, 'type')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;playlist' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;stop' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;clear' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;remove' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;move' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;play-next' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;position' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;shuffle' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;reverse' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;loop-file' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;loop-playlist' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;volume' {
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, '<absolute|relative> <absolute>: Set the volume <relative>: Change the volume relative to the current level (a negative value decreases the level) ')
            [CompletionResult]::new('--mode', 'mode', [CompletionResultType]::ParameterName, '<absolute|relative> <absolute>: Set the volume <relative>: Change the volume relative to the current level (a negative value decreases the level) ')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;mute' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;set' {
            [CompletionResult]::new('-j', 'j', [CompletionResultType]::ParameterName, 'Parse <value> as JSON')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'Parse <value> as JSON')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;get' {
            [CompletionResult]::new('-j', 'j', [CompletionResultType]::ParameterName, 'Print property as JSON')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'Print property as JSON')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;run' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;metadata' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;format' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;observe' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;wait' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'mpvc;help' {
            [CompletionResult]::new('play', 'play', [CompletionResultType]::ParameterValue, 'Resume playback')
            [CompletionResult]::new('pause', 'pause', [CompletionResultType]::ParameterValue, 'Pause playback')
            [CompletionResult]::new('toggle', 'toggle', [CompletionResultType]::ParameterValue, 'Toggle between play and pause')
            [CompletionResult]::new('next', 'next', [CompletionResultType]::ParameterValue, 'Start playing the next entry in the playlist')
            [CompletionResult]::new('prev', 'prev', [CompletionResultType]::ParameterValue, 'Start playing the previous entry in the playlist')
            [CompletionResult]::new('seek', 'seek', [CompletionResultType]::ParameterValue, 'Set the playback position. By default seeks by a relative amount of seconds. You may use negative values.')
            [CompletionResult]::new('restart', 'restart', [CompletionResultType]::ParameterValue, 'Restart playback of the current file (equivalent to ''seek -m absolute 0'')')
            [CompletionResult]::new('kill', 'kill', [CompletionResultType]::ParameterValue, 'Kill the mpv process controlling the socket')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Load the given file or playlist and play it')
            [CompletionResult]::new('playlist', 'playlist', [CompletionResultType]::ParameterValue, 'Print playlist entries')
            [CompletionResult]::new('stop', 'stop', [CompletionResultType]::ParameterValue, 'Stop playback and clear the playlist')
            [CompletionResult]::new('clear', 'clear', [CompletionResultType]::ParameterValue, 'Clear the playlist, except the currently playing file')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove the given entry from the playlist (0-indexed). If the entry is currently playing, playback will stop.')
            [CompletionResult]::new('move', 'move', [CompletionResultType]::ParameterValue, 'Move the given playlist entry to a new position')
            [CompletionResult]::new('play-next', 'play-next', [CompletionResultType]::ParameterValue, 'Move the given playlist entry to be after the currently playing file')
            [CompletionResult]::new('position', 'position', [CompletionResultType]::ParameterValue, 'Play the given entry in the playlist')
            [CompletionResult]::new('shuffle', 'shuffle', [CompletionResultType]::ParameterValue, 'Shuffle the playlist')
            [CompletionResult]::new('reverse', 'reverse', [CompletionResultType]::ParameterValue, 'Reverse the playlist')
            [CompletionResult]::new('loop-file', 'loop-file', [CompletionResultType]::ParameterValue, 'Control whether the current file should be repeated after playback')
            [CompletionResult]::new('loop-playlist', 'loop-playlist', [CompletionResultType]::ParameterValue, 'Control whether the playlist should be repeated after the end is reached')
            [CompletionResult]::new('volume', 'volume', [CompletionResultType]::ParameterValue, 'Control the volume level')
            [CompletionResult]::new('mute', 'mute', [CompletionResultType]::ParameterValue, 'Control whether audio output is muted')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set a property to the given value')
            [CompletionResult]::new('get', 'get', [CompletionResultType]::ParameterValue, 'Retrieve a property (see property ''property-list'' for possible values)')
            [CompletionResult]::new('run', 'run', [CompletionResultType]::ParameterValue, 'Run an mpv command')
            [CompletionResult]::new('metadata', 'metadata', [CompletionResultType]::ParameterValue, 'Retrieve a metadata attribute from the currently playing file (see property ''metadata'' for possible values)')
            [CompletionResult]::new('format', 'format', [CompletionResultType]::ParameterValue, 'Replace the given specifiers in the format string with their real-time values from mpv')
            [CompletionResult]::new('observe', 'observe', [CompletionResultType]::ParameterValue, 'Print all mpv events in real-time. Additionally, observe a set of properties and inform about changes.')
            [CompletionResult]::new('wait', 'wait', [CompletionResultType]::ParameterValue, 'Block until one of the given events is triggered, or until one of the given properties is changed')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'mpvc;help;play' {
            break
        }
        'mpvc;help;pause' {
            break
        }
        'mpvc;help;toggle' {
            break
        }
        'mpvc;help;next' {
            break
        }
        'mpvc;help;prev' {
            break
        }
        'mpvc;help;seek' {
            break
        }
        'mpvc;help;restart' {
            break
        }
        'mpvc;help;kill' {
            break
        }
        'mpvc;help;add' {
            break
        }
        'mpvc;help;playlist' {
            break
        }
        'mpvc;help;stop' {
            break
        }
        'mpvc;help;clear' {
            break
        }
        'mpvc;help;remove' {
            break
        }
        'mpvc;help;move' {
            break
        }
        'mpvc;help;play-next' {
            break
        }
        'mpvc;help;position' {
            break
        }
        'mpvc;help;shuffle' {
            break
        }
        'mpvc;help;reverse' {
            break
        }
        'mpvc;help;loop-file' {
            break
        }
        'mpvc;help;loop-playlist' {
            break
        }
        'mpvc;help;volume' {
            break
        }
        'mpvc;help;mute' {
            break
        }
        'mpvc;help;set' {
            break
        }
        'mpvc;help;get' {
            break
        }
        'mpvc;help;run' {
            break
        }
        'mpvc;help;metadata' {
            break
        }
        'mpvc;help;format' {
            break
        }
        'mpvc;help;observe' {
            break
        }
        'mpvc;help;wait' {
            break
        }
        'mpvc;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
