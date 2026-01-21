#!/bin/sh

# Sample RC file which you can use as your starting point
# It will run apps over and over again, but you can pass your own code here

# run Double Commander if not running
if [ -z "$(pidof doublecmd)" ]; then
    doublecmd > /dev/null 2>&1 &
fi

# run Sublime Text if not running
if [ -z "$(pidof subl)" ]; then
    subl > /dev/null 2>&1 &
fi

# run Visual Studio Code if not running
if [ -z "$(pidof code)" ]; then
    code > /dev/null 2>&1 &
fi


