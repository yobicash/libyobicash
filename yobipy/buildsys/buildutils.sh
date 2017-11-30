#!/usr/bin/env bash

# useful utility functions for building stuff and logging it
# meant for inclusion in other scripts, not running directly

IFS=\"
BUILD_START=`date -Is`
LOGFILE=`realpath ./build.log`
CURTASK_START=$BUILD_START
CURTASK=Script
DEPS_START=$BUILD_START
CURDEP_START=$BUILD_START
CURDEP=None
CURDEP_URL=None
CURDEP_FILENAME=`mktemp`

YOBICASH_ROOT=$HOME/.yobicash          # where all yobicash stuff lives
BUILDSYS_ROOT=$YOBICASH_ROOT/buildsys   # build-system specific stuff
DEPCACHE=$BUILDSYS_ROOT/depcache        # cache of downloaded files
DEPBUILD=$BUILDSYS_ROOT/depbuild        # where to unpack tarballs etc (i.e where we unpack for ./configure && make)

YOBIPKGPREFIX=$YOBICASH_ROOT/packages/installed # where to install packages that have no other home

echo "BuildSys - simple tools for installing yobicash and yobipy"
echo "Cache location: $DEPCACHE"
echo "Build location: $DEPBUILD"
echo
echo "If build fails, try clearing the contents of $DEPBUILD first, if it still fails, try doing 'rm -rf $BUILDSYS_ROOT'"
echo "Note that if you are running buildutils.sh directly, nothing will happen: run install_deps.sh and then build_yobipy.sh"

function reset_log() {
    CUR_TIMESTAMP=`date -Is`
    echo
    echo "Resetting log: $LOGFILE"
    echo
    echo "[$CUR_TIMESTAMP] Reset logfile" >$LOGFILE
}

function log_entry() {
    LOGCONTEXT=$1
    LOGTEXT=$2
    CURTIME=`date -Is`
    echo "[$CURTIME] $LOGCONTEXT - $LOGTEXT" >>$LOGFILE
}

# unpacks a tarball or plain old .tar file
# assumes badly behaved tarballs (i.e no prefix directory)
# also does not overwrite existing files
function untar_dep() {
    TARBALL_NAME=$1
    TARBALL_BASENAME=`echo $TARBALL_NAME | sed 's/.tar.*//'`
    UNPACK_TARDIR="$DEPBUILD/$TARBALL_BASENAME"
    eval_with_log "untar_dep()" "mkdir -p $UNPACK_TARDIR"
    log_entry "untar_dep()" "Unpacking tarball $TARBALL_NAME into $UNPACK_TARDIR"
    eval_with_log "untar_dep()" "pushd $UNPACK_TARDIR; tar kxvf $DEPCACHE/$TARBALL_NAME; popd"
}

# warning - black magic below
# do not call _prefix_time directly
function _prefix_time() {
    LOGCONTEXT=$1
    while IFS= read -r line; do
       log_entry "$LOGCONTEXT" ">>> $line"
    done
    if [ ! -z "$line" ]; then
       log_entry "$LOGCONTEXT" ">>> $line"
    fi
}

function eval_with_log() {
    LOGCONTEXT=$1
    CMD_TO_RUN=$2
    
    TEMP_DIR=$(mktemp -d --tmpdir buildsys.output.XXXXXXXXX)
    CMD_OUT="$TEMP_DIR/out"

    mkfifo $CMD_OUT
    
    _prefix_time $LOGCONTEXT < $CMD_OUT &

    log_entry "$LOGCONTEXT" "Executing command: $CMD_TO_RUN"

    eval $CMD_TO_RUN &> $CMD_OUT ; EVAL_RETVAL=$?
    rm -f $CMD_OUT
    wait

    log_entry "$LOGCONTEXT" "Command finished executing with exitcode $EVAL_RETVAL"
    rm -rf $TEMP_DIR

    return $EVAL_RETVAL
}

function count_words() {
    echo $#;
}

