use anyhow::Result;
use rdbc::ast::network_ast::*;
use rdbc::file::read_file_content;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dbc2json", about = "Convert DBC file to JSON")]
struct Opt {
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
    let data = read_file_content(opt.input.to_str().unwrap())?;
    let network_ast = parse_dbc(&data)?;
    let network_ast_json = serde_json::to_string_pretty(&network_ast)?;
    std::fs::write(opt.output, network_ast_json)?;
    Ok(())
}
