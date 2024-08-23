use clap::Parser;
use rxing::BarcodeFormat;

use crate::error::DecodeResult;

#[derive(Debug, Clone, Parser)]
#[command(about = "Decode a barcode")]
pub struct DecodeArgs {
    #[clap(value_name = "FILE", index = 1)]
    pub file: String,

    #[arg(long, help = "The format of the barcode to decode")]
    pub format: Option<BarcodeFormat>,
}

pub fn run(args: &DecodeArgs) -> DecodeResult<()> {
    let file = args.file.clone();

    let format = args.format.clone().map(|f| f.into());
    let res = rxing::helpers::detect_in_file(&file, format)?;

    let text = res.getText();

    println!("{}", text);

    Ok(())
}
