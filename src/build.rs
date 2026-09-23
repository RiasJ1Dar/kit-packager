use crate::error::Result;
use crate::run;
use std::path::{Path, PathBuf};

pub struct BuildOpts {
    /// Cargo project root (directory with Cargo.toml). Default: `.`
    pub project: PathBuf,
    /// Extra args after `cargo build --release`.
    pub cargo_args: Vec<String>,
    /// Optional binary name (`--bin`).
    pub bin: Option<String>,
    /// Optional target triple (`--target`).
    pub target: Option<String>,
}

/// Shell out to `cargo build --release` in the project directory.
pub fn build(opts: &BuildOpts) -> Result<PathBuf> {
    let mut args: Vec<String> = vec!["build".into(), "--release".into()];
    if let Some(bin) = &opts.bin {
        args.push("--bin".into());
        args.push(bin.clone());
    }
    if let Some(target) = &opts.target {
        args.push("--target".into());
        args.push(target.clone());
    }
    args.extend(opts.cargo_args.iter().cloned());

    eprintln!("→ cargo {}", args.join(" "));
    run::tool("cargo", &args, Some(&opts.project))?;

    let artifact = guess_artifact(&opts.project, opts.bin.as_deref(), opts.target.as_deref())?;
    eprintln!("built: {}", artifact.display());
    Ok(artifact)
}

fn guess_artifact(project: &Path, bin: Option<&str>, target: Option<&str>) -> Result<PathBuf> {
    let name = match bin {
        Some(b) => b.to_string(),
        None => {
            // Prefer package name from Cargo.toml `[package] name = "…"`.
            let toml = std::fs::read_to_string(project.join("Cargo.toml")).unwrap_or_default();
            parse_package_name(&toml).unwrap_or_else(|| "app".into())
        }
    };
    let mut path = project.join("target");
    if let Some(t) = target {
        path = path.join(t);
    }
    path = path.join("release").join(&name);
    #[cfg(windows)]
    {
        let exe = path.with_extension("exe");
        if exe.exists() {
            return Ok(exe);
        }
    }
    if path.exists() {
        Ok(path)
    } else {
        // Still return the expected path so callers can wire the next step.
        Ok(path)
    }
}

fn parse_package_name(toml: &str) -> Option<String> {
    let mut in_package = false;
    for line in toml.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            in_package = t == "[package]";
            continue;
        }
        if in_package {
            if let Some(rest) = t.strip_prefix("name") {
                let rest = rest.trim().strip_prefix('=')?.trim();
                let name = rest.trim_matches('"').trim_matches('\'').to_string();
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
    }
    None
}
