# Building an AppImage for microlaunch
- **Follow the steps for cloning submodules in BUILDING.md**
- Ensure docker is installed and set up correctly.
- Put the `sdk` folder of the Steamworks SDK into `appimage-docker/steamworks_sdk/`.
- Run `build-appimage.sh` from `appimage-docker` directory.
- If nothing goes wrong, `microlaunch-x86_64.AppImage` should appear in `appimage-docker/output/`.