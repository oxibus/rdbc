#[path = "../_bin_utils.rs"]
pub mod bin_utils;

use std::path::PathBuf;

use anyhow::Result;
use bin_utils::parser_dbc_file;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "dbc2json", about = "Convert DBC file to JSON", version)]
struct Opt {
    /// Input file encoding
    #[arg(short, long, default_value = "UTF-8")]
    encoding: String,

    /// Input dbc file
    input: PathBuf,

    /// Output json file
    output: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::parse();
    let network_ast = parser_dbc_file(opt.input, &opt.encoding)?;
    let network_ast_json = serde_json::to_string_pretty(&network_ast)?;
    std::fs::write(opt.output, network_ast_json)?;
    Ok(())
}
