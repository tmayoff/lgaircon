#!/bin/bash

cross build

scp target/armv7-unknown-linux-gnueabihf/debug/lgaircon lgaircon@aircon.local:/home/lgaircon/lgaircon/aircon_cross