use super::dbc_common_parsers::*;
use super::dbc_error::DbcParseError;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
use std::fmt;

/// Version identifier of the DBC file.
/// Format: `VERSION "<VersionIdentifier>"`
#[derive(PartialEq, Debug, Clone)]
pub struct DbcVersion(pub String);

impl fmt::Display for DbcVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VERSION \"{}\"", self.0)
    }
}

pub fn dbc_version(input: &str) -> IResult<&str, DbcVersion, DbcParseError> {
    let res = map(preceded(spacey(tag("VERSION")), string_literal), |s| {
        DbcVersion(s)
    })(input);
    match res {
        Ok((remain, version)) => {
            log::info!("parse version: {}", version.0);
            Ok((remain, version))
        }
        Err(e) => {
            log::trace!("parse version failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadVersion))
        }
    }
}

#[test]
fn test_dbc_version() {
    assert_eq!(
        dbc_version("VERSION \"1.0.0\""),
        Ok(("", DbcVersion("1.0.0".into())))
    );

    assert_eq!(
        dbc_version("VERSION  \"3.0.1\""),
        Ok(("", DbcVersion("3.0.1".into())))
    );
}
