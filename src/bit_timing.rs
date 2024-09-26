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

/// The bit timing section defines the baudrate and the settings of the BTR registers of
/// the network. This section is obsolete and not used any more. Nevertheless he
/// keyword 'BS_' must appear in the DBC file.
///
/// Format:: `bit_timing = 'BS_:' [baudrate ':' BTR1 ',' BTR2 ] ;`
#[derive(PartialEq, Debug, Clone)]
pub struct BitTiming(pub Option<f64>);

impl fmt::Display for BitTiming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(speed) => write!(f, "BS_: {}", speed),
            None => write!(f, "BS_:"),
        }
    }
}

pub fn parser_bit_timing(input: &str) -> IResult<&str, Option<BitTiming>, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("BS_")),
            spacey(tag(":")),
            opt(float_value),
            many0(line_ending),
        )),
        |(_, _, speed, _)| match speed {
            None => Some(BitTiming(None)),
            Some(speed) => Some(BitTiming(Some(speed))),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_bit_timing_01() {
        let ret = parser_bit_timing(
            r#"BS_: 12.34

"#,
        );
        match ret {
            Ok((_remain, bus_config)) => {
                assert_eq!(bus_config, Some(BitTiming(Some(12.34))));
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
                assert_eq!(bus_config, Some(BitTiming(None)));
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }
}
