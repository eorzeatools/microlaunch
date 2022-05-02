#!/bin/bash
SRC=/root/microlaunch
APPDIR=/root/AppDir
cd $SRC
STEAM_SDK_LOCATION=$SRC/appimage-docker/steamworks_sdk/sdk cargo b --release # Build it!
# Copy files required for AppImage.
cd $SRC/appimage-docker/assets/
cp AppRun $APPDIR/
cp microlaunch.desktop $APPDIR/
cp icon.png $APPDIR/
cp $SRC/target/release/microlaunch $APPDIR/usr/bin/
# We have to include libsteam_api.so for microlaunch to work, so we look for it.
FP=`find $SRC/target/release/ -name libsteam_api.so`
cp $FP $APPDIR/usr/lib/
appimagetool $APPDIR
cp microlaunch-x86_64.AppImage $SRC/appimage-docker/output/
