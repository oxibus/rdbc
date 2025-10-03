use std::path::PathBuf;

use anyhow::Result;
use rrdbc::file::parser_dbc_file;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dbc2json", about = "Convert DBC file to JSON")]
struct Opt {
    /// Input file encoding
    #[structopt(short, long, default_value = "UTF-8")]
    encoding: String,

    /// Input dbc file
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    /// Output json file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();
    let network_ast = parser_dbc_file(opt.input.to_str().unwrap(), &opt.encoding)?;
    let network_ast_json = serde_json::to_string_pretty(&network_ast)?;
    std::fs::write(opt.output, network_ast_json)?;
    Ok(())
}
