use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(flatten)]
    pub verbose: Verbosity,
    #[arg(
        short,
        long,
        name = "dygma-support",
        help = "Read information from Dygma Neuron"
    )]
    pub dygma_support: bool,
    #[arg(
        short,
        long,
        name = "bluetooth-names",
        help = "Read information about BLE devices"
    )]
    pub bluetooth_support: bool,
    #[arg(
        short,
        long,
        name = "json",
        help = "Format output as json"
    )]
    pub json: bool,

}
