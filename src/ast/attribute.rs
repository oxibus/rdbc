use super::common_parsers::*;
use super::error::DbcParseError;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::IResult;

pub fn parser_attribute_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    delimited(tag("\""), dbc_identifier, tag("\""))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_name_01() {
        assert_eq!(parser_attribute_name("\"hello\""), Ok(("", "hello")));
    }

    #[test]
    fn test_attribute_name_02() {
        assert_eq!(
            parser_attribute_name("\"RWEnvVar_wData_Val\""),
            Ok(("", "RWEnvVar_wData_Val"))
        );
    }
}
