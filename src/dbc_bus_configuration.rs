use super::dbc_common_parsers::*;
use super::dbc_error::DbcParseError;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

/// Bus configuration.
/// Format:: `BS_: <Speed>`
/// Speed in kBit/s
#[derive(PartialEq, Debug, Clone)]
pub struct DbcBusConfiguration(pub Option<f64>);

impl fmt::Display for DbcBusConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(speed) => write!(f, "BS_: {}", speed),
            None => write!(f, "BS_:"),
        }
    }
}

pub fn dbc_bus_configuration(
    input: &str,
) -> IResult<&str, Option<DbcBusConfiguration>, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("BS_")),
            spacey(tag(":")),
            opt(float_value),
            many0(line_ending),
        )),
        |(_, _, speed, _)| match speed {
            None => Some(DbcBusConfiguration(None)),
            Some(speed) => Some(DbcBusConfiguration(Some(speed))),
        },
    )(input);
    match res {
        Ok((remain, bus_config)) => {
            log::info!("parse bus config: {:?}", bus_config);
            Ok((remain, bus_config))
        }
        Err(e) => {
            log::trace!("parse bus config failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadBusConfig))
        }
    }
}

#[test]
fn test_dbc_bus_configuration() {
    assert_eq!(
        dbc_bus_configuration(
            r#"BS_: 12.34

"#
        ),
        Ok(("", Some(DbcBusConfiguration(Some(12.34))))),
    );

    assert_eq!(
        dbc_bus_configuration(
            r#"BS_:

"#
        ),
        Ok(("", Some(DbcBusConfiguration(None)))),
    );
}
