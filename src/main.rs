use anyhow::Result;
use rdbc::ast::{attribute_default, network_ast::*};

fn main() -> Result<()> {
    env_logger::init();

    let ret =
        attribute_default::parser_attribute_default(r#"BA_DEF_DEF_  "FloatAttribute" 25.25;"#);
    log::info!("ret: {:?}", ret);

    // let data = std::fs::read_to_string("dbc/mytest/a.dbc")?;
    // let (remain, onedbc) = dbc_value(&data)?;
    // log::info!("remain: {}", remain);
    // log::info!("onedbc: {}", onedbc);
    Ok(())
}
