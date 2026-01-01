use clap::Parser;

use crate::consts::{EXT_HELP_MSG, RC_FILE_NAME};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = EXT_HELP_MSG)]
pub(crate) struct CmdArgs {
    #[arg(
        short,
        long,
        default_value = RC_FILE_NAME,
        help = "Use a custom RC file instead of default"
    )]
    pub(crate) file: String,

    #[arg(short, long, help = "Check if binary is installed in the startup")]
    pub(crate) status: bool,

    #[arg(short, long, help = "Install binary in the startup")]
    pub(crate) install: bool,

    #[arg(short, long, help = "Uninstall binary from the startup")]
    pub(crate) uninstall: bool,

    #[arg(short, long, help = "Run the RC file")]
    pub(crate) run: bool,

    #[arg(short = 'o', long, help = "Allow only one app instance")]
    pub(crate) single_instance: bool,

    #[arg(
        short,
        long,
        help = "do not output any messages, except errors (RC file can still output some messages)"
    )]
    pub(crate) quiet: bool,
}
