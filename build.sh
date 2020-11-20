#!/usr/bin/env sh
chmod +x ./appimagetool-x86_64.AppImage
ARCH=x86_64 ./appimagetool-x86_64.AppImage src/appimage/src/setcolors.AppDir src/appimage/target/setcolors-x86_64.AppImage
