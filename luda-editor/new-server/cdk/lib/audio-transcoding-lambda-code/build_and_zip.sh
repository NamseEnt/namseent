#!/bin/bash

cargo build --release
mv target/release/audio-transcoding-lambda-code bootstrap
zip -r archive.zip bootstrap ffmpeg
