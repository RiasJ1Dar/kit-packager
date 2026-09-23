**Українська** · [English](README.en.md)

# kit-packager

CLI-обгортка навколо вже наявного стеку: збирає бінарник (`cargo`),
підписує OTA-канал (`ota-sign`), викладає GitHub Release (`gh`).
Без вендорингу `ota-sign` — лише виклик, якщо він у `PATH`.

## Навіщо

Для десктопних кітів ([desktop-remote-kit](https://github.com/RiasJ1Dar/desktop-remote-kit)
та хостів на ньому) реліз — це три кроки, які легко злити в один скрипт.
Цей інструмент і є той скрипт, у вигляді Rust CLI.

## Встановлення

```bash
# залежності
# - rustup / cargo
# - gh (https://cli.github.com), авторизований
# - опційно: ota-sign
cargo install --git https://github.com/RiasJ1Dar/ota-sign

cargo install --git https://github.com/RiasJ1Dar/kit-packager
# або з клону:
cargo install --path .
```

## Три кроки

### 1. `build` — зібрати release-бінарник

```bash
kit-packager build --project ../my-desktop-app
# або з іменем бінарника / таргетом:
kit-packager build --project . --bin my-app --target x86_64-pc-windows-gnu
```

Друкує шлях до артефакту в `target/release/…` (або `target/<triple>/release/…`).

### 2. `sign` — OTA-канал через ota-sign

Потрібен `ota-sign` у `PATH` і секретний ключ (`ota-sign keygen ./keys/release`).

```bash
mkdir -p dist && cp target/release/my-app dist/
kit-packager sign \
  --source ./dist \
  --out ./channel \
  --app MyApp \
  --version 1.2.0 \
  --secret ./keys/release.secret
```

Результат: `channel/manifest.json`, `channel/manifest.sig`, `channel/blobs/…`
(формат — як у [ota-sign](https://github.com/RiasJ1Dar/ota-sign)).

Якщо `ota-sign` немає — команда падає з підказкою; можна одразу йти на `release`
із сирим `.exe` / архівом.

### 3. `release` — GitHub Release через `gh`

```bash
kit-packager release \
  --tag v1.2.0 \
  --repo RiasJ1Dar/my-desktop-app \
  --asset ./dist/my-app \
  --asset ./channel/manifest.json \
  --asset ./channel/manifest.sig
# blobs можна залити окремо (Pages / raw) або архівувати:
# tar czf channel.tar.gz -C channel . && --asset channel.tar.gz
```

Якщо тег уже є — спробує `gh release upload --clobber`.

## Типовий workflow для desktop kit

```bash
# ключ один раз
ota-sign keygen ./keys/release   # *.secret не в git

kit-packager build --project ./apps/host --bin host
mkdir -p dist && cp target/release/host dist/

kit-packager sign \
  --source ./dist --out ./channel \
  --app Host --version 0.1.0 \
  --secret ./keys/release.secret

kit-packager release \
  --tag v0.1.0 \
  --repo You/Host \
  --asset dist/host \
  --notes "перший реліз"
```

Клієнт з `desktop-remote-kit` бачить тег через `releases/latest`;
підтягування файлів — через `ota-sign apply-url`, якщо канал викладено.

## Чого навмисне немає

- Власної криптографії / формату OTA — це `ota-sign`.
- Підпису коду Windows (Authenticode) — окремо, сертифікат у хоста.
- CI-workflow у цьому репо — додай у хост-проєкті за потреби.

## Ліцензія

MIT
