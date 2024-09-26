use super::dbc_error::DbcParseError;
use escape8259::unescape;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::digit0;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
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

pub fn is_nonescaped_string_char(c: char) -> bool {
    let cv = c as u32;
    (cv >= 0x20) && (cv != 0x22) && (cv != 0x5C)
}

// One or more unescaped text characters
pub fn nonescaped_string(input: &str) -> IResult<&str, &str, DbcParseError> {
    take_while1(is_nonescaped_string_char)(input)
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
    let (remain, raw_string) = delimited(tag("\""), string_body, tag("\""))(input)?;

    match unescape(raw_string) {
        Ok(s) => Ok((remain, s)),
        Err(_) => Err(nom::Err::Failure(DbcParseError::BadEscape)),
    }
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
        Err(_) => Err(nom::Err::Failure(DbcParseError::BadInt)),
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
        Err(_) => Err(nom::Err::Failure(DbcParseError::BadFloat)),
    }
}

pub fn number_value(input: &str) -> IResult<&str, f64, DbcParseError> {
    alt((
        map(float_value, |f| f.into()),
        map(integer_value, |i| i as f64),
    ))(input)
}

pub fn dbc_object_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

pub fn dbc_node_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_object_name(input)
}
