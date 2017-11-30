# libyobicash python bindings (yobipy)

This directory contains the python bindings for libyobicash, also known as yobipy.

## Installing - the easy way

 1 - Install the dependencies using install_deps.sh, this will install rust, python and all other required tools+deps into your home directory.
 2 - Build everything using build_yobipy.sh - this script will also run the test suites for all components, upon error check build.log.
 3 - A python package will be built and a virtualenv configured for you, to use the virtualenv simply type "pipenv shell" and use python 3.6 to run your application code.

In summary:
```
./install_deps.sh
./build_yobipy.sh
pipenv shell
python3.6 /path/to/your/app/code.py
```

## Manual installation

First, build both libyobicash and yobipy_ll by running cargo in the root of this repo:

```
 cargo build --all
```

This will compile the libyobicash library and the low level python binding module in target/debug/ by default.
Add this directory to your LD_LIBRARY_PATH and your PYTHONPATH environment variables:

```
 export LD_LIBRARY_PATH=/path/to/code/libyobicash/target/debug/
 export PYTHONPATH=/path/to/code/libyobicash/target/debug/
```

If it worked, you should be able to import the low-level module without errors:
```
Python 3.6.1 (default, Aug 22 2017, 23:45:05) 
[GCC 6.3.0 20170516] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import libyobipy_ll
>>> 
```

You can then either use pip+pipenv to setup the python module in a virtualenv, or add your checkout of yobipy to your PYTHONPATH and install the
python modules listed in Pipfile manually.
