# fish completion for trance

complete -c trance -f
complete -c trance -n "__fish_use_subcommand" -a "status" -d "Query daemon and active screensaver status"
complete -c trance -n "__fish_use_subcommand" -a "saver" -d "Select or list installed screensavers"
complete -c trance -n "__fish_use_subcommand" -a "preview" -d "Preview a screensaver fullscreen"
complete -c trance -n "__fish_use_subcommand" -a "stop" -d "Stop current screensaver preview"
complete -c trance -n "__fish_use_subcommand" -a "timeout" -d "Set idle timeout in minutes"
complete -c trance -n "__fish_use_subcommand" -a "enable" -d "Enable idle screensaver activation"
complete -c trance -n "__fish_use_subcommand" -a "disable" -d "Disable idle screensaver activation"
complete -c trance -n "__fish_use_subcommand" -a "fps" -d "Toggle FPS overlay counter"
complete -c trance -n "__fish_use_subcommand" -a "scale" -d "Adjust simulation render scale"
complete -c trance -n "__fish_use_subcommand" -a "doctor" -d "Run system diagnostics and repair"
complete -c trance -n "__fish_use_subcommand" -a "interactive" -d "Launch interactive TUI control panel"
complete -c trance -n "__fish_use_subcommand" -a "clean" -d "Clean stale IPC socket and runtime files"
complete -c trance -n "__fish_use_subcommand" -a "completion" -d "Generate shell completion script"
