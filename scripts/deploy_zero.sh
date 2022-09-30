#!/bin/bash

cross build --target arm-unknown-linux-gnueabihf --release && scp target/arm-unknown-linux-gnueabihf/release/lgaircon lg@lgaircon.local:/home/lg/aircon_cross