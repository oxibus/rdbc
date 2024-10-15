use super::error::DbcParseError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::none_of;
use nom::character::complete::satisfy;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::IResult;

pub fn printable_character(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(satisfy(|c| {
        let c = c as u32;
        c >= 0x20 && c <= 0x74
    }))(input)
}

pub fn nonescaped_string(input: &str) -> IResult<&str, String, DbcParseError> {
    let parsred = recognize(none_of("\"\\"))(input)?;
    Ok((parsred.0, parsred.1.to_string()))
}

pub fn escape_code(input: &str) -> IResult<&str, String, DbcParseError> {
    let parsred = recognize(pair(
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
    ))(input)?;

    Ok((parsred.0, parsred.1.to_string()))
}

fn parse_backslash(input: &str) -> IResult<&str, String, DbcParseError> {
    let parsed = tag("\\")(input)?;
    Ok((parsed.0, parsed.1.to_string()))
}

fn parse_char(input: &str) -> IResult<&str, String, DbcParseError> {
    let parsed = anychar(input)?;
    Ok((parsed.0, parsed.1.to_string()))
}

pub fn escape_code_02(input: &str) -> IResult<&str, String, DbcParseError> {
    map(pair(parse_backslash, parse_char), |(_, c)| {
        format!("\\\\{c}")
    })(input)
}

pub fn string_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(many0(alt((nonescaped_string, escape_code, escape_code_02))))(input)
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_char_string_03() {
        assert_eq!(
            char_string(r#""hello \I world""#),
            Ok(("", "hello \\I world".to_string()))
        );
    }
}
