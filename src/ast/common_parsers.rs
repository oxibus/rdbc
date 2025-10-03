use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::{
    alphanumeric1, digit0, digit1, i32, multispace0, one_of, satisfy, space0, u32,
};
use nom::combinator::{map, not, opt, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, pair};
use nom::{AsChar, IResult, Parser};

use super::error::DbcParseError;

pub fn spacey<I, O, E>(
    f: impl Parser<I, Output = O, Error = E>,
) -> impl Parser<I, Output = O, Error = E>
where
    I: nom::Input,
    <I as nom::Input>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(space0, f, space0)
}

pub fn multispacey<I, O, E>(
    f: impl Parser<I, Output = O, Error = E>,
) -> impl Parser<I, Output = O, Error = E>
where
    I: nom::Input,
    <I as nom::Input>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace0)
}

pub fn c_identifier(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize((
        alt((tag("_"), recognize(satisfy(|c| c.is_alpha())))),
        opt(recognize(many0(alt((tag("_"), alphanumeric1))))),
    ))
    .parse(input)
}

pub fn digit1to9(input: &str) -> IResult<&str, char, DbcParseError> {
    one_of("123456789").parse(input)
}

pub fn uint(input: &str) -> IResult<&str, &str, DbcParseError> {
    alt((tag("0"), recognize(pair(digit1to9, digit0)))).parse(input)
}

pub fn integer_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(pair(opt(tag("-")), uint)).parse(input)
}

pub fn integer_value(input: &str) -> IResult<&str, i64, DbcParseError> {
    let (remain, raw_int) = integer_body(input)?;
    match raw_int.parse::<i64>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => Err(nom::Err::Error(DbcParseError::BadInt)),
    }
}

pub fn frac(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(pair(tag("."), digit1)).parse(input)
}

pub fn exp(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize((tag_no_case("e"), opt(alt((tag("-"), tag("+")))), digit1)).parse(input)
}

pub fn float_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize((
        opt(tag("-")),
        uint,
        alt((recognize(pair(frac, opt(exp))), exp)),
    ))
    .parse(input)
}

pub fn float_value(input: &str) -> IResult<&str, f64, DbcParseError> {
    let (remain, raw_float) = float_body(input)?;
    match raw_float.parse::<f64>() {
        Ok(f) => Ok((remain, f)),
        Err(_) => Err(nom::Err::Error(DbcParseError::BadFloat)),
    }
}

pub fn number_value(input: &str) -> IResult<&str, f64, DbcParseError> {
    alt((map(float_value, |f| f), map(integer_value, |i| i as f64))).parse(input)
}

pub fn unsigned_integer(input: &str) -> IResult<&str, u32, DbcParseError> {
    u32.parse(input)
}

pub fn signed_integer(input: &str) -> IResult<&str, i32, DbcParseError> {
    i32.parse(input)
}

pub fn dbc_key_word(input: &str) -> IResult<&str, &str, DbcParseError> {
    alt((
        alt((
            tag("VERSION"),
            tag("NS_"),
            tag("NS_DESC_"),
            tag("CM_"),
            tag("BA_DEF_"),
            tag("BA_"),
            tag("VAL_"),
            tag("CAT_DEF_"),
            tag("CAT_"),
            tag("FILTER"),
        )),
        alt((
            tag("BA_DEF_DEF_"),
            tag("EV_DATA_"),
            tag("ENVVAR_DATA_"),
            tag("SGTYPE_"),
            tag("SGTYPE_VAL_"),
            tag("BA_DEF_SGTYPE_"),
            tag("BA_SGTYPE_"),
            tag("SIG_TYPE_REF_"),
            tag("VAL_TABLE_"),
            tag("SIG_GROUP_"),
        )),
        alt((
            tag("SIG_VALTYPE_"),
            tag("SIGTYPE_VALTYPE_"),
            tag("BO_TX_BU_"),
            tag("BA_DEF_REL_"),
            tag("BA_REL_"),
            tag("BA_DEF_DEF_REL_"),
            tag("BU_SG_REL_"),
            tag("BU_EV_REL_"),
            tag("BU_BO_REL_"),
            tag("SG_MUL_VAL_"),
        )),
        alt((
            tag("BS_"),
            tag("BU_"),
            tag("BO_"),
            tag("SG_"),
            tag("EV_"),
            tag("VECTOR__INDEPENDENT_SIG_MSG"),
            tag("VECTOR__XXX"),
        )),
    ))
    .parse(input)
}

pub fn dbc_object_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_').parse(input)
}

pub fn dbc_identifier_01(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize((
        alt((tag("_"), recognize(satisfy(|c| c.is_alpha())))),
        opt(recognize(many0(alt((tag("_"), tag("-"), alphanumeric1))))),
    ))
    .parse(input)
}

pub fn dbc_identifier(input: &str) -> IResult<&str, &str, DbcParseError> {
    let res = not(dbc_key_word).parse(input);
    match res {
        Ok((remain, _)) => dbc_identifier_01(remain),
        Err(_) => Err(nom::Err::Error(DbcParseError::UseKeywordAsIdentifier)),
    }
}

pub fn parser_node_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_identifier(input)
}

pub fn parser_message_id(input: &str) -> IResult<&str, u32, DbcParseError> {
    unsigned_integer(input)
}

pub fn parser_signal_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_identifier(input)
}

pub fn parser_env_var_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_identifier(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_value_01() {
        assert_eq!(float_value("0.0"), Ok(("", 0.0)));
    }

    #[test]
    fn test_float_value_02() {
        assert_eq!(float_value("1.52588e-05"), Ok(("", 1.52588e-05)));
    }

    #[test]
    fn test_float_value_03() {
        assert_eq!(float_value("1.52588E-05"), Ok(("", 1.52588e-05)));
    }

    #[test]
    fn test_c_identifier_01() {
        assert_eq!(c_identifier("a"), Ok(("", "a")));
    }

    #[test]
    fn test_c_identifier_02() {
        assert_eq!(c_identifier("abc"), Ok(("", "abc")));
    }

    #[test]
    fn test_c_identifier_03() {
        assert_eq!(c_identifier("hello_world"), Ok(("", "hello_world")));
    }

    #[test]
    fn test_c_identifier_04() {
        assert_eq!(c_identifier("_hello_world"), Ok(("", "_hello_world")));
    }

    #[test]
    fn test_c_identifier_05() {
        assert_eq!(
            c_identifier("_hello_world_123zzz"),
            Ok(("", "_hello_world_123zzz"))
        );
    }

    #[test]
    fn test_c_identifier_06() {
        assert_eq!(c_identifier("BigData"), Ok(("", "BigData")));
    }

    #[test]
    fn test_c_identifier_07() {
        assert_eq!(c_identifier("Big-Data"), Ok(("-Data", "Big")));
    }

    #[test]
    fn test_dbc_identifier_01() {
        assert_eq!(dbc_identifier("a"), Ok(("", "a")));
    }

    #[test]
    fn test_dbc_identifier_02() {
        assert_eq!(
            dbc_identifier("BS_"),
            Err(nom::Err::Error(DbcParseError::UseKeywordAsIdentifier))
        );
    }

    #[test]
    fn test_dbc_identifier_03() {
        assert_eq!(dbc_identifier("hello_world"), Ok(("", "hello_world")));
    }

    #[test]
    fn test_dbc_identifier_04() {
        assert_eq!(dbc_identifier("_HelloWorld"), Ok(("", "_HelloWorld")));
    }
}
