use crate::error::{Error, Result};
use crate::run;
use std::path::PathBuf;

pub struct ReleaseOpts {
    /// `owner/repo` for `gh release`. If None, use current git remote.
    pub repo: Option<String>,
    /// Tag name, e.g. `v1.2.0`.
    pub tag: String,
    /// Release title (default: tag).
    pub title: Option<String>,
    /// Optional notes / changelog body.
    pub notes: Option<String>,
    /// Files or directories to upload as release assets.
    pub assets: Vec<PathBuf>,
    /// Create as draft.
    pub draft: bool,
    /// Mark as prerelease.
    pub prerelease: bool,
}

/// Create (or update assets on) a GitHub Release via `gh`.
pub fn release(opts: &ReleaseOpts) -> Result<()> {
    if !run::which("gh") {
        return Err(Error::MissingTool("gh"));
    }

    let mut args: Vec<String> = vec![
        "release".into(),
        "create".into(),
        opts.tag.clone(),
    ];
    if let Some(repo) = &opts.repo {
        args.push("--repo".into());
        args.push(repo.clone());
    }
    let title = opts.title.as_deref().unwrap_or(&opts.tag);
    args.push("--title".into());
    args.push(title.to_string());
    if let Some(notes) = &opts.notes {
        args.push("--notes".into());
        args.push(notes.clone());
    } else {
        args.push("--generate-notes".into());
    }
    if opts.draft {
        args.push("--draft".into());
    }
    if opts.prerelease {
        args.push("--prerelease".into());
    }
    for asset in &opts.assets {
        if !asset.exists() {
            return Err(Error::Msg(format!(
                "asset not found: {}",
                asset.display()
            )));
        }
        args.push(asset.display().to_string());
    }

    eprintln!("→ gh {}", args.join(" "));
    // If the release already exists, fall back to uploading assets.
    match run::tool("gh", &args, None) {
        Ok(()) => Ok(()),
        Err(Error::CommandFailed { .. }) if !opts.assets.is_empty() => {
            eprintln!("release create failed — trying `gh release upload` for existing tag");
            upload_assets(opts)
        }
        Err(e) => Err(e),
    }
}

fn upload_assets(opts: &ReleaseOpts) -> Result<()> {
    let mut args: Vec<String> = vec![
        "release".into(),
        "upload".into(),
        opts.tag.clone(),
        "--clobber".into(),
    ];
    if let Some(repo) = &opts.repo {
        args.push("--repo".into());
        args.push(repo.clone());
    }
    for asset in &opts.assets {
        args.push(asset.display().to_string());
    }
    eprintln!("→ gh {}", args.join(" "));
    run::tool("gh", &args, None)
}