function get_apt_deps() {
    if [ $UID -eq 0 ]; then
       log_entry "get_apt_deps()" "Installing dependencies via apt-get"
       eval_with_log "get_apt_deps()" "apt-get update && apt-get install -y $*"
    else
       for pkg_name in $*
       do
	  CHECK_PKGS="$pkg_name $CHECK_PKGS"
       done
       log_entry "get_apt_deps()" "Not running as root, manually checking for packages to install out of $CHECK_PKGS"
       for pkg_name in $*
       do
           dpkg -s $pkg_name &>/dev/null
	   if [ ! $? -eq 0 ]; then
              MISSING_PACKAGES="$pkg_name $MISSING_PACKAGES"
	      log_entry "get_apt_geps()" "Package $pkg_name not installed, will prompt user to install"
	   fi
       done
       MISSING_COUNT=`count_words $MISSING_PACKAGES`
       if [ ! $MISSING_COUNT -eq 0 ]; then
	       printf "BuildSys needs these packages installed:\n"
	       echo
	       echo $MISSING_PACKAGES | xargs -n 1 printf "\t %s\n" 
	       echo
	       echo "Please open another terminal, and use the following commands:"
	       echo
	       printf "\t sudo apt-get update\n"
	       printf "\t sudo apt-get install $MISSING_PACKAGES\n"
	       echo
	       echo "If these packages are already installed, or once you have finished installing them, hit enter to continue"
               echo "Please note that BuildSys will not recheck, so please ensure the packages listed are installed before you hit enter"
	       read
       fi
    fi
}

function start_build() {
    echo "Building..."
    log_entry "start_build()" "Build starting"
    PASSMSG=`mktemp`
    log_entry "start_build()" "Created temporary file for build success message: $PASSMSG"
    FAILMSG=`mktemp`
    log_entry "start_build()" "Created temporary file for build failure message: $FAILMSG"
    eval_with_log "start_build()" "export PATH=$YOBIPKGPREFIX/installed:$PATH"
    log_entry "start_build()" "Set path to $PATH"
}

function start_install() {
    echo "Installing..."
    eval_with_log "start_install()" "mkdir -p $YOBIPKGPREFIX/installed/bin"
    eval_with_log "start_install()" "export PATH=$YOBIPKGPREFIX/installed/bin:$PATH"
    log_entry "start_install()" "Set path to $PATH"
    log_entry "start_install()" "Installation started"
}

function finish_install() {
    echo "Installation completed successfully!"
    log_entry "finish_install()" "Installation finished"
}

function abort_install() {
    log_entry "abort_install()" "Aborted installation"

    echo
    echo -e '\e[31m\e[1mFailed to complete installation! \e[0m \e[39m'
    echo
    echo "See $LOGFILE for more details on failure\n"

    log_entry "abort_install()" "Terminating script due to error"
    exit 1
}

function start_dep_fetch() {
    echo "Fetching dependencies..."
    log_entry "start_dep_fetch()" "Beginning dependency fetch, creating dependency cache directory"
    eval_with_log "start_dep_fetch()" "mkdir -p $HOME/.yobicash/packages"
    DEPCACHE=`realpath $HOME/.yobicash/packages`
    log_entry "start_dep_fetch()" "Fetching dependencies into $DEPCACHE"
}

function abort_dep_fetch() {
    log_entry "abort_dep_fetch()" "Aborted dependency fetch"

    echo
    echo -e '\e[31m\e[1mFailed to download dependencies! \e[0m \e[39m'
    echo
    echo "See $LOGFILE for more details on failure\n"

    log_entry "abort_dep_fetch()" "Terminating script due to error"
    exit 1
}

function fetch_dep() {
    CURDEP=$1
    CURDEP_URL=$2
    CURDEP_FILENAME=$3
    CURDEP_START=`date -Is`

    log_entry "fetch_dep()" "Downloading $CURDEP_FILENAME from $CURDEP_URL"
    printf " * %-40s" "Fetching $CURDEP"

    CURLCMD="curl -o $DEPCACHE/$CURDEP_FILENAME -z $DEPCACHE/$CURDEP_FILENAME $CURDEP_URL"

    eval_with_log "fetch_dep()" "$CURLCMD"

    if [ $? -eq 0 ]; then
       END_TIMESTAMP=`date -Is`
       echo -e '\e[1m[\e[92m  OK  \e[39m]\e[0m'
       log_entry "fetch_dep()" "Finished downloading \"$CURDEP\""
    else
       echo  -e '\e[1m[\e[31m FAIL \e[39m]\e[0m'
       abort_dep_fetch
    fi
}

