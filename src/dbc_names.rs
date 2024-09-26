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
            many0(dbc_one_line_name),
            many0(line_ending),
        )),
        |(_, _, names, _)| DbcNames(names),
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

#[test]
fn test_dbc_one_line_name() {
    assert_eq!(
        dbc_one_line_name(
            r#"  BS_
"#
        ),
        Ok(("", "BS_".into())),
    );

    assert_eq!(
        dbc_one_line_name(
            r#"    CM_
"#
        ),
        Ok(("", "CM_".into())),
    );
}

#[test]
fn test_dbc_names_01() {
    assert_eq!(
        dbc_names(
            r#"NS_:
    BS_
    CM_


"#
        ),
        Ok(("", DbcNames(vec!["BS_".into(), "CM_".into()]))),
    );
}

#[test]
fn test_dbc_names_02() {
    assert_eq!(
        dbc_names(
            r#"

NS_ :
	NS_DESC_
	CM_
	BA_DEF_
	BA_


"#
        ),
        Ok((
            "",
            DbcNames(vec![
                "NS_DESC_".into(),
                "CM_".into(),
                "BA_DEF_".into(),
                "BA_".into()
            ])
        )),
    );
}
