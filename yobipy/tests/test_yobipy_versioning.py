""" These tests will fail if you try to build using an incompatible yobipyll or libyobicash
"""

import libyobipyll
import yobipy
import semver

def test_sane_yobipyll():
    """ Test the yobipyll version is supported by the python code

    """
    yobipy_oldest     = yobipy.OLDEST_SUPPORTED_LL
    yobipy_ll_version = libyobipyll.get_ll_version()

    parsed_ll_ver     = semver.parse_version_info(yobipy_ll_version)

    majmin = '%d.%d' % (parsed_ll_ver.major, parsed_ll_ver.minor)
    assert semver.compare(yobipy_ll_version,yobipy_oldest) >= 0
    assert majmin in  yobipy.SUPPORTED_LL_VERSIONS 

def test_sane_libyobicash():
    """ Test yobipyll maj.min version is >= libyobicash version (if it isn't, something is messed up)
    """
    ll_version = libyobipyll.get_ll_version()
    lib_version = libyobipyll.get_libyobicash_version()

    assert semver.compare(ll_version,lib_version) >= 0

def test_versions_match():
    """ Test the versions reported by yobipy and libyobipyll match
        They always should, as yobipy is just a wrapper, but some edge cases could cause this to fail
    """
    assert libyobipyll.get_ll_version()          == yobipy.get_ll_version()
    assert libyobipyll.get_libyobicash_version() == yobipy.get_libyobicash_version()
