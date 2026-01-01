#!/bin/sh

# Sample RC file which you can use as your starting point

_USER=${USER}

# get name of the real user
if [ -n "$SUDO_USER" ]; then
	_USER=${SUDO_USER}
fi

# run Double Commander
if [ -z "$(pidof doublecmd)" ]; then
    sudo -u ${_USER} doublecmd &
fi

# run Kate
if [ -z "$(pidof kate)" ]; then
    sudo -u ${_USER} kate &
fi

# run Visual Studio Code
if [ -z "$(pidof code)" ]; then
    sudo -u ${_USER} code &
fi




