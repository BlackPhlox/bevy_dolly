#!/bin/sh

# Step 1 :
# Convert git submodule to a folder under the current repo
git rm --cached dolly # Delete reference to submodule HEAD (no trailing slash)
git rm .gitmodules -f # Remove submodule ref,
rm -rf dolly/.git # Remove submodule git metadata
mkdir src/dolly # Create dolly folder in src
mv dolly/src/* src/dolly # Move src files into src/dolly
git add src/dolly # Add files instead of commit reference
rm -rf dolly

# Step 2 :
# Replace 'dolly = { path = "dolly", default-features = false }' with this under dependecies in Cargo.toml:
# bevy_math = "New Version"
# bevy_transform = "New Version"
sed -i 's/dolly = { path = "dolly", default-features = false }/bevy_math = "*"\nbevy_transform = "*"/g' Cargo.toml

# Step 3 :
# Change "pub use dolly;"" in src/lib.rs to "pub mod dolly;"
sed -i 's/pub use dolly;/pub mod dolly;/g' src/lib.rs

# Step 4 : Add file src/dolly/mod.rs:
#pub mod driver;
#pub mod drivers;
#pub mod handedness;
#pub mod prelude;
#pub mod rig;
#pub mod util;
echo -e 'pub mod driver;\npub mod drivers;\npub mod handedness;\npub mod prelude;\npub mod rig;\npub mod util;' > src/dolly/mod.rs

# Step 5 :
# Replace 'crate::{' with 'crate::dolly::{' in the src/dolly folder
find src/dolly -type f -exec sed -i 's/crate::/crate::dolly::/g' {} \;

# Don't run command if script is executed from GH Actions 
if [ -z "${CI}" ]; then
    read -p "Press any key to resume ..."
fi
