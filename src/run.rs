use crate::error::{Error, Result};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Run an external tool; stream stdout/stderr to the user.
pub fn tool<I, S>(name: &'static str, args: I, cwd: Option<&Path>) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new(name);
    cmd.args(args).stdin(Stdio::null());
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(Error::MissingTool(name));
        }
        Err(e) => return Err(e.into()),
    };
    if status.success() {
        Ok(())
    } else {
        Err(Error::CommandFailed {
            cmd: name.to_string(),
            status,
        })
    }
}

/// True if `name` resolves on PATH. Does **not** run `--version`: tools like
/// `ota-sign` only expose `--help`, and a failed version probe looked like "missing".
pub fn which(name: &str) -> bool {
    resolve(name).is_some()
}

fn resolve(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
        #[cfg(windows)]
        {
            let exe = dir.join(format!("{name}.exe"));
            if exe.is_file() {
                return Some(exe);
            }
        }
    }
    None
}
