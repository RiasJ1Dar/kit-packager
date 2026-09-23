[Українська](README.md) · **English**

# kit-packager

Thin CLI around your existing stack: build a binary (`cargo`),
sign an OTA channel (`ota-sign`), publish a GitHub Release (`gh`).
Does **not** vendor ota-sign — shells out if it is on `PATH`.

## Install

```bash
# needs: cargo, gh (authenticated); optional: ota-sign
cargo install --git https://github.com/RiasJ1Dar/ota-sign
cargo install --git https://github.com/RiasJ1Dar/kit-packager
```

## Three steps

```bash
kit-packager build --project ../my-desktop-app --bin my-app

mkdir -p dist && cp target/release/my-app dist/
kit-packager sign \
  --source ./dist --out ./channel \
  --app MyApp --version 1.2.0 \
  --secret ./keys/release.secret

kit-packager release \
  --tag v1.2.0 \
  --repo You/my-desktop-app \
  --asset ./dist/my-app
```

See [README.md](README.md) for the full Ukrainian workflow notes.

## License

MIT
