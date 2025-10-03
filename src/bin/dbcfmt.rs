#[path = "../_bin_utils.rs"]
pub mod bin_utils;

use std::path::PathBuf;

use anyhow::Result;
use bin_utils::parser_dbc_file;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "dbcfmt", about = "Format DBC file", version)]
struct Opt {
    /// Input file encoding
    #[arg(short, long, default_value = "UTF-8")]
    encoding: String,

    /// Input dbc file
    input: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::parse();
    let network_ast = parser_dbc_file(&opt.input, &opt.encoding)?;
    let output_data = format!("{network_ast}");
    std::fs::write(opt.input, output_data)?;
    Ok(())
}
