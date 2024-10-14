use anyhow::Result;
use rdbc::ast::network_ast::*;

fn main() -> Result<()> {
    env_logger::init();

    let data = std::fs::read_to_string("dbc/mytest/a.dbc")?;
    let (_remain, onedbc) = dbc_value(&data)?;
    let j = serde_json::to_string(&onedbc)?;
    println!("{}", j);
    Ok(())
}