function finish_dep_fetch() {
    END_TIMESTAMP=`date -Is`
    echo "Finished fetching dependencies"
    echo
    log_entry "finish_dep_fetch()" "Finished fetching dependencies"
}

function finish_build() {
    END_TIMESTAMP=`date -Is`
    echo "Build completed successfully!"
    log_entry "finish_build()" "Build finished"
    cat $PASSMSG
    eval_with_log "finish_build():passmsg" "cat $PASSMSG"
    eval_with_log "finish_build()" "rm -f $PASSMSG $FAILMSG"
}

function abort_build() {
    END_TIMESTAMP=`date -Is`
    log_entry "abort_build()" "Build failed!"
    echo
    echo -e '\e[31m\e[1mBuild failed! \e[0m \e[39m'
    echo
    cat $FAILMSG
    printf "See $LOGFILE for more details on build failure\n"

    eval_with_log "abort_build():failmsg" "cat $FAILMSG"

    log_entry "abort_build():passmsg" "NOTE: Pass message may be incomplete or empty due to build failure"
    eval_with_log "abort_build():passmsg" "cat $PASSMSG"

    eval_with_log "abort_build()" "rm -f $PASSMG $FAILMSG"
    eval_with_log "abort_build()" "Terminating process due to build failure"
    exit 1
}

function begin_build_task() {
    CURTASK=$1
    CURTASK_START=`date -Is`
    log_entry "begin_build_task()" "Build task \"$CURTASK\" started"
    printf " * %-40s" "$CURTASK"
}

function complete_build_task() {
    log_entry "complete_build_task()" "Build task \"$CURTASK\" completed"
    echo -e '\e[1m[\e[92m  OK  \e[39m]\e[0m'
}

function fail_build_task() {
    log_entry "fail_build_task()" "Build task \"$CURTASK\" failed!"
    echo  -e '\e[1m[\e[31m FAIL \e[39m]\e[0m'
}

# below takes 2 params: a string with the task description, and the name of a function to try and run
# the function should return 0 on success, and any other value on failure
# if the function fails, this script will terminate
# e.g
#    try_task "My wonderful task" my_task_func
function try_build_task() {
    begin_build_task $1
    printf "[%s] Running command: %s\n" "`date -Is`" "$2" >>$LOGFILE
    "$2" ; TASK_RETVAL=$?
    log_entry "try_build_task()" "RETVAL=$TASK_RETVAL"
    if [[ $TASK_RETVAL -eq 0 ]]; then
       complete_build_task
    else
       fail_build_task
       abort_build
    fi
}



function begin_install_task() {
    CURTASK=$1
    CURTASK_START=`date -Is`
    log_entry "begin_install_task()" "Install task \"$CURTASK\" started"
    printf " * %-40s" "$CURTASK"
}

function complete_install_task() {
    log_entry "complete_install_task()" "Install task \"$CURTASK\" completed"
    echo -e '\e[1m[\e[92m  OK  \e[39m]\e[0m'
}

function fail_install_task() {
    log_entry "fail_install_task()" "Build task \"$CURTASK\" failed!"
    echo  -e '\e[1m[\e[31m FAIL \e[39m]\e[0m'
}

function try_install_task() {
    begin_install_task $1
    printf "[%s] Running command: %s\n" "`date -Is`" "$2" >>$LOGFILE
    "$2" ; TASK_RETVAL=$?
    log_entry "try_install_task()" "RETVAL=$TASK_RETVAL"
    if [[ $TASK_RETVAL -eq 0 ]]; then
       complete_install_task
    else
       fail_install_task
       abort_install
    fi
}
