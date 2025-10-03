use std::fs::File;
use std::io::Read;

use anyhow::Result;

use crate::ast::network_ast::{parse_dbc, NetworkAst};
use crate::encoding::to_utf8;

pub fn read_file_content(filename: &str, encoding: &str) -> Result<String> {
    let data = if encoding.to_lowercase() == "utf-8" {
        std::fs::read_to_string(filename)?
    } else {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let data = to_utf8(encoding, &buffer)?;
        String::from_utf8(data)?
    };

    Ok(data)
}

pub fn parser_dbc_file(filename: &str, encoding: &str) -> Result<NetworkAst> {
    let data = read_file_content(filename, encoding)?;
    let network_ast = parse_dbc(&data)?;
    Ok(network_ast)
}
