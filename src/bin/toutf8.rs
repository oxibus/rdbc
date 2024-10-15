use anyhow::Result;
use rrdbc::encoding::to_utf8;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "toutf8", about = "Recode file to UTF-8")]
struct Opt {
    /// Input file encoding
    #[structopt(short, long)]
    encoding: String,

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

    let data = to_utf8(opt.encoding.as_str(), &buffer)?;

    std::fs::write(opt.output, data)?;

    Ok(())
}
