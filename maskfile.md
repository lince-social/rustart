# Creatus Maskfile

## run
```bash
cargo install mprocs --locked
mprocs \
    "bacon . --job seed" \
    "cargo run -- --docs" \
    "docker ps" \
    "docker compose -f database-compose.yml down" \
    "docker compose -f database-compose.yml up -d"
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
