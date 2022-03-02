#!/bin/sh

main() {
    local _installer_version
    _installer_version="1.0.0"
    local _version
    _version="0.2"
    need_cmd mkdir
    need_cmd mktemp
    need_cmd cp
    need_cmd curl
    
    #Check if beaver is already installed
    local _dir 
    #_dir = "$(ensure mktemp -d)"
    #ignore rmdir "$_dir"

    for arg in "$@"; do
        case "$arg" in
            -h|--help)
                usage
                exit 0
                ;;
            -v|--version)
                echo $_installer_version
                exit 0
                ;;
            *)
                ;;
        esac
    done
}

usage() {
    cat 1>&2 <<EOF
beaver-installer $_installer_version
The installer for BeaverTodos

USAGE:
    install.sh [FLAGS] [OPTIONS]

FLAGS:
    -b, --build             Enable verbose output
    -d, --download          Disable progress output
    -h, --help              Prints help information
    -V, --version           Prints version information

OPTIONS:
        --default-host <default-host>              Choose a default host triple
        --default-toolchain <default-toolchain>    Choose a default toolchain to install
        --default-toolchain none                   Do not install any toolchains
        --profile [minimal|default|complete]       Choose a profile
    -c, --component <components>...                Component name to also install
    -t, --target <targets>...                      Target name to also install
EOF
}

need_cmd() {

    if ! check_cmd "$1"; then
        err "need '$1' (command not found)"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
}

# Run a command that should never fail. If the command fails execution
# will immediately terminate with an error showing the failing
# command.
ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

# This is just for indicating that commands' results are being
# intentionally ignored. Usually, because it's being executed
# as part of error handling.
ignore() {
    "$@"
}

say() {
    printf 'beaver_installer: %s\n' "$1"
}

err() {
    say "$1" >&2
    exit 1
}


main "$@" || exit 1
