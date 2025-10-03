use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use rrdbc::encoding::utf8_to_gbk;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "utf82gbk", about = "Recode file from UTF-8 to GBK")]
struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();

    let mut file = File::open(opt.input)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let data = utf8_to_gbk(&buffer)?;

    std::fs::write(opt.output, data)?;

    Ok(())
}
