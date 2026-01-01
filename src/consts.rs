use const_format::formatcp;

#[cfg(target_os = "windows")]
pub(crate) const RC_FILE_NAME: &str = ".\\rc_local.bat";

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) const RC_FILE_NAME: &str = "./rc_local.sh";

pub(crate) const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub(crate) const EXT_HELP_MSG: &str = formatcp!(
    "Universal {rc_file_name} runner.\n
You can use this app to run file {rc_file_name} automatically at system startup.",
    rc_file_name = RC_FILE_NAME
);
