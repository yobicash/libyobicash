import libyobipyll

# the version of yobipy
YOBIPY_VERSION = '0.1.0'

# the oldest supported version of libyobipyll
OLDEST_SUPPORTED_LL = '0.1.0'

# this is so we can optionally drop support for intermediate versions if required
SUPPORTED_LL_VERSIONS = set(['0.1'])

# the below 2 functions seem redundant, but they're included here for a reason
# specifically, so that documentation can be generated from JUST the python code, with the low-level driver mocked out
def get_ll_version():
    """ Get the version of the low-level native module at runtime
       
        This function returns the version of the low-level native module currently being used. All version numbers
        follow the semantic versioning standard.

        Returns:
           str: The version of the low-level native module currently being used

        Note:
          Not to be confused with get_libyobicash_version() - which returns the version of the rust libyobicash library.
          Also not to be confused with the global constant YOBIPY_VERSION - which contains the version of the yobipy python module.
    """
    return libyobipyll.get_ll_version()


def get_libyobicash_version():
    """ Get the version of libyobicash in use at runtime

        This function returns the version of the rust libyobicash module currently in use by the low-level module.
        If you want to get the version of the libyobicash module the low-level module was compiled against instead, that is not currently
        supported.

        Returns:
           str: The libyobicash version currently in use by the low-level module

        Note:
          Not to be confused with get_ll_version() - which returns the version of the low-level yobipy native module.
          Also not to be confused with the global constant YOBIPY_VERSION - which contains the version of the yobipy python module.
    """
    return libyobipyll.get_libyobicash_version()
