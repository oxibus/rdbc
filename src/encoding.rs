// https://github.com/hsivonen/recode_rs

use std::io::{Read, Write};
use std::str::from_utf8_mut;

use encoding_rs::*;

use crate::error::DbcError;

pub fn utf8_to_gbk(src_data: &[u8]) -> Result<Vec<u8>, DbcError> {
    recode(src_data, "UTF-8", "GBK")
}

pub fn gbk_to_utf8(src_data: &[u8]) -> Result<Vec<u8>, DbcError> {
    recode(src_data, "GBK", "UTF-8")
}

pub fn to_utf8(src_encoding_label: &str, src_data: &[u8]) -> Result<Vec<u8>, DbcError> {
    recode(src_data, src_encoding_label, "UTF-8")
}

pub fn recode(
    src_data: &[u8],
    src_encoding_label: &str,
    dst_encoding_label: &str,
) -> Result<Vec<u8>, DbcError> {
    let src_encoding = get_encoding(Some(src_encoding_label.to_string()))?;
    let dst_encoding = get_encoding(Some(dst_encoding_label.to_string()))?;

    let mut decoder = src_encoding.new_decoder();
    let mut encoder = dst_encoding.new_encoder();

    let mut buf = std::io::Cursor::new(Vec::new());

    convert_via_utf8(
        &mut decoder,
        &mut encoder,
        &mut std::io::Cursor::new(src_data),
        &mut buf,
        true,
    )?;

    Ok(buf.into_inner())
}

pub fn get_encoding(opt: Option<String>) -> Result<&'static Encoding, DbcError> {
    match opt {
        None => Ok(UTF_8),
        Some(label) => match Encoding::for_label(label.as_bytes()) {
            None => Err(DbcError::InvalidEncodingLabel(label)),
            Some(encoding) => Ok(encoding),
        },
    }
}

pub fn convert_via_utf8(
    decoder: &mut Decoder,
    encoder: &mut Encoder,
    read: &mut dyn Read,
    write: &mut dyn Write,
    last: bool,
) -> Result<(), DbcError> {
    let mut input_buffer = [0u8; 2048];
    let mut intermediate_buffer_bytes = [0u8; 4096];
    let intermediate_buffer: &mut str = from_utf8_mut(&mut intermediate_buffer_bytes[..]).unwrap();
    let mut output_buffer = [0u8; 4096];
    let mut current_input_ended = false;
    while !current_input_ended {
        match read.read(&mut input_buffer) {
            Err(e) => {
                log::error!("Error reading input, error = {}", e);
                return Err(DbcError::EncodingReadInputError);
            }
            Ok(decoder_input_end) => {
                current_input_ended = decoder_input_end == 0;
                let input_ended = last && current_input_ended;
                let mut decoder_input_start = 0usize;
                loop {
                    let (decoder_result, decoder_read, decoder_written, _) = decoder.decode_to_str(
                        &input_buffer[decoder_input_start..decoder_input_end],
                        intermediate_buffer,
                        input_ended,
                    );
                    decoder_input_start += decoder_read;

                    let last_output = if input_ended {
                        match decoder_result {
                            CoderResult::InputEmpty => true,
                            CoderResult::OutputFull => false,
                        }
                    } else {
                        false
                    };

                    // Regardless of whether the intermediate buffer got full
                    // or the input buffer was exhausted, let's process what's
                    // in the intermediate buffer.

                    if encoder.encoding() == UTF_8 {
                        // If the target is UTF-8, optimize out the encoder.
                        if let Err(e) =
                            write.write_all(&intermediate_buffer.as_bytes()[..decoder_written])
                        {
                            log::error!("Error writing output, error = {}", e);
                            return Err(DbcError::EncodingWriteOutputError);
                        }
                    } else {
                        let mut encoder_input_start = 0usize;
                        loop {
                            let (encoder_result, encoder_read, encoder_written, _) = encoder
                                .encode_from_utf8(
                                    &intermediate_buffer[encoder_input_start..decoder_written],
                                    &mut output_buffer,
                                    last_output,
                                );
                            encoder_input_start += encoder_read;
                            if let Err(e) = write.write_all(&output_buffer[..encoder_written]) {
                                log::error!("Error writing output, error = {}", e);
                                return Err(DbcError::EncodingWriteOutputError);
                            }
                            match encoder_result {
                                CoderResult::InputEmpty => {
                                    break;
                                }
                                CoderResult::OutputFull => {
                                    continue;
                                }
                            }
                        }
                    }

                    // Now let's see if we should read again or process the
                    // rest of the current input buffer.
                    match decoder_result {
                        CoderResult::InputEmpty => {
                            break;
                        }
                        CoderResult::OutputFull => {
                            continue;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
