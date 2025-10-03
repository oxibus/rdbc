use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use rrdbc::ast::network_ast::NetworkAst;

#[derive(Debug, Parser)]
#[command(name = "json2dbc", about = "Convert JSON to dbc file", version)]
struct Opt {
    /// Input json file
    input: PathBuf,

    /// Output dbc file
    output: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::parse();
    let input_data = std::fs::read_to_string(opt.input)?;
    let network_ast: NetworkAst = serde_json::from_str(&input_data)?;
    let output_data = format!("{network_ast}");
    std::fs::write(opt.output, output_data)?;
    Ok(())
}
