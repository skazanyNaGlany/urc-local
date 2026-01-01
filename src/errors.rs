use thiserror::Error;

#[derive(Debug, Error)]
#[error("app is already running")]
pub struct ErrAlreadyRunning;

#[derive(Debug, Error)]
#[error("unable to get pathname of current executable file (it is valid?")]
pub struct ErrNoExePathname;

#[derive(Debug, Error)]
#[error("app is already installed in the startup")]
pub struct ErrAlreadyInstalled;

#[derive(Debug, Error)]
#[error("app is not installed in the startup")]
pub struct ErrCurrentlyUninstalled;
