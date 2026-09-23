use crate::error::{Error, Result};
use crate::run;
use std::path::{Path, PathBuf};

pub struct SignOpts {
    /// Directory of files to publish (e.g. `./dist` with the binary).
    pub source: PathBuf,
    /// Output OTA channel directory.
    pub out: PathBuf,
    pub app: String,
    pub version: String,
    /// Path to `*.secret` hex key for ota-sign.
    pub secret: PathBuf,
}

/// Call `ota-sign publish` if the CLI is on PATH.
/// Does not vendor ota-sign — install it separately:
///   cargo install --git https://github.com/RiasJ1Dar/ota-sign
pub fn sign(opts: &SignOpts) -> Result<()> {
    if !run::which("ota-sign") {
        return Err(Error::Msg(
            "ota-sign not on PATH. Install: cargo install --git https://github.com/RiasJ1Dar/ota-sign\n\
             Or skip sign and use `kit-packager release` with plain assets."
                .into(),
        ));
    }
    if !opts.secret.exists() {
        return Err(Error::Msg(format!(
            "secret key not found: {} (ota-sign keygen ./keys/release)",
            opts.secret.display()
        )));
    }
    std::fs::create_dir_all(&opts.out)?;

    let args = [
        "publish".to_string(),
        "--source".into(),
        opts.source.display().to_string(),
        "--out".into(),
        opts.out.display().to_string(),
        "--app".into(),
        opts.app.clone(),
        "--version".into(),
        opts.version.clone(),
        "--secret".into(),
        opts.secret.display().to_string(),
    ];
    eprintln!("→ ota-sign {}", args.join(" "));
    run::tool("ota-sign", &args, None)?;
    eprintln!(
        "signed channel: {} (manifest.json + manifest.sig + blobs/)",
        opts.out.display()
    );
    Ok(())
}

/// Ensure `source` exists; if given a single file, copy it into a temp-ish dist dir.
pub fn ensure_source_dir(source: &Path) -> Result<PathBuf> {
    if source.is_dir() {
        return Ok(source.to_path_buf());
    }
    if source.is_file() {
        let parent = source
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("kit-packager-dist");
        std::fs::create_dir_all(&parent)?;
        let dest = parent.join(
            source
                .file_name()
                .ok_or_else(|| Error::Msg("empty source file name".into()))?,
        );
        std::fs::copy(source, &dest)?;
        return Ok(parent);
    }
    Err(Error::Msg(format!(
        "source not found: {}",
        source.display()
    )))
}
