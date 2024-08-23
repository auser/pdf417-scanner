use std::{path::PathBuf, process::Command};

use clap::Parser;

use crate::error::{DecodeError, DecodeResult};

#[derive(Debug, Clone, Parser)]
#[command(about = "Decode a barcode")]
pub struct GenArgs {
    #[arg(short, long)]
    pub num: Option<u32>,

    #[arg(short, long, default_value = "fixtures")]
    pub dir: Option<PathBuf>,
}

pub fn run(args: &GenArgs) -> DecodeResult<()> {
    let num = args.num.clone().unwrap_or(1);
    let dir = args.dir.clone().unwrap_or(PathBuf::from("fixtures"));

    let mut cmd = Command::new("poetry");
    cmd.args([
        "run",
        "python",
        "main.py",
        "--num",
        &num.to_string(),
        "--dir",
        &dir.to_string_lossy(),
    ])
    .current_dir("tests");

    let status = cmd.status()?;
    if !status.success() {
        return Err(DecodeError::RuntimeError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to generate samples",
        )));
    }

    Ok(())
}
