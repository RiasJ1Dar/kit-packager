use crate::error::{Error, Result};
use std::ffi::OsStr;
use std::path::Path;
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

pub fn which(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
