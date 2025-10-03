use anyhow::Result;
use rrdbc::ast::network_ast::dbc_value;

fn main() -> Result<()> {
    env_logger::init();

    let data = std::fs::read_to_string("dbc/mytest/a.dbc")?;
    let (remain, onedbc) = dbc_value(&data)?;
    log::info!("remain: {remain}");
    log::info!("onedbc: {onedbc}");
    Ok(())
}
