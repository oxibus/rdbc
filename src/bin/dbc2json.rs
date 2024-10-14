use anyhow::Result;
use rdbc::ast::network_ast::*;
use rdbc::file::read_file_content;

fn main() -> Result<()> {
    env_logger::init();

    let data = read_file_content("dbc/dbc_from_cantools/abs.dbc")?;
    let network_ast = parse_dbc(&data)?;
    let network_ast_json = serde_json::to_string(&network_ast)?;
    println!("{}", network_ast_json);

    Ok(())
}
