#!/bin/sh

# Sample RC file which you can use as your starting point

# run Double Commander if not running
if [ -z "$(pidof doublecmd)" ]; then
    doublecmd&
fi

# run Sublime Text if not running
if [ -z "$(pidof kate)" ]; then
    kate&
fi

# run Visual Studio Code if not running
if [ -z "$(pidof code)" ]; then
    code&
fi
