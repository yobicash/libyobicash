#!/usr/bin/env bash

source ./buildsys/buildutils.sh 

reset_log
start_dep_fetch
fetch_dep "Rust installer" "https://sh.rustup.rs" "rust_installer.sh"
fetch_dep "Python 3.6.2" "https://www.python.org/ftp/python/3.6.2/Python-3.6.2.tar.xz" "Python-3.6.2.tar.xz"
finish_dep_fetch

function install_rust_core() {
    eval_with_log "install_rust_core()" "chmod +x $DEPCACHE/rust_installer.sh"
    eval_with_log "install_rust_core()" "$DEPCACHE/rust_installer.sh -y"
    return $?
}

function install_rust_nightly() {
   eval_with_log "install_rust_nightly()" "source $HOME/.cargo/env"
   eval_with_log "install_rust_nightly()" "yes | rustup -v update"
   eval_with_log "install_rust_nightly()" "yes | rustup -v toolchain install nightly"
   return $?
}

function configure_python() {
   if [ -d $DEPBUILD/Python-3.6.2/Python-3.6.2 ]; then
      log_entry "configure_python()" "Python tarball appears to already be unpacked, skipping - if errors occur here, clean your buildsys cache and retry"
   else
      untar_dep "Python-3.6.2.tar.xz"
   fi
   eval_with_log "configure_python()" "pushd $DEPBUILD/Python-3.6.2/Python-3.6.2"
   if [ ! -f Makefile ]; then
      eval_with_log "configure_python()" "./configure --prefix=$YOBIPKGPREFIX --enable-optimizations"
      CONFIGURE_RETVAL=$?
   else
      log_entry "configure_python()" "Python 3.6.2 already configured, skipping configuration"
      CONFIGURE_RETVAL=0
   fi
   eval_with_log "configure_python()" "popd"
   return $CONFIGURE_RETVAL
}

function compile_python() {
   if [ ! -f $YOBIPKGPREFIX/bin/python3.6 ]; then
      eval_with_log "compile_python()" "pushd $DEPBUILD/Python-3.6.2/Python-3.6.2"
      NCORES=`nproc --all`
      eval_with_log "compile_python()" "make -j$NCORES altinstall"
      COMPILE_RETVAL=$?
      eval_with_log "compile_python()" "popd"
   else
      log_entry "compile_python()" "Python 3.6.2 already installed, skipping compilation"
      CONFIGURE_RETVAL=$?
   fi
   return $COMPILE_RETVAL
}

function install_pip() {
   eval_with_log "install_pip()" "$YOBIPKGPREFIX/bin/python3.6 -m pip install --upgrade pip pipenv"
}

function install_pip_packages() {
   eval_with_log "install_pip_packages()" "$YOBIPKGPREFIX/bin/python3.6 -m pipenv --python $YOBIPKGPREFIX/bin/python3.6 --three install --system"
}

start_install
try_install_task "Install rust core" install_rust_core
try_install_task "Install rust nightly" install_rust_nightly
try_install_task "Unpack and configure Python 3.6.2" configure_python
try_install_task "Compile and install Python 3.6.2" compile_python
try_install_task "Install latest pip and pipenv" install_pip
try_install_task "Install packages from Pipfile" install_pip_packages
finish_install
