use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use rrdbc::encoding::to_utf8;

#[derive(Debug, Parser)]
#[command(name = "toutf8", about = "Recode file to UTF-8", version)]
struct Opt {
    /// Input file encoding
    #[arg(short, long)]
    encoding: String,

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

    let data = to_utf8(opt.encoding.as_str(), &buffer)?;

    std::fs::write(opt.output, data)?;

    Ok(())
}
