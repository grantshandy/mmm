#!/bin/bash

cross build --target=arm-unknown-linux-gnueabihf --release
scp target/arm-unknown-linux-gnueabihf/release/mmm grant@sprinkler:/home/grant/