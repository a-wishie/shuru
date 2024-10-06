#compdef shuru

_shuru() {
    local commands
    commands=($(shuru --list-commands))

    local completions_options
    completions_options=(bash zsh fish)

    local -a options
    options=(
        '--completions[The shell to generate completions for]: :($(printf "%s\n" "${completions_options[@]}"))'
        '--list-commands[Show available commands]'
        '-h[Print help]'
        '--help[Print help]'
        '-V[Print version]'
        '--version[Print version]'
        '--update-versions[Update all commands to versions in shuru.toml]'
        '--clear-cache[Clear all cached versions]'
    )

    _arguments -s $options
    _describe -t commands 'shuru commands' commands "$@"
}

compdef _shuru shuru
