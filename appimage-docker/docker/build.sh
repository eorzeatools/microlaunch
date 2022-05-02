#!/bin/bash
SRC=/root/microlaunch
APPDIR=/root/AppDir
cd $SRC
STEAM_SDK_LOCATION=$SRC/appimage-docker/steamworks_sdk/sdk cargo b --release
cd $SRC/appimage-docker/assets/
cp AppRun $APPDIR/
cp microlaunch.desktop $APPDIR/
cp icon.png $APPDIR/
cp $SRC/target/release/microlaunch $APPDIR/usr/bin/
FP=`find $SRC/target/release/ -name libsteam_api.so`
cp $FP $APPDIR/usr/lib/
appimagetool $APPDIR
cp microlaunch-x86_64.AppImage $SRC/appimage-docker/output/
