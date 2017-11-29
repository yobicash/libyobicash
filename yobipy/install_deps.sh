#!/usr/bin/env bash

source ./buildsys/buildutils.sh 

reset_log
start_dep_fetch
fetch_dep "Rust installer" "https://sh.rustup.rs" "rust_installer.sh"
finish_dep_fetch

function install_rust_core() {
    chmod +x $DEPCACHE/rust_installer.sh
    $DEPCACHE/rust_installer.sh -vy &>>$LOGFILE
    return $?
}

function install_rust_nightly() {
   source $HOME/.cargo/env
   rustup -v toolchain install nightly &>>$LOGFILE
   return $?
}

start_install
try_install_task "Install rust core" install_rust_core
try_install_task "Install rust nightly" install_rust_nightly
finish_install
