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
use nom::Parser;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CharString(pub String);

impl fmt::Display for CharString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chars = self.0.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.peek() {
                    Some('\\') => {
                        f.write_str("\\")?;
                        chars.next();
                    }
                    _ => {
                        f.write_str("\\")?;
                    }
                }
            } else {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

pub fn parser_char_string(input: &str) -> IResult<&str, CharString, DbcParseError> {
    let res = string_literal(input)?;
    Ok((res.0, CharString(res.1)))
}

pub fn printable_character(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(satisfy(|c| {
        let c = c as u32;
        (0x20..0x75).contains(&c)
    }))
    .parse(input)
}

pub fn nonescaped_string(input: &str) -> IResult<&str, String, DbcParseError> {
    let parsred = recognize(none_of("\"\\")).parse(input)?;
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
    ))
    .parse(input)?;

    Ok((parsred.0, parsred.1.to_string()))
}

fn parse_backslash(input: &str) -> IResult<&str, String, DbcParseError> {
    let parsed = tag("\\").parse(input)?;
    Ok((parsed.0, parsed.1.to_string()))
}

fn parse_char(input: &str) -> IResult<&str, String, DbcParseError> {
    let parsed = anychar(input)?;
    Ok((parsed.0, parsed.1.to_string()))
}

pub fn escape_code_02(input: &str) -> IResult<&str, String, DbcParseError> {
    map(pair(parse_backslash, parse_char), |(_, c)| {
        format!("\\\\{c}")
    })
    .parse(input)
}

pub fn string_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(many0(alt((nonescaped_string, escape_code, escape_code_02)))).parse(input)
}

pub fn string_literal(input: &str) -> IResult<&str, String, DbcParseError> {
    let res = delimited(tag("\""), string_body, tag("\"")).parse(input);

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
    fn test_char_string_to_string_01() {
        assert_eq!(
            CharString("hello".to_string()).to_string(),
            "hello".to_string()
        );
    }

    #[test]
    fn test_char_string_to_string_02() {
        assert_eq!(
            CharString("hello\\Iworld".to_string()).to_string(),
            r#"hello\Iworld"#
        );
    }

    #[test]
    fn test_char_string_to_string_03() {
        assert_eq!(
            CharString("hello\nworld".to_string()).to_string(),
            r#"hello
world"#
        );
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

    #[test]
    fn test_char_string_03() {
        assert_eq!(
            char_string(r#""hello \I world""#),
            Ok(("", "hello \\I world".to_string()))
        );
    }
}
