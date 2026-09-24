[Українська](README.md) · **English**

# kit-packager

A small release orchestrator for Rust desktop applications:

```text
cargo build --release → ota-sign publish → gh release create
```

It does not reimplement Cargo, the OTA format, or GitHub CLI. It validates the
required files, prints each external command, and fails when a command returns a
non-zero exit status. Current version: 0.1.0.

## Requirements

| Tool | Used by |
|---|---|
| Rust / Cargo | installation and `build` |
| [ota-sign](https://github.com/RiasJ1Dar/ota-sign) on `PATH` | `sign` |
| [GitHub CLI](https://cli.github.com/), authenticated with `gh auth login` | `release` |

```bash
cargo install --git https://github.com/RiasJ1Dar/ota-sign
cargo install --git https://github.com/RiasJ1Dar/kit-packager
```

Or install `kit-packager` from a clone:

```bash
git clone https://github.com/RiasJ1Dar/kit-packager.git
cd kit-packager
cargo install --path .
```

## Complete workflow

Create the OTA key once and keep the secret file out of Git:

```bash
ota-sign keygen ./keys/release
```

Build the application:

```bash
kit-packager build --project ./apps/host --bin host
```

Sign a directory of artifacts:

```bash
mkdir -p dist
cp apps/host/target/release/host dist/

kit-packager sign \
  --source ./dist \
  --out ./channel \
  --app Host \
  --version 0.1.0 \
  --secret ./keys/release.secret
```

Create the GitHub Release:

```bash
kit-packager release \
  --tag v0.1.0 \
  --repo You/Host \
  --asset ./dist/host \
  --asset ./channel/manifest.json \
  --asset ./channel/manifest.sig \
  --notes "first release"
```

OTA blobs can be hosted through GitHub Pages / Raw or attached as a separate
release archive.

## `build`

```bash
kit-packager build [--project DIR] [--bin NAME] [--target TRIPLE] [-- CARGO_ARGS...]
```

| Option | Meaning |
|---|---|
| `--project DIR` | Directory containing `Cargo.toml`; defaults to `.` |
| `--bin NAME` | Pass `--bin NAME` to Cargo |
| `--target TRIPLE` | Pass `--target TRIPLE` to Cargo |
| arguments after `--` | Append them to `cargo build --release` |

Example with a feature:

```bash
kit-packager build --project . --bin my-app -- --features portable
```

After a successful build, the command prints the expected
`target/release/<name>` or `target/<triple>/release/<name>` path; on Windows
it checks the `.exe` variant.

## `sign`

```bash
kit-packager sign \
  --source PATH \
  [--out ./channel] \
  --app APP_ID \
  --version VERSION \
  --secret KEY.secret
```

`--source` may be a directory or one file. A single file is copied into a
sibling `kit-packager-dist/` directory before `ota-sign publish` receives it.
The output under `--out` is:

```text
manifest.json
manifest.sig
blobs/<sha512>.bin
```

If `ota-sign` is unavailable, skip this step and publish a plain binary with
`release`.

## `release`

```bash
kit-packager release \
  --tag TAG \
  [--repo OWNER/REPO] \
  [--title TEXT] \
  [--notes TEXT] \
  --asset PATH [--asset PATH ...] \
  [--draft] [--prerelease]
```

At least one `--asset` is required. Without `--repo`, GitHub CLI resolves the
repository from the current Git remote. The title defaults to the tag; omitted
notes use `gh --generate-notes`.

The CLI first runs `gh release create`. If a release for the tag already
exists, it tries `gh release upload --clobber`, so a repeated run replaces the
supplied assets.

## Scope

- OTA cryptography and channel format belong to `ota-sign`.
- Windows Authenticode signing is not performed.
- CI workflows belong in the application repository.
- The CLI does not edit `Cargo.toml` versions or generate a changelog.

## Development

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

## License

MIT