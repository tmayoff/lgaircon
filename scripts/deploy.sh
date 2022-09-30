#!/bin/bash

cross build

scp target/armv7-unknown-linux-gnueabihf/debug/lgaircon lgaircon@lgaircon.local:/home/lgaircon/lgaircon/aircon_cross