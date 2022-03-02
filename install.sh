#!/bin/sh

main() {
    #Ensure we are in the same directory as the install script 
    BASEDIR=$(dirname $0)
    cd $BASEDIR

    local _installer_version="1.0.0"
    local _version="0.2.0"
    need_cmd mkdir
    need_cmd cp
    need_cmd chmod
    need_cmd mktemp
    need_cmd rm
    local _dir="$(ensure mktemp -d)"
        
    local _need_to_build=false
    local _need_to_download=false
    local _local_bin_folder="$HOME/.local/bin"
    for arg in "$@"; do
        case "$arg" in
            -h|--help)
                usage
                exit 0
                ;;
            -b|--build)
                _need_to_build=true
                ;;
            -d|--download)
                _need_to_download=true
                ;;
            -v|--version)
                echo $_installer_version
                exit 0
                ;;
            *)
                ;;
        esac
    done

    if $_need_to_build && $_need_to_download; then
        err "You cannot set both build and download."
        exit 1
    fi
    if ! $_need_to_build && ! $_need_to_download; then
        _need_to_download=true
    fi

    local _executable_path
    if $_need_to_build; then
        build
        _executable_path=$RETVAL
    fi

    if $_need_to_download; then
        download $_dir
        _executable_path=$RETVAL
    fi

    install $_executable_path $_local_bin_folder
    ignore rm -r "$_dir"
}

usage() {
    cat 1>&2 <<EOF
beaver-installer $_installer_version
The installer for BeaverTodos

USAGE:
    install.sh [FLAGS]

FLAGS:
    -b, --build             Build from source and install
    -d, --download          Download binaries and install
                            *This is the Default behavior if no flag is specified
    -r, --root              Install in /usr/local/bin instead of
                            ~/.local/bin
    -h, --help              Prints help information
    -V, --version           Prints version information

EOF
}

build() {
    need_cmd cargo
    need_cmd which
    echo "Building the release of Beaver Todos"
    ensure cargo build --release
    local _executable_path=$(which target/release/beaver)
    RETVAL=$_executable_path
}

download() {
    echo "Downloading the latest binaries..."
    need_cmd curl
    local _filename="$1/beaver"
    curl -sSL https://github.com/jeremydumais/BeaverTodos/releases/download/v$version/beaver --output $_filename
    RETVAL=$_filename
}

install() {
    echo "Starting installation..."
    #Check if the _local_bin_folder folder exist
    if [ ! -d $2 ] 
    then
        echo "Folder $2 not exist. Creating the folder..."
        ensure mkdir -p $2
        echo "Done"
    fi
    echo "Copy the beaver executable file in $2"
    cp $1 $2/
    ensure chmod ugo+x $2/beaver
    echo ""
    echo "Installation completed!"
    echo ""

    #Check if $HOME/.local/bin is in the path
    if echo "$PATH" | grep $2 > /dev/null 2>&1; then
        echo "You can now use the beaver command."
        echo ""
    else
        echo "To get started you need $2 in your 'PATH'
environment variable. This has not been done automatically."
        echo ""
    fi
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
