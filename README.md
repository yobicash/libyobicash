libyobicash
===========

[![Build Status](https://travis-ci.org/yobicash/libyobicash.svg?branch=master)](https://travis-ci.org/yobicash/libyobicash)
[![Coverage Status](https://coveralls.io/repos/github/yobicash/libyobicash/badge.svg?branch=master)](https://coveralls.io/github/yobicash/libyobicash?branch=master)

Work in progress

A note on versioning:

libyobicash follows the semantic versioning standard (see semver.org for details), this means essentially when making a release:

 * If only minor bug fixes and nothing else is included with no public-facing API changes: increment the patch number
 * If new API functions are added, old API functions are marked depreciated, or large bits of internal code are improved but it's still backwards compatible: increment minor number
 * If the release contains significant new API functions, changes behaviour of old API functions or deletes old API functions: increment the major number

Each time major is incremented, reset minor and patch to 0, and each time minor is incremented reset patch to 0.

For libyobicash, the relevant version strings are found in Cargo.toml and src/lib.rs - for yobipy (the python bindings), the version number is found in yobipyll/Cargo.toml for the low-level
native module (the glue layer between libyobicash and the pure python code) and is accessible at runtime along with other build metadata.

The pure python code in yobipy/ should generally speaking be updated in line with yobipyll to keep the 2 in sync, the version string may be found in `yobipy/yobipy/__init__.py`


For the whole repo, versioning can get messy and complex, so releases should be handled with the following rules:

 * Each time libyobicash is updated: if still backwards compatible, yobipyll should be left at the current version until it is updated to support the new underlying functionality in libyobicash
 * Once yobipyll is updated to support new functionality, the regular semantic versioning rules should be used to update yobipyll's version number
 * Each time yobipyll is updated: if still backwards compatible, as above yobipy should be left at the current version until it is updated to support the new underlying functionality
 * It is possible that yobipy will continue to support older versions of yobipyll and libyobicash - therefore, unless it removes support for older versions, the major number need not be incremented
 * Each time a new minor release is made of libyobicash, yobipy's SUPPORTED_VERSIONS set should be updated and methods marked as depreciated etc - then regular semver rules applied to update yobipy
 * If yobipy's API changes significantly or it drops support for old versions (i.e SUPPORTED_VERSIONS loses an item), yobipy's major version should be incremented
 * For convenience, yobipy also includes an OLDEST_SUPPORTED constant - when support is dropped, this should be updated (this allows flexibility, we can drop intermediate versions if wanted)

Making a release of libyobicash:
  1. Ensure the code in master branch is up to date, merge in all PRs etc that will form part of the release
  2. Run test suite, verify everything works correctly - including yobipy, and update yobipy if needed
  3. Increment the version numbers as appropriate in src/lib.rs and Cargo.toml, update CHANGELOG
  4. Build binary packages for all platforms in release mode, do basic sanity checks on the output, do not publish yet
  5. Branch off as Maj.Min.Patch-RC
  6. Get it reviewed - open a PR to merge Maj.Min.Patch-RC back into master
  7. If review fails, make required changes, increment patch number (or others if appropriate), branch off again into Maj.Min.Patch-RC
  8. If changes are made to master, ensure they are merged into the latest -RC branch before submitting for PR again - if required, fix up conflicts etc
  9. Repeat 6 and 7 until review passes, ensuring to keep CHANGELOG up to date and keeping in sync with master
 10. Review should only pass when a clean merge to master (no conflicts) is possible - reviewer should confirm this in local clone without pushing
 11. PR merger merges changes from master into Maj.Min branch (or creates it if required) - again, clean merge is needed here, this should be verified in local clone without pushing
 12. PR merger confirms clean merges are possible and uses GitHub website to approve the PR and automatically merge cleanly, obviously test suite etc should also pass
 13. Tag Maj.Min.Patch-RELEASE from the Maj.Min branch
 14. Checkout the new tag, build binary packages for all supported platforms, github release, publish packages etc - if this all works, job done, if not....
 15. This should never happen: if bad code makes it into a -RELEASE tag and thus builds fail, revert commits containing merge to Maj.Min branch (or delete the branch if first release)
 16. After reverting commits, discussion should be had on why build failed and problem fixed in a new branch, PR to master should then be submitted

Making a release of yobipy + yobipyll:
 0. Any changes made solely to yobipy+yobipyll should still cause libyobicash to be incremented in the patch number - this can be noted in CHANGELOG as "yobipy/yobipyll update: bla bla"
 1. Follow process above for release of libyobicash, this ensures everything works correctly, if needed first release a new libyobicash
 2. Use the build_yobipy.sh script to create distribution packages for yobipy
 3. Upload the generated wheel to pypi, update RTD documentation etc

Backporting bugfixes (should only be used for serious bugs, security issues etc):
 1. Ensure relevant fixes are included in latest release first before backporting, creating a new release if needed, even if only Patch version is incremented
 2. Checkout the Maj.Min branch to backport the fix to
 3. Branch off again, create a working banch for this backport named Maj.Min.Patch-backport-briefsummary (for example 0.1.337-backport-badkeys or something)
 4. In the working branch, backport ONLY the bugfix (if it can't be backported without breaking API compatibility, a new minor release will be needed - this is probably a sign it's too old a version to support)
 5. Once the working branch has a solid bugfix (run all normal testing), increment Patch version, updated CHANGELOG and submit PR to merge upstream to the Maj.Min branch
 6. PR is reviewed, if changes are needed, make changes as appropriate
 7. If while working on backport, another issue is backported first, it is up to the PR reviewer to ensure patch version and CHANGELOG history is consistent and doesn't run backwards
 8. Once PR is accepted+merged, tag Maj.Min.Patch, checkout new tag and build binary packages + github release and announce to users - as with above, care must be taken to verify clean merge in local clone

