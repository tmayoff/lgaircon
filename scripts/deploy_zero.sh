#!/bin/bash

cross build --target arm-unknown-linux-gnueabihf --release && scp target/arm-unknown-linux-gnueabihf/release/lgaircon lgaircon@aircon.local:/home/lgaircon/lgaircon/aircon_cross