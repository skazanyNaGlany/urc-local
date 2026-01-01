extern crate single_instance;

mod cmd_args;
mod consts;
mod errors;
mod utils;

use crate::errors::{
    ErrAlreadyInstalled, ErrAlreadyRunning, ErrCurrentlyUninstalled, ErrNoExePathname,
};
use anyhow::Result;
use auto_launch::AutoLaunch;
use clap::Parser;
use cmd_args::CmdArgs;
use consts::CARGO_PKG_NAME;
use realpath_ext::{RealpathFlags, realpath};
use single_instance::SingleInstance;
use std::env::{current_dir, current_exe, set_current_dir};
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<()> {
    let instance = SingleInstance::new(CARGO_PKG_NAME)?;

    if !instance.is_single() {
        return Err(ErrAlreadyRunning.into());
    }

    let mut cmd_args = CmdArgs::parse();

    set_working_dir_as_exe()?;
    resolve_paths(&mut cmd_args)?;

    if cmd_args.status {
        print_status(&cmd_args)?;
    } else if cmd_args.run {
        run_rc_file(&cmd_args)?;
    } else if cmd_args.install {
        install(&cmd_args)?;
    } else if cmd_args.uninstall {
        uninstall(&cmd_args)?;
    }

    return Ok(());
}

fn install(cmd_args: &CmdArgs) -> Result<()> {
    let auto_launch = get_auto_launch_instance(cmd_args)?;

    if auto_launch.is_enabled()? {
        return Err(ErrAlreadyInstalled.into());
    }

    auto_launch.enable()?;

    println!("Installed");

    return Ok(());
}

fn uninstall(cmd_args: &CmdArgs) -> Result<()> {
    let auto_launch = get_auto_launch_instance(cmd_args)?;

    if !auto_launch.is_enabled()? {
        return Err(ErrCurrentlyUninstalled.into());
    }

    auto_launch.disable()?;

    println!("Uninstalled");

    return Ok(());
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn run_rc_file(cmd_args: &CmdArgs) -> Result<()> {
    println!("Running RC file at {}", cmd_args.file);

    let mut process = Command::new("sh").args([&cmd_args.file]).spawn()?;
    let exit_status = process.wait()?;

    println!("Process exit status: {}", exit_status);

    return Ok(());
}

#[cfg(any(target_os = "windows"))]
fn run_rc_file(cmd_args: &CmdArgs) -> Result<()> {
    println!("Running RC file at {}", cmd_args.file);

    let mut process = Command::new(&cmd_args.file).spawn()?;
    let exit_status = process.wait()?;

    println!("Process exit status: {}", exit_status);

    return Ok(());
}

fn get_auto_launch_instance(cmd_args: &CmdArgs) -> Result<AutoLaunch> {
    match current_exe()?.to_str() {
        Some(current_exe) => {
            return Ok(AutoLaunch::new(
                CARGO_PKG_NAME,
                current_exe,
                &["--run", "--file", &cmd_args.file],
            ));
        }
        _ => {
            return Err(ErrNoExePathname.into());
        }
    }
}

fn print_status(cmd_args: &CmdArgs) -> Result<()> {
    let auto_launch = get_auto_launch_instance(cmd_args)?;

    if auto_launch.is_enabled()? {
        println!("App is installed in the startup")
    } else {
        println!("App is not installed in the startup")
    }

    return Ok(());
}

fn resolve_paths(cmd_args: &mut CmdArgs) -> Result<()> {
    resolve_rc_local_path(cmd_args)?;

    return Ok(());
}

fn resolve_rc_local_path(cmd_args: &mut CmdArgs) -> Result<()> {
    println!("Getting real path of RC file {}", &cmd_args.file);

    match realpath(&cmd_args.file, RealpathFlags::empty())?.to_str() {
        Some(real_rc_file_pathname) => {
            cmd_args.file = real_rc_file_pathname.to_string();

            println!("RC file real path: {}", cmd_args.file);
        }
        _ => {
            println!("Cannot get a real path for RC file (is is valid?)")
        }
    }

    return Ok(());
}

fn set_working_dir_as_exe() -> Result<()> {
    println!("Setting current working directory to the location of the executable");

    match PathBuf::from(current_exe()?).as_path().parent() {
        Some(current_exe_path) => {
            set_current_dir(current_exe_path)?;
        }
        _ => {}
    }

    println!(
        "Current working directory: {}",
        current_dir()?.to_string_lossy()
    );

    return Ok(());
}
