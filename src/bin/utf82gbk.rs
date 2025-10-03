use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use rrdbc::encoding::utf8_to_gbk;

#[derive(Debug, Parser)]
#[command(name = "utf82gbk", about = "Recode file from UTF-8 to GBK", version)]
struct Opt {
    /// Input file
    input: PathBuf,

    /// Output file
    output: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::parse();

    let mut file = File::open(opt.input)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let data = utf8_to_gbk(&buffer)?;

    std::fs::write(opt.output, data)?;

    Ok(())
}
