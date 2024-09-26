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

#[derive(PartialEq, Debug, Clone)]
pub struct BitTimingValue {
    // Baudrate
    pub baudrate: f64,
    // bit timing register 1
    pub btr1: f64,
    // bit timing register 2
    pub btr2: f64,
}

impl fmt::Display for BitTimingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.baudrate, self.btr1, self.btr2)
    }
}

/// The bit timing section defines the baudrate and the settings of the BTR registers of
/// the network. This section is obsolete and not used any more. Nevertheless he
/// keyword 'BS_' must appear in the DBC file.
///
/// Format:: `bit_timing = 'BS_:' [baudrate ':' BTR1 ',' BTR2 ] ;`
#[derive(PartialEq, Debug, Clone)]
pub struct BitTiming {
    pub value: Option<BitTimingValue>,
}

impl fmt::Display for BitTiming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(value) => write!(f, "BS_: {}\n", value),
            None => write!(f, "BS_:\n"),
        }
    }
}

pub fn parser_bit_timing_value(input: &str) -> IResult<&str, BitTimingValue, DbcParseError> {
    let res: Result<(&str, BitTimingValue), nom::Err<DbcParseError>> = map(
        tuple((
            spacey(number_value),
            spacey(tag(":")),
            spacey(number_value),
            spacey(tag(":")),
            spacey(number_value),
        )),
        |(baudrate, _, btr1, _, btr2)| BitTimingValue {
            baudrate,
            btr1,
            btr2,
        },
    )(input);
    match res {
        Ok((remain, bit_timing)) => {
            log::info!("parse bit timing value: {:?}", bit_timing);
            Ok((remain, bit_timing))
        }
        Err(e) => {
            log::trace!("parse bit timing value failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadBitTimingValue))
        }
    }
}

pub fn parser_bit_timing(input: &str) -> IResult<&str, Option<BitTiming>, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("BS_")),
            spacey(tag(":")),
            spacey(opt(parser_bit_timing_value)),
            spacey(opt(tag(";"))),
            many0(line_ending),
        )),
        |(_, _, value, _, _)| match value {
            None => Some(BitTiming { value: None }),
            Some(value) => Some(BitTiming { value: Some(value) }),
        },
    )(input);
    match res {
        Ok((remain, bit_timing)) => {
            log::info!("parse bit timing: {:?}", bit_timing);
            Ok((remain, bit_timing))
        }
        Err(e) => {
            log::trace!("parse bit timing failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadBitTiming))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_bit_timing_01() {
        let ret = parser_bit_timing(
            r#"BS_: 12.34:123:456

"#,
        );
        match ret {
            Ok((_remain, bus_config)) => {
                assert_eq!(
                    bus_config,
                    Some(BitTiming {
                        value: Some(BitTimingValue {
                            baudrate: 12.34,
                            btr1: 123f64,
                            btr2: 456f64,
                        })
                    })
                );
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_parser_bit_timing_02() {
        let ret = parser_bit_timing(
            r#"BS_:

"#,
        );
        match ret {
            Ok((_remain, bus_config)) => {
                assert_eq!(bus_config, Some(BitTiming { value: None }));
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_parser_bit_timing_03() {
        let ret = parser_bit_timing(
            r#"BS_: ;

"#,
        );
        match ret {
            Ok((_remain, bus_config)) => {
                assert_eq!(bus_config, Some(BitTiming { value: None }));
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_parser_bit_timing_04() {
        let ret = parser_bit_timing(
            r#"BS_: 12.34:123:456 ;

"#,
        );
        match ret {
            Ok((_remain, bus_config)) => {
                assert_eq!(
                    bus_config,
                    Some(BitTiming {
                        value: Some(BitTimingValue {
                            baudrate: 12.34,
                            btr1: 123f64,
                            btr2: 456f64,
                        })
                    })
                );
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }
}
