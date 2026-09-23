use std::process::ExitStatus;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("command not found: {0} (install it or put it on PATH)")]
    MissingTool(&'static str),

    #[error("`{cmd}` exited with {status}")]
    CommandFailed { cmd: String, status: ExitStatus },

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Msg(String),
}

pub type Result<T> = std::result::Result<T, Error>;
