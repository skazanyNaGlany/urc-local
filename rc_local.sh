#!/bin/sh

# Sample RC file which you can use as your starting point

_USER=${USER}

# get name of the real user
if [ -n "$SUDO_USER" ]; then
	_USER=${SUDO_USER}
fi

# run Double Commander
if [ -z "$(pidof doublecmd)" ]; then
    nohup sudo -u ${_USER} doublecmd > /dev/null 2>&1 &
fi

# run Kate
if [ -z "$(pidof kate)" ]; then
    nohup sudo -u ${_USER} kate > /dev/null 2>&1 &
fi

# run Visual Studio Code
if [ -z "$(pidof code)" ]; then
    nohup sudo -u ${_USER} code > /dev/null 2>&1 &
fi




