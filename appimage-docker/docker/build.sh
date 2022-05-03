#!/bin/bash
SRC=/root/microlaunch
APPDIR=/root/AppDir
# Change cargo target directory so we can delete it from inside the container, but keep it outside the container.
export CARGO_TARGET_DIR=/root/cache/target
export STEAM_SDK_LOCATION=$SRC/appimage-docker/steamworks_sdk/sdk 

build_binary() {
    cd $SRC
    cargo b --release # Build it!
}
cargo_clean() {
    cd $SRC
    cargo clean
}
build_appimage(){
    # Copy files required for AppImage.
    cd $SRC/appimage-docker/assets/
    cp AppRun $APPDIR/
    cp microlaunch.desktop $APPDIR/
    cp icon.png $APPDIR/
    cp $CARGO_TARGET_DIR/release/microlaunch $APPDIR/usr/bin/
    # We have to include libsteam_api.so for microlaunch to work, so we look for it.
    FP=`find $CARGO_TARGET_DIR/release/ -name libsteam_api.so`
    cp $FP $APPDIR/usr/lib/
    cd /root/
    appimagetool $APPDIR
    cp microlaunch-x86_64.AppImage $SRC/appimage-docker/output/
}
build() {
    build_binary
    build_appimage
}

# $PARAMTER is passed in as environment variable from build-appimage.sh
case $PARAMETER in
    clean)  cargo_clean ;;
    *)      build ;;
esac