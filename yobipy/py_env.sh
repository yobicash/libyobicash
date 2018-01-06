#!/usr/bin/env bash


export SOURCE_ROOT_PATH=`realpath ..`
export RUST_TGT_PATH=`realpath $SOURCE_ROOT_PATH/target/debug`
export LD_LIBRARY_PATH=$RUST_TGT_PATH:$LD_LIBRARY_PATH
export PYTHONPATH=$RUST_TGT_PATH:$PYTHONPATH

