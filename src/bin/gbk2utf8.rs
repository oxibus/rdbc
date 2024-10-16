use anyhow::Result;
use rrdbc::encoding::gbk_to_utf8;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gbk2utf8", about = "Recode file from GBK to UTF-8")]
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

    let data = gbk_to_utf8(&buffer)?;

    std::fs::write(opt.output, data)?;

    Ok(())
}
