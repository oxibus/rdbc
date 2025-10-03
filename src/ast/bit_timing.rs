use std::fmt;

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u64};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

use super::common_parsers::*;
use super::error::DbcParseError;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BitTimingValue {
    // Baudrate
    pub baudrate: u64,
    // bit timing register 1
    pub btr1: u64,
    // bit timing register 2
    pub btr2: u64,
}

impl fmt::Display for BitTimingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.baudrate, self.btr1, self.btr2)
    }
}

/// The bit timing section defines the baudrate and the settings of the BTR registers of
/// the network. This section is obsolete and not used any more. Nevertheless, the
/// keyword `BS_` must appear in the DBC file.
///
/// Format:: `bit_timing = 'BS_:' [baudrate ':' BTR1 ',' BTR2 ] ;`
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BitTiming {
    pub value: Option<BitTimingValue>,
}

impl fmt::Display for BitTiming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(value) => writeln!(f, "BS_: {}", value),
            None => writeln!(f, "BS_:"),
        }
    }
}

pub fn parser_bit_timing_value(input: &str) -> IResult<&str, BitTimingValue, DbcParseError> {
    let res: Result<(&str, BitTimingValue), nom::Err<DbcParseError>> = map(
        (
            spacey(u64),
            spacey(tag(":")),
            spacey(u64),
            spacey(tag(":")),
            spacey(u64),
        ),
        |(baudrate, _, btr1, _, btr2)| BitTimingValue {
            baudrate,
            btr1,
            btr2,
        },
    )
    .parse(input);
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
        (
            multispacey(tag("BS_")),
            spacey(tag(":")),
            spacey(opt(parser_bit_timing_value)),
            spacey(opt(tag(";"))),
            many0(line_ending),
        ),
        |(_, _, value, _, _)| match value {
            None => Some(BitTiming { value: None }),
            Some(value) => Some(BitTiming { value: Some(value) }),
        },
    )
    .parse(input);
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
            r#"BS_: 12:123:456

"#,
        );
        match ret {
            Ok((_remain, bus_config)) => {
                assert_eq!(
                    bus_config,
                    Some(BitTiming {
                        value: Some(BitTimingValue {
                            baudrate: 12,
                            btr1: 123,
                            btr2: 456,
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
            r#"BS_: 12:123:456 ;

"#,
        );
        match ret {
            Ok((_remain, bus_config)) => {
                assert_eq!(
                    bus_config,
                    Some(BitTiming {
                        value: Some(BitTimingValue {
                            baudrate: 12,
                            btr1: 123,
                            btr2: 456,
                        })
                    })
                );
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_bit_timing_string_01() {
        let bit_timing = BitTiming {
            value: Some(BitTimingValue {
                baudrate: 12,
                btr1: 123,
                btr2: 456,
            }),
        };
        assert_eq!(bit_timing.to_string(), "BS_: 12:123:456\n");
    }

    #[test]
    fn test_bit_timing_string_02() {
        let bit_timing = BitTiming { value: None };
        assert_eq!(bit_timing.to_string(), "BS_:\n");
    }
}
