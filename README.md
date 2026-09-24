**Українська** · [English](README.en.md)

# kit-packager

CLI-оркестратор для релізу Rust desktop-застосунків:

```text
cargo build --release → ota-sign publish → gh release create
```

Інструмент не дублює Cargo, OTA-формат або GitHub CLI. Він перевіряє потрібні
файли, друкує фактичні зовнішні команди й завершується помилкою, якщо одна з них
повернула ненульовий exit code. Поточна версія: 0.1.0.

## Вимоги

| Інструмент | Потрібен для |
|---|---|
| Rust / Cargo | встановлення і `build` |
| [ota-sign](https://github.com/RiasJ1Dar/ota-sign) у `PATH` | `sign` |
| [GitHub CLI](https://cli.github.com/) з виконаним `gh auth login` | `release` |

```bash
cargo install --git https://github.com/RiasJ1Dar/ota-sign
cargo install --git https://github.com/RiasJ1Dar/kit-packager
```

Або встанови `kit-packager` із клону:

```bash
git clone https://github.com/RiasJ1Dar/kit-packager.git
cd kit-packager
cargo install --path .
```

## Повний сценарій

Один раз створи ключ OTA. Секретний файл не додавай у Git:

```bash
ota-sign keygen ./keys/release
```

Збери застосунок:

```bash
kit-packager build --project ./apps/host --bin host
```

Підпиши каталог з артефактами:

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

Створи GitHub Release:

```bash
kit-packager release \
  --tag v0.1.0 \
  --repo You/Host \
  --asset ./dist/host \
  --asset ./channel/manifest.json \
  --asset ./channel/manifest.sig \
  --notes "перший реліз"
```

Блоби OTA можна опублікувати через GitHub Pages / Raw або додати до окремого
архіву релізу.

## `build`

```bash
kit-packager build [--project DIR] [--bin NAME] [--target TRIPLE] [-- CARGO_ARGS...]
```

| Параметр | Значення |
|---|---|
| `--project DIR` | Тека з `Cargo.toml`; за замовчуванням `.` |
| `--bin NAME` | Передати Cargo `--bin NAME` |
| `--target TRIPLE` | Передати Cargo `--target TRIPLE` |
| аргументи після `--` | Додати їх до `cargo build --release` |

Приклад із feature:

```bash
kit-packager build --project . --bin my-app -- --features portable
```

Після успішної збірки команда друкує очікуваний шлях
`target/release/<name>` або `target/<triple>/release/<name>`; на Windows
перевіряє варіант із `.exe`.

## `sign`

```bash
kit-packager sign \
  --source PATH \
  [--out ./channel] \
  --app APP_ID \
  --version VERSION \
  --secret KEY.secret
```

`--source` може бути каталогом або одним файлом. Один файл копіюється в
сусідню теку `kit-packager-dist/`, після чого `ota-sign publish` отримує
каталог. Результат у `--out`:

```text
manifest.json
manifest.sig
blobs/<sha512>.bin
```

Якщо `ota-sign` відсутній, можна пропустити цей крок і опублікувати звичайний
бінарник через `release`.

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

Потрібен хоча б один `--asset`. Без `--repo` GitHub CLI визначає репозиторій
із Git remote поточної теки. Без `--title` використовується тег, без
`--notes` — `gh --generate-notes`.

Спочатку виконується `gh release create`. Якщо реліз із таким тегом уже існує,
CLI пробує `gh release upload --clobber`, тому повторний запуск оновлює
передані assets.

## Межі відповідальності

- Криптографія та формат OTA належать `ota-sign`.
- Authenticode-підписування Windows-бінарників тут не виконується.
- CI workflow має жити в репозиторії застосунку.
- CLI не змінює версію в `Cargo.toml` і не створює changelog.

## Розробка

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

## Ліцензія

MIT