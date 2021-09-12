#!/bin/bash

cargo fmt
cross build --target=arm-unknown-linux-gnueabihf
scp target/arm-unknown-linux-gnueabihf/debug/ser grant@sprinkler:/home/grant/
ssh grant@sprinkler sudo $HOME/ser
ssh grant@sprinkler sudo killall ser