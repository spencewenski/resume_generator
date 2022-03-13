#!/usr/bin/env sh
#
# Setup the directory on first download

# Get the directory that contains this script
SETUP_DIR=""
if [[ $(uname -s) == "Darwin" ]]; then
    SETUP_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
else
    SETUP_DIR="$(dirname $(readlink -f $0))"
fi

ln -sfv $SETUP_DIR/git-hooks/pre-commit .git/hooks/pre-commit

# Make sure cargo components are available
rustup component add rustfmt
