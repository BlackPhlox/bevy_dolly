#!/bin/sh

# Convert git submodule to a folder under the current repo
git rm --cached dolly # Delete reference to submodule HEAD (no trailing slash)
git rm .gitmodules # Remove submodule ref,
rm -rf dolly/.git # Remove submodule git metadata
git add dolly # Add files instead of commit reference

# Don't run command if script is executed from GH Actions 
if [ -z "${CI}" ]; then
    read -p "Press any key to resume ..."
fi