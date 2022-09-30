target := "arm-unknown-linux-gnueabihf"

build:
    cross build --target {{target}}

deploy: build
    scp target/arm-unknown-linux-gnueabihf/release/lgaircon lgaircon@aircon.local:/home/lgaircon/lgaircon/aircon_cross