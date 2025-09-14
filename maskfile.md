# Lince's Rust Template
Hello, world

# Commands
## install
```bash
cargo install bacon mdbook cargo-edit cargo-udeps --locked
```

## update
```bash
rustup self update
rustup update stable
cargo upgrade
mask install
```

## run
```bash
mask install
bacon --job run .
```
> Installs mprocs if not installed and uses it for running all the processes one might need.

## cross
```bash
docker pull ghcr.io/cross-rs/x86_64-unknown-linux-gnu:main --platform linux/x86_64
cross build --target x86_64-unknown-linux-gnu --release
```
> Cross-builds for an unknown GNU/Linux, or as I like to call it "GNU plus Linux".

## off
```bash
cargo fix --broken-code --allow-dirty && cargo clippy --fix --allow-dirty --quiet >/dev/null 2>&1
```
> Removes all code that is redundant, applies cargo and clippy fixes

## book
```bash
mask update
mdbook serve --port 9999
```
