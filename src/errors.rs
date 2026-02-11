use daemonize;
use thiserror;

#[derive(Debug, thiserror::Error)]
#[error("app is already running")]
pub struct ErrAlreadyRunning;

#[derive(Debug, thiserror::Error)]
#[error("unable to get pathname of current executable file (it is valid?")]
pub struct ErrNoExePathname;

#[derive(Debug, thiserror::Error)]
#[error("app is already installed in the startup")]
pub struct ErrAlreadyInstalled;

#[derive(Debug, thiserror::Error)]
#[error("app is not installed in the startup")]
pub struct ErrCurrentlyUninstalled;

#[derive(Debug, thiserror::Error)]
#[error("daemonize error: {0}")]
pub struct ErrDaemonize(#[from] pub daemonize::Error);

#[cfg(any(target_os = "windows"))]
#[derive(Error, Debug)]
#[error("cannot daemonize on that platform")]
pub struct ErrCannotDaemonize;
