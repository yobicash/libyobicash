#!/usr/bin/env bash

source ./buildsys/buildutils.sh 

SOURCE_ROOT_PATH=`realpath ..`
RUST_TGT_PATH=`realpath $SOURCE_ROOT_PATH/target/debug`

function build_crates {
    pushd .. >/dev/null
    cargo build --color never -v -p libyobicash &>>$LOGFILE
    RETVAL=$?
    popd >/dev/null
    return $RETVAL
}

function build_ll_module {
    pushd .. >/dev/null
    cargo build --color never -v -p libyobicash &>>$LOGFILE
    RETVAL=$?
    popd >/dev/null
    return $RETVAL
}

function test_libyobicash {
    pushd .. >/dev/null
    RUST_BACKTRACE=1 cargo test -v &>>$LOGFILE
    RETVAL=$?
    popd >/dev/null
    return $RETVAL
}

function test_yobipy {
    export LD_LIBRARY_PATH=$RUST_TGT_PATH:$LD_LIBRARY_PATH
    export PYTHONPATH=$RUST_TGT_PATH:$PYTHONPATH
    
    PYTEST_TMP=`mktemp`
    pytest -vv --cov=yobipy --cov-report term --color=yes tests &>>$PYTEST_TMP
    RETVAL=$?
    cat $PYTEST_TMP >>$LOGFILE

    echo "Code coverage report for yobipy test suite:" >>$PASSMSG
    grep -A5000 -m1 -e "- coverage:" $PYTEST_TMP >>$PASSMSG

    rm -f $PYTEST_TMP
    return $RETVAL
}


start_build
try_task "Build libyobicash" build_crates
try_task "Build low-level extension module" build_ll_module
try_task "Run libyobicash test suite" test_libyobicash
try_task "Run yobipy pytest suite" test_yobipy
finish_build
