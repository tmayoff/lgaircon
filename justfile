target := "arm-unknown-linux-gnueabihf"

build:
    cross build --release --target {{target}}

test:
    cross test --target {{target}}

deploy: build
    scp target/arm-unknown-linux-gnueabihf/release/lgaircon lg@lgaircon.local:/home/lg/lgaircon
    scp res/lgaircon.service lg@lgaircon.local:/home/lg/lgaircon.service