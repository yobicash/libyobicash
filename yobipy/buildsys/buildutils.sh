#!/usr/bin/env bash

# useful utility functions for building stuff and logging it
# meant for inclusion in other scripts, not running directly

IFS=\"
START_TIMESTAMP=`date -Is`
LOGFILE=`realpath ./build.log`
PASSMSG=`mktemp`
FAILMSG=`mktemp`
CURTASK_START=$START_TIMESTAMP
CURTASK=Script

function start_build() {
    echo "Logging to $LOGFILE"
    echo "Build started at $START_TIMESTAMP" > $LOGFILE
    touch $PASSMSG
    touch $FAILMSG
}

function start_deps() {
    mkdir -p $HOME/.yobicash/packages
    DEPCACHE=`realpath $HOME/.yobicash/packages`
    echo "Fetching dependencies into $DEPCACHE:"
}

function fetch_dep() {
    printf "$1:"
    curl -o $DEPCACHE/$3 $2 --progress-bar
}

function finish_deps() {
    echo "Finished fetching dependencies"
}

function finish_build() {
    END_TIMESTAMP=`date -Is`
    echo "Build completed successfully!"
    echo "Build finished at $END_TIMESTAMP" >>$LOGFILE
    echo
    cat $PASSMSG
    rm -f $PASSMSG
    exit 0
}

function abort_build() {
    END_TIMESTAMP=`date -Is`
    echo "Build failed at $END_TIMESTAMP" >>$LOGFILE
    echo
    echo -e '\e[31m\e[1mBuild failed! \e[0m \e[39m'
    echo
    cat $FAILMSG
    printf "See $LOGFILE for more details on build failure\n"
    rm -f $FAILMSG
    exit 1
}

function begin_task() {
    CURTASK=$1
    CURTASK_START=`date -Is`
    echo "$CURTASK started at $CURTASK_START" >>$LOGFILE
    printf "%-40s" "$CURTASK"
}

function complete_task() {
    TASK_END=`date -Is`
    echo "$CURTASK finished at $TASK_END" >>$LOGFILE
    echo -e '\e[1m[\e[92m  OK  \e[39m]\e[0m'
}

function fail_task() {
    TASK_END=`date -Is`
    echo "$CURTASK failed at $TASK_END" >>$LOGFILE
    echo  -e '\e[1m[\e[31m FAIL \e[39m]\e[0m'
}

# below takes 2 params: a string with the task description, and the name of a function to try and run
# the function should return 0 on success, and any other value on failure
# if the function fails, this script will terminate
# e.g
#    try_task "My wonderful task" my_task_func
function try_task() {
    begin_task $1
    printf "Running command: $2\n" >>$LOGFILE
    if $2; then
       complete_task
    else
       fail_task
       abort_build
    fi
}



