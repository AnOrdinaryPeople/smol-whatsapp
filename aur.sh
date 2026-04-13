#!/bin/bash

cargo tauri build
mv src-tauri/target/release/bundle/deb/*.deb smol-whatsapp.deb
makepkg --nodeps --nocheck --skipchecksums -f
rm -rf pkg src smol-whatsapp.deb
