use super::dbc_common_parsers::*;
use super::dbc_error::DbcParseError;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
use std::fmt;

/// Version identifier of the DBC file.
///
/// Format: `VERSION "<VersionIdentifier>"`
#[derive(PartialEq, Debug, Clone)]
pub struct Version(pub String);

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VERSION \"{}\"", self.0)
    }
}

pub fn parser_version(input: &str) -> IResult<&str, Version, DbcParseError> {
    let res = map(preceded(spacey(tag("VERSION")), string_literal), |s| {
        Version(s)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbc_version_01() {
        assert_eq!(
            parser_version("VERSION \"1.0.0\""),
            Ok(("", Version("1.0.0".into())))
        );
    }

    #[test]
    fn test_dbc_version_02() {
        assert_eq!(
            parser_version("VERSION  \"3.0.1\""),
            Ok(("", Version("3.0.1".into())))
        );
    }

    #[test]
    fn test_dbc_version_03() {
        assert_eq!(
            parser_version("VERSION        \"9\""),
            Ok(("", Version("9".into())))
        );
    }
}
