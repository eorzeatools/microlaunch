FROM rust:buster

# Install dependencies.
RUN apt update -y && apt upgrade -y && \
apt install -y libxcb-composite0-dev 

# Download and set up appimagetool
RUN wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage -O /opt/appimagetool && \
cd /opt/; chmod +x appimagetool; sed -i 's|AI\x02|\x00\x00\x00|' appimagetool; ./appimagetool --appimage-extract && \
mv /opt/squashfs-root /opt/appimagetool.AppDir && \
ln -s /opt/appimagetool.AppDir/AppRun /usr/local/bin/appimagetool

# Set up AppDir structure.
WORKDIR /root/AppDir/
RUN mkdir -p usr/bin && \
mkdir -p usr/lib

# Copy build script, set entry point.
WORKDIR /root/
COPY build.sh build.sh
CMD ./build.sh
