//! Thin release orchestrator: `cargo build` → `ota-sign` → `gh release`.
//! Does not vendor ota-sign; calls the CLI if present on PATH.

mod build;
mod error;
mod release;
mod run;
mod sign;

use clap::{Parser, Subcommand};
use error::{Error, Result};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "kit-packager",
    about = "Build, sign (ota-sign), and publish GitHub Releases for desktop kit binaries"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// `cargo build --release` in a project directory.
    Build {
        /// Project root (Cargo.toml). Default: `.`
        #[arg(long, default_value = ".")]
        project: PathBuf,
        /// Pass `--bin NAME` to cargo.
        #[arg(long)]
        bin: Option<String>,
        /// Pass `--target TRIPLE` to cargo.
        #[arg(long)]
        target: Option<String>,
        /// Extra args after `cargo build --release` (use `--` separator).
        #[arg(last = true)]
        cargo_args: Vec<String>,
    },
    /// Sign/publish an OTA channel via `ota-sign publish` (must be on PATH).
    Sign {
        /// Directory (or single binary) to publish.
        #[arg(long)]
        source: PathBuf,
        /// Output channel directory (manifest + blobs).
        #[arg(long, default_value = "./channel")]
        out: PathBuf,
        /// App id stored in the manifest.
        #[arg(long)]
        app: String,
        /// Version string stored in the manifest.
        #[arg(long)]
        version: String,
        /// Path to `*.secret` hex key (`ota-sign keygen`).
        #[arg(long)]
        secret: PathBuf,
    },
    /// Create a GitHub Release and upload assets via `gh`.
    Release {
        /// Tag, e.g. `v1.2.0`.
        #[arg(long)]
        tag: String,
        /// `owner/repo`. Default: current git remote via gh.
        #[arg(long)]
        repo: Option<String>,
        /// Release title (default: tag).
        #[arg(long)]
        title: Option<String>,
        /// Release notes body. If omitted, `--generate-notes`.
        #[arg(long)]
        notes: Option<String>,
        /// Asset files to upload.
        #[arg(long = "asset", value_name = "PATH")]
        assets: Vec<PathBuf>,
        /// Create as draft.
        #[arg(long)]
        draft: bool,
        /// Mark as prerelease.
        #[arg(long)]
        prerelease: bool,
    },
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Build {
            project,
            bin,
            target,
            cargo_args,
        } => {
            let artifact = build::build(&build::BuildOpts {
                project,
                bin,
                target,
                cargo_args,
            })?;
            println!("{}", artifact.display());
            Ok(())
        }
        Cmd::Sign {
            source,
            out,
            app,
            version,
            secret,
        } => {
            let source = sign::ensure_source_dir(&source)?;
            sign::sign(&sign::SignOpts {
                source,
                out,
                app,
                version,
                secret,
            })
        }
        Cmd::Release {
            tag,
            repo,
            title,
            notes,
            assets,
            draft,
            prerelease,
        } => {
            if assets.is_empty() {
                return Err(Error::Msg(
                    "pass at least one --asset PATH (binary, archive, or OTA channel files)"
                        .into(),
                ));
            }
            release::release(&release::ReleaseOpts {
                repo,
                tag,
                title,
                notes,
                assets,
                draft,
                prerelease,
            })
        }
    }
}
