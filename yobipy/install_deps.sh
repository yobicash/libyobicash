#!/usr/bin/env bash

source buildutils.sh 

start_deps
fetch_dep "Rust installer" "https://sh.rustup.rs" "rust_installer.sh"
finish_deps


