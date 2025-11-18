# Rustart
This is a template to start projects in rust: Ru[st(art)].

# Versioning

To automatically have new versions and releases, it's essential to follow a commit standard: [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

This project uses the [Git Cliff](https://github.com/orhun/git-cliff) crate along with the `mask release` command to perform **automatic version bumps**, create tags and releases on GitHub, and update the `CHANGELOG.md`.

# Recommended Commit Structure

\<type\>(\<optional scope\>): \<short message\>

- **type**: `fix`, `feat`, `perf`, `security`, etc.
- **scope**: affected module or component (optional but recommended).
- **message**: brief description of what changed.

> Examples:
> ```bash
> git commit -m "fix(auth): fix login bug"
> git commit -m "feat(export): add PDF export"
> git commit -m "perf(db): optimize report query"
> git commit -m "security(auth): fix CSRF vulnerability"
> git commit -m "feat(api): new login route (BREAKING CHANGE)"
> ```

# Commit Types and Version Impact

| Commit Prefix     | Version Bump    | Meaning |
|------------------|----------------|------------|
| `fix`            | Patch (`*.*.X`) | Bug fix without changing existing functionality. |
| `perf`           | Patch (`*.*.X`) | Performance improvement without breaking compatibility. |
| `security`       | Patch (`*.*.X`) | Fix for security vulnerabilities. |
| `feat`           | Minor (`*.X.*`) | New feature or compatible improvement. |
| `BREAKING CHANGE`| Major (`X.*.*`) | Change that breaks compatibility with previous versions. |

> ⚠️ Commits with prefixes like `chore`, `build`, `ci`, `docs`, `style`, `test`, or `refactor` **do not generate a version bump**, but still appear in the changelog.

# Commands

You can run the application using [mask](https://github.com/jacobdeichert/mask), installed with:

```bash
cargo install mask
```

After that, enter these commands in the terminal (mask + HEADERS_BELOW). Example: 'mask up'.

## upgrade
```bash
rustup self update
rustup update stable
cargo install cargo-edit cargo-udeps --locked
cargo upgrade
```
> Runst several Rust toolchain's commands for making the system up to date

## docs
```bash
cargo install typst-cli --locked
zathura documentation/documentation.pdf &
typst watch --font-path documentation/font/IBM_Plex_Sans/static documentation/documentation.typ
```
> Starts typst documentation with mprocs

## up
```bash
docker compose -f persistence-compose.yml up -d
```
> Runs Docker Compose up

## down
```bash
docker compose -f persistence-compose.yml down
```
> Runs Docker Compose down

## run
```bash
cargo install bacon mprocs --locked

if [[ -d .env ]]; then
    cp .env.example .env
fi

mprocs \
"bacon --job run ." \
"docker ps" \
"mask up" \
"mask down" \
```
> Installs mprocs if not installed and uses it for running all the processes one might need.

## off
```bash
mask upgrade
cargo fix --broken-code --allow-dirty && cargo clippy --fix --allow-dirty --quiet >/dev/null 2>&1
```
> Removes all code that is redundant, applies cargo and clippy fixes

## release
```bash
#!/usr/bin/env bash
set -euo pipefail

branch=$(git rev-parse --abbrev-ref HEAD)

if [[ ! -f "cliff.toml" ]]; then
    echo "⚙️  Initializing git-cliff config..."
    git cliff --init
fi

if git describe --tags --abbrev=0 &>/dev/null; then
    last_tag=$(git describe --tags --abbrev=0)
else
    last_tag=""
fi
echo "Last tag: ${last_tag:-<none>}"

if [[ ! -f CHANGELOG.md ]]; then
    echo "📝 Creating initial CHANGELOG.md..."
    touch CHANGELOG.md
fi

if [[ -n "$last_tag" ]]; then
    NEXT_VERSION=$(git cliff --bumped-version)
else
    NEXT_VERSION="0.1.0"
fi

read -rp "Next version (auto: $NEXT_VERSION): " input_version
VERSION=${input_version:-$NEXT_VERSION}

read -rp "Release title (optional, press enter to use '$VERSION'): " input_title
TITLE=${input_title:-"Release $VERSION"}

git cliff --unreleased --bump --tag "$VERSION" -o CHANGELOG.md

git add CHANGELOG.md
git commit -m "chore(release): $VERSION"
git tag -a "$VERSION" -m "$TITLE"
git push origin "$branch"
git push origin "$VERSION"

if command -v gh &>/dev/null; then
    echo "📦 Creating GitHub release for $VERSION..."
    gh release create "$VERSION" -F CHANGELOG.md --title "$TITLE"
    echo "✅ GitHub release created."
else
    echo "⚠️  'gh' CLI not found. Install it with:"
    echo "    sudo pacman -S github-cli && gh auth login"
    echo "Then rerun 'mask release' to auto-publish the release."
fi

echo "✅ Released $VERSION from $branch"
```
> Creates a new release, with automatic or manual version bump. Tries to install Arch tools to do it.

## cross
```bash
docker pull ghcr.io/cross-rs/x86_64-unknown-linux-gnu:main --platform linux/x86_64
cross build --target x86_64-unknown-linux-gnu --release
```
> Cross-builds for an unknown GNU/Linux, or as I like to call it "GNU plus Linux".
