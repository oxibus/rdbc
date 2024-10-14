use anyhow::Result;
use encoding_rs::GBK;
use encoding_rs::WINDOWS_1252;
use std::fs::File;
use std::io::Read;

pub fn try_read_file_content_with_utf8(filename: &str) -> Result<String> {
    let data = std::fs::read_to_string(filename)?;
    Ok(data)
}

pub fn try_read_file_content_with_gbk(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let (cow, _, had_errors) = GBK.decode(&buffer);
    if had_errors {
        return Err(anyhow::anyhow!("GBK decode had errors"));
    }

    Ok(cow.to_string())
}

pub fn try_read_file_content_with_window_1252(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let (cow, _, had_errors) = WINDOWS_1252.decode(&buffer);
    if had_errors {
        return Err(anyhow::anyhow!("WINDOWS_1252 decode had errors"));
    }

    Ok(cow.to_string())
}

pub fn read_file_content(filename: &str) -> Result<String> {
    let data = try_read_file_content_with_utf8(filename);
    if data.is_ok() {
        return data;
    }

    let data = try_read_file_content_with_gbk(filename);
    if data.is_ok() {
        return data;
    }

    let data = try_read_file_content_with_window_1252(filename);
    if data.is_ok() {
        return data;
    }

    Err(anyhow::anyhow!(
        "read file content failed, file encoding is not supported"
    ))
}
