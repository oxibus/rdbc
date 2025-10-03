use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use rrdbc::encoding::gbk_to_utf8;

#[derive(Debug, Parser)]
#[command(name = "gbk2utf8", about = "Recode file from GBK to UTF-8", version)]
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

    let data = gbk_to_utf8(&buffer)?;

    std::fs::write(opt.output, data)?;

    Ok(())
}
