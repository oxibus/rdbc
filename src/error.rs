use crate::ast::error::DbcParseError;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DbcError {
    #[error("error parsing dbc")]
    ParseError(DbcParseError),

    #[error("invalid encoding label")]
    InvalidEncodingLabel(String),
    #[error("encoding reading input error")]
    EncodingReadInputError,
    #[error("encoding writing output error")]
    EncodingWriteOutputError,
}
