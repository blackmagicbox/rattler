# Use Powershell instead of sh on windows
set windows-shell := ["powershell.exe","-NoLogo" ,"-Command"]

port := "4000"
game := "rattler"

check:
    cargo fmt --all
    cargo clippy --all-targets --all-features -- -D warnings

run:
    cargo run

build-web:
    cargo build --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/{{game}}.wasm web/{{game}}.wasm

serve: build-web
    simple-http-server --ip 0.0.0.0 --nocache -i -p {{port}} web