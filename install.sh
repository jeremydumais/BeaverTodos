#!/bin/sh

main() {
    #Ensure we are in the same directory as the install script 
    BASEDIR=$(dirname $0)
    cd $BASEDIR

    local _installer_version="1.0.0"
    local _version="0.2"
    need_cmd mkdir
    need_cmd mktemp
    need_cmd cp
    need_cmd curl
    
    #Check if beaver is already installed
    local _dir 
    #_dir = "$(ensure mktemp -d)"
    #ignore rmdir "$_dir"


    local _need_to_build=false
    local _need_to_download=false

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

    local _executable_path
    if $_need_to_build; then
        build
        _executable_path=$RETVAL
        echo $_executable_path
    fi

    install $_executable_path
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

build() {
    need_cmd cargo
    need_cmd which
    echo "Building the release of Beaver Todos"
    ensure cargo build --release
    local _executable_path=$(which target/release/beaver)
    RETVAL=$_executable_path
}

install() {
    local _local_bin_folder="$HOME/.local/bin"
    local _path_variable_updated=false
    #Check if the $HOME/.local/bin folder exist
    if [ ! -d $_local_bin_folder ] 
    then
        echo "Folder $_local_bin_folder not exist. Creating the folder..."
        ensure mkdir -p $_local_bin_folder
        echo "Done"
    fi
    echo "Copy the beaver executable file in $_local_bin_folder"
    cp $1 $_local_bin_folder
    ensure chmod ugo+x $_local_bin_folder/beaver
    #Check if $HOME/.local/bin is in the path
    if [[ ":$PATH:" == *":$_local_bin_folder:"* ]]; then
        echo "Your path is correctly set"
    else
        echo "Adding $_local_bin_folder to the PATH variable..."
        ensure source echo "export PATH=$PATH:/home/jed/.local/bin"
        echo done
        _path_variable_updated=true
    fi
    echo ""
    echo "Installation completed!"
    echo ""
    if $_path_variable_updated 
    then
        echo "To get started you may need to restart your current shell.
This would reload its PATH environment variable to include
Beaver's bin directory ($_local_bin_folder)."
        echo ""
    else
        echo "You can now use the beaver command."
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
