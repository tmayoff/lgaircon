#!/bin/bash

PI_IP=10.0.0.237
TARGET=armv7-unknown-linux-gnueabihf

cargo build --target $TARGET

scp ./target/$TARGET/debug/lgaircon lgaircon@$PI_IP:/home/lgaircon