# libyobicash python bindings (yobipy)

This directory contains the python bindings for libyobicash, also known as yobipy.

## The easy way

Install dependencies with pip and then run the build_yobipy.sh script.

Current dependencies: pytest-cov and a working rust install.

## Setting up manually

First, build both libyobicash and yobipy_ll:

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

Now you can use the python libs to work with libyobicash from python.
