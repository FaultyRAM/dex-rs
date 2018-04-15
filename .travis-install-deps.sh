#!/bin/sh
if [ "$RUSTFMT" = "true" ]; then
    rustup -v component add rustfmt-preview
else
    if [ "$TRAVIS_OS_NAME" = "linux" ]; then
        sudo add-apt-repository "deb http://mirrors.kernel.org/ubuntu bionic main universe"
        sudo apt-get -qq update
        sudo apt-get install -y \
            libavcodec-dev \
            libavdevice-dev \
            libavfilter-dev \
            libavformat-dev \
            libavresample-dev \
            libavutil-dev \
            libswresample-dev \
            libswscale-dev
    else
        brew update
        brew install ffmpeg
    fi
fi
