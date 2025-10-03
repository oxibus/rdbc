use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use super::common_parsers::dbc_identifier;
use super::error::DbcParseError;

pub fn parser_attribute_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    delimited(tag("\""), dbc_identifier, tag("\"")).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_name_01() {
        assert_eq!(parser_attribute_name(r#""hello""#), Ok(("", "hello")));
    }

    #[test]
    fn test_attribute_name_02() {
        assert_eq!(
            parser_attribute_name(r#""RWEnvVar_wData_Val""#),
            Ok(("", "RWEnvVar_wData_Val"))
        );
    }
}
