use clap::Parser;
use log::LevelFilter;

use self::{decode::DecodeArgs, gen_samples::GenArgs};
use crate::error::DecodeResult;

mod decode;
mod gen_samples;

#[derive(Debug, Clone, Parser)]
#[command(
    author,
    version,
    about,
    long_about = r#"
PDF417er is a tool for decoding PDF417 barcodes.
"#
)]
pub struct DecoderArgs {
    #[arg(short, long)]
    pub verbose: Option<LevelFilter>,

    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Clone, Parser)]
pub enum Subcommand {
    Decode(DecodeArgs),
    GenSamples(GenArgs),
}

pub fn execute() -> DecodeResult<()> {
    let args = DecoderArgs::parse();
    log::set_max_level(args.verbose.unwrap_or(LevelFilter::Info));
    env_logger::init();

    match args.subcommand {
        Subcommand::Decode(decode_args) => decode::run(&decode_args),
        Subcommand::GenSamples(gen_args) => gen_samples::run(&gen_args),
    }
}
