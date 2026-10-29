# Use Powershell instead of sh on windows
set windows-shell := ["powershell.exe","-NoLogo" ,"-Command"]

set default-list := true

port := "4000"
game := "rattler"


# check for warnings in the code
check:
    cargo clippy --all-targets --all-features -- -D warnings

# format the code
fmt:
    cargo fmt --all

# run the game
run:
    cargo run

# build the game for web
build-web:
    cargo build --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/{{game}}.wasm web/{{game}}.wasm

# serve the game on localhost
serve: build-web
    simple-http-server --ip 0.0.0.0 --nocache -i -p {{port}} web