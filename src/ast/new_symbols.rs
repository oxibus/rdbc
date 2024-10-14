use super::common_parsers::*;
use super::error::DbcParseError;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;

/// New Symbols, Names used throughout the DBC file.
///
/// Format:
///
/// ```text
/// NS_:
///     BS_
///     CM_
///     ...
/// ```
/// */
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NewSymbols(pub Vec<String>);

impl fmt::Display for NewSymbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "NS_:")?;
        for name in &self.0 {
            writeln!(f, "\t{name}")?;
        }
        Ok(())
    }
}

fn parser_one_line_new_symbols(input: &str) -> IResult<&str, String, DbcParseError> {
    map(
        tuple((space0, dbc_object_name, space0, line_ending)),
        |(_, name, _, _)| name.to_owned(),
    )(input)
}

pub fn parser_new_symbols(input: &str) -> IResult<&str, NewSymbols, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("NS_")),
            multispacey(tag(":")),
            multispacey(many0(parser_one_line_new_symbols)),
        )),
        |(_, _, names)| NewSymbols(names),
    )(input);
    match res {
        Ok((remain, names)) => {
            log::info!("parse names: {:?}", names.0);
            Ok((remain, names))
        }
        Err(e) => {
            log::trace!("parse names failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadNames))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_one_line_new_symbols_01() {
        let ret = parser_one_line_new_symbols(
            r#"  BS_
  "#,
        );
        match ret {
            Ok((_remain, name)) => {
                assert_eq!(name, "BS_".to_string());
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_parser_one_line_new_symbols_02() {
        let ret = parser_one_line_new_symbols(
            r#"  CM_
    "#,
        );
        match ret {
            Ok((_remain, name)) => {
                assert_eq!(name, "CM_".to_string());
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_parser_new_symbols_01() {
        let ret = parser_new_symbols(
            r#"NS_:
    BS_
    CM_


"#,
        );
        match ret {
            Ok((_remain, names)) => {
                assert_eq!(names, NewSymbols(vec!["BS_".into(), "CM_".into()]));
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_parser_new_symbols_02() {
        let ret = parser_new_symbols(
            r#"

NS_ :
    NS_DESC_
    CM_
    BA_DEF_
    BA_


"#,
        );

        match ret {
            Ok((_remain, names)) => {
                assert_eq!(
                    names,
                    NewSymbols(vec![
                        "NS_DESC_".into(),
                        "CM_".into(),
                        "BA_DEF_".into(),
                        "BA_".into()
                    ])
                );
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_new_symbol_string_01() {
        let names = NewSymbols(vec!["NS_DESC_".into(), "CM_".into()]);
        assert_eq!(format!("{}", names), "NS_:\n\tNS_DESC_\n\tCM_\n");
    }

    #[test]
    fn test_new_symbol_string_02() {
        let names = NewSymbols(vec![]);
        assert_eq!(format!("{}", names), "NS_:\n");
    }
}
