use super::dbc_common_parsers::*;
use super::dbc_error::DbcParseError;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

/// Names used throughout the DBC file.
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
#[derive(PartialEq, Debug, Clone)]
pub struct DbcNames(pub Vec<String>);

impl fmt::Display for DbcNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "NS_:")?;
        for name in &self.0 {
            writeln!(f, "\t{name}")?;
        }
        Ok(())
    }
}

fn dbc_one_line_name(input: &str) -> IResult<&str, String, DbcParseError> {
    map(
        tuple((space0, dbc_object_name, space0, line_ending)),
        |(_, name, _, _)| name.to_owned(),
    )(input)
}

pub fn dbc_names(input: &str) -> IResult<&str, DbcNames, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("NS_")),
            multispacey(tag(":")),
            multispacey(many0(dbc_one_line_name)),
        )),
        |(_, _, names)| DbcNames(names),
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
    fn test_dbc_one_line_name_01() {
        let ret = dbc_one_line_name(
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
    fn test_dbc_one_line_name_02() {
        let ret = dbc_one_line_name(
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
    fn test_dbc_names_01() {
        let ret = dbc_names(
            r#"NS_:
    BS_
    CM_


"#,
        );
        match ret {
            Ok((_remain, names)) => {
                assert_eq!(names, DbcNames(vec!["BS_".into(), "CM_".into()]));
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_dbc_names_02() {
        let ret = dbc_names(
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
                    DbcNames(vec![
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
}
