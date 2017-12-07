#!/usr/bin/env bash


SOURCE_ROOT_PATH=`realpath ..`
RUST_TGT_PATH=`realpath $SOURCE_ROOT_PATH/target/debug`
NCORES=`nproc --all`

function build_crates {
    eval_with_log "build_crates()" "pushd $SOURCE_ROOT_PATH"
    eval_with_log "build_crates()" "cargo clean"
    eval_with_log "build_crates()" "cargo build --color never -vv -j $NCORES -p libyobicash"
    BUILD_CRATES_RETVAL=$?
    eval_with_log "build_crates()" "popd"
    return $BUILD_CRATES_RETVAL
}

function build_ll_module {
    eval_with_log "build_ll_module()" "pushd $SOURCE_ROOT_PATH"
    eval_with_log "build_ll_module()" "cargo build --color never -vv -j $NCORES -p yobipyll"
    BUILD_LLMOD_RETVAL=$?
    eval_with_log "build_ll_module()" "popd"
    return $BUILD_LLMOD_RETVAL
}

function test_libyobicash {
    eval_with_log "test_libyobicash()" "pushd $SOURCE_ROOT_PATH"
    eval_with_log "test_libyobicash()" "RUST_BACKTRACE=1 cargo test --color never -v"
    TEST_LIBYOBICASH_RETVAL=$?
    eval_with_log "test_libyobicash()" "cargo test --no-run"
    eval_with_log "test_libyobicash()" "mkdir -p htmlcov/libyobicash; kcov htmlcov/libyobicash target/debug/libyobicash*"
    eval_with_log "test_libyobicash()" "mkdir -p htmlcov/libyobicash; kcov htmlcov/libyobicash target/debug/mod*"
    eval_with_log "test_libyobicash()" "popd"
    return $TEST_LIBYOBICASH_RETVAL
}

function test_yobipy {
    eval_with_log "test_yobipy()" "export LD_LIBRARY_PATH=$RUST_TGT_PATH:$LD_LIBRARY_PATH"
    eval_with_log "test_yobipy()" "export PYTHONPATH=$RUST_TGT_PATH:$PYTHONPATH"
    
    PYTEST_TMP=`mktemp`
    eval_with_log "test_yobipy()" "pytest -vv --cov=yobipy --cov-report html --cov-report term --color=yes tests &>>$PYTEST_TMP"
    TEST_YOBIPY_RETVAL=$?

    printf "\n\npytest code coverage for yobipy:\n" >>$PASSMSG
    eval_with_log "test_yobipy()" "grep -A5000 -m1 coverage $PYTEST_TMP | grep -v 'passed in' >>$PASSMSG"
    eval_with_log "test_yobipy()" "grep -A5000 -m1 FAILURE $PYTEST_TMP >>$FAILMSG"

    eval_with_log "test_yobipy()" "rm -f $PYTEST_TMP"
    return $TEST_YOBIPY_RETVAL
}

start_build
try_build_task "Build libyobicash" build_crates
try_build_task "Build low-level extension module" build_ll_module
try_build_task "Run libyobicash test suite" test_libyobicash
try_build_task "Run yobipy pytest suite" test_yobipy
finish_build