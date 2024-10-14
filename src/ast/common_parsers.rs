use super::error::DbcParseError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit0;
use nom::character::complete::digit1;
use nom::character::complete::i32;
use nom::character::complete::multispace0;
use nom::character::complete::none_of;
use nom::character::complete::one_of;
use nom::character::complete::satisfy;
use nom::character::complete::space0;
use nom::character::complete::u32;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::AsChar;
use nom::IResult;

pub fn spacey<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(space0, f, space0)
}

pub fn multispacey<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace0)
}

pub fn printable_character(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(satisfy(|c| {
        let c = c as u32;
        c >= 0x20 && c <= 0x74
    }))(input)
}

pub fn nonescaped_string(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(none_of("\"\\"))(input)
}

pub fn escape_code(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(pair(
        tag("\\"),
        alt((
            tag("\""),
            tag("\\"),
            tag("/"),
            tag("b"),
            tag("f"),
            tag("n"),
            tag("r"),
            tag("t"),
            tag("u"),
        )),
    ))(input)
}

pub fn string_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(many0(alt((nonescaped_string, escape_code))))(input)
}

pub fn string_literal(input: &str) -> IResult<&str, String, DbcParseError> {
    let res = delimited(tag("\""), string_body, tag("\""))(input);

    match res {
        Ok((remain, s)) => Ok((remain, s.to_string())),
        Err(_) => Err(nom::Err::Error(DbcParseError::BadEscape)),
    }
}

pub fn char_string(input: &str) -> IResult<&str, String, DbcParseError> {
    string_literal(input)
}

pub fn c_identifier(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(tuple((
        alt((tag("_"), recognize(satisfy(|c| c.is_alpha())))),
        opt(recognize(many0(alt((tag("_"), alphanumeric1))))),
    )))(input)
}

pub fn digit1to9(input: &str) -> IResult<&str, char, DbcParseError> {
    one_of("123456789")(input)
}

pub fn uint(input: &str) -> IResult<&str, &str, DbcParseError> {
    alt((tag("0"), recognize(pair(digit1to9, digit0))))(input)
}

pub fn integer_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(pair(opt(tag("-")), uint))(input)
}

pub fn integer_value(input: &str) -> IResult<&str, i64, DbcParseError> {
    let (remain, raw_int) = integer_body(input)?;
    match raw_int.parse::<i64>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => Err(nom::Err::Error(DbcParseError::BadInt)),
    }
}

pub fn frac(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(pair(tag("."), digit1))(input)
}

pub fn exp(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(tuple((tag("e"), opt(alt((tag("-"), tag("+")))), digit1)))(input)
}

pub fn float_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(tuple((
        opt(tag("-")),
        uint,
        alt((recognize(pair(frac, opt(exp))), exp)),
    )))(input)
}

pub fn float_value(input: &str) -> IResult<&str, f64, DbcParseError> {
    let (remain, raw_float) = float_body(input)?;
    match raw_float.parse::<f64>() {
        Ok(f) => Ok((remain, f)),
        Err(_) => Err(nom::Err::Error(DbcParseError::BadFloat)),
    }
}

pub fn number_value(input: &str) -> IResult<&str, f64, DbcParseError> {
    alt((
        map(float_value, |f| f.into()),
        map(integer_value, |i| i as f64),
    ))(input)
}

pub fn unsigned_integer(input: &str) -> IResult<&str, u32, DbcParseError> {
    u32(input)
}

pub fn signed_integer(input: &str) -> IResult<&str, i32, DbcParseError> {
    i32(input)
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
    ))(input)
}

pub fn dbc_object_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

pub fn dbc_identifier(input: &str) -> IResult<&str, &str, DbcParseError> {
    let res = not(dbc_key_word)(input);
    match res {
        Ok((remain, _)) => c_identifier(remain),
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

    #[test]
    fn test_char_string_01() {
        assert_eq!(char_string("\"hello\""), Ok(("", "hello".to_string())));
    }

    #[test]
    fn test_char_string_02() {
        assert_eq!(
            char_string(
                r#""hello
world""#
            ),
            Ok(("", "hello\nworld".to_string()))
        );
    }
}
