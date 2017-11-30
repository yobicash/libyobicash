#!/usr/bin/env bash
source ./buildsys/buildutils.sh
reset_log
echo
source ./install_deps.sh
echo
source ./build_yobipy.sh
