use anyhow::Result;
use rrdbc::ast::network_ast::NetworkAst;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "json2dbc", about = "Convert JSON to dbc file")]
struct Opt {
    /// Input json file
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    /// Output dbc file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();
    let input_data = std::fs::read_to_string(opt.input)?;
    let network_ast: NetworkAst = serde_json::from_str(&input_data)?;
    let output_data = format!("{}", network_ast);
    std::fs::write(opt.output, output_data)?;
    Ok(())
}
