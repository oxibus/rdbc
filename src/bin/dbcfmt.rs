use anyhow::Result;
use rrdbc::file::parser_dbc_file;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dbcfmt", about = "Format DBC file")]
struct Opt {
    /// Input file encoding
    #[structopt(short, long, default_value = "UTF-8")]
    encoding: String,

    /// Input dbc file
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();
    let network_ast = parser_dbc_file(opt.input.to_str().unwrap(), &opt.encoding)?;
    let output_data = format!("{}", network_ast);
    std::fs::write(opt.input, output_data)?;
    Ok(())
}
