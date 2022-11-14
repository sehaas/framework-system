//! Module to factor out commandline interaction
//! This way we can use it in the regular OS commandline tool on Linux and Windows,
//! as well as on the UEFI shell tool.
use clap::Parser;

use crate::commandline::Cli;

/// Swiss army knife for Framework laptops
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct ClapCli {
    /// List current firmware versions version
    #[arg(short, long)]
    versions: bool,

    /// Show current power status (battery and AC)
    #[arg(long)]
    power: bool,

    /// Show information about USB-C PD prots
    #[arg(long)]
    pdports: bool,

    /// Show info from SMBIOS (Only on UEFI)
    //#[arg(long)]
    //info: bool,

    /// Show privacy switch statuses (camera and microphone)
    #[arg(long)]
    privacy: bool,

    /// Parse versions from PD firmware binary file
    #[arg(long)]
    pd_bin: Option<std::path::PathBuf>,

    /// Parse versions from EC firmware binary file
    #[arg(long)]
    ec_bin: Option<std::path::PathBuf>,

    /// Run self-test to check if interaction with EC is possible
    #[arg(long, short)]
    test: bool,
}

pub fn parse(args: &[String]) -> Cli {
    let args = ClapCli::parse_from(args);

    Cli {
        versions: args.versions,
        power: args.power,
        pdports: args.pdports,
        privacy: args.privacy,
        pd_bin: args
            .pd_bin
            .map(|x| x.into_os_string().into_string().unwrap()),
        ec_bin: args
            .ec_bin
            .map(|x| x.into_os_string().into_string().unwrap()),
        test: args.test,
        // TODO: Set help. Not very important because Clap handles this by itself
        help: false,
        // UEFI only for now. Don't need to handle
        allupdate: false,
        info: false,
        raw_command: vec![],
    }
}