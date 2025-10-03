//! Common utility functions for binary tools.
//! This file is included directly in multiple binaries
//! because we do not want to make its functions public API.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use rrdbc::ast::network_ast::{parse_dbc, NetworkAst};
use rrdbc::encoding::to_utf8;

pub fn read_file_content<P: AsRef<Path>>(filename: P, encoding: &str) -> Result<String> {
    let data = if encoding.eq_ignore_ascii_case("utf-8") {
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

pub fn parser_dbc_file<P: AsRef<Path>>(filename: P, encoding: &str) -> Result<NetworkAst> {
    let data = read_file_content(filename, encoding)?;
    let network_ast = parse_dbc(&data)?;
    Ok(network_ast)
}
