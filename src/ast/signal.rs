use super::char_string::parser_char_string;
use super::char_string::CharString;
use super::common_parsers::*;
use super::error::DbcParseError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;

/*
 SG_ S7 m1 : 40|24@1- (1,0) [0|0] "" Vector__XXX
 SG_ S8 m2 : 40|8@1- (1,0) [0|0] "" Vector__XXX
 SG_ S6 M : 32|8@1- (1,0) [0|0] "" Vector__XXX
 SG_ S3 m0 : 16|16@1- (1,0) [0|0] "" Vector__XXX
 SG_ S2 m0 : 8|8@1- (1,0) [0|0] "" Vector__XXX
*/
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum DbcSignalMultiplexer {
    M,
    MultiplexerIdentifier(i64),
}

impl fmt::Display for DbcSignalMultiplexer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbcSignalMultiplexer::M => write!(f, "M"),
            DbcSignalMultiplexer::MultiplexerIdentifier(id) => write!(f, "m{id}"),
        }
    }
}

/// Endianness: 1 = little-endian, Intel; 0 = big-endian, Motorola
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

impl fmt::Display for ByteOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ByteOrder::LittleEndian => write!(f, "1"),
            ByteOrder::BigEndian => write!(f, "0"),
        }
    }
}

/// Signed: + = unsigned; - = signed
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    Signed,
    Unsigned,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Signed => write!(f, "-"),
            ValueType::Unsigned => write!(f, "+"),
        }
    }
}

/// The message's signal section lists all signals placed on the message, their position
/// in the message's data field and their properties.
///
/// ```text
/// signal = 'SG_' signal_name multiplexer_indicator ':' start_bit '|'
/// signal_size '@' byte_order value_type '(' factor ',' offset ')'
/// '[' minimum '|' maximum ']' unit receiver {',' receiver} ;
/// signal_name = DBC_identifier ;
/// multiplexer_indicator = ' ' | [m multiplexer_switch_value] [M] ;
/// start_bit = unsigned_integer ;
/// signal_size = unsigned_integer ;
/// byte_order = '0' | '1' ; (* 0=big endian, 1=little endian *)
/// value_type = '+' | '-' ; (* +=unsigned, -=signed *)
/// factor = double ;
/// offset = double ;
/// minimum = double ;
/// maximum = double ;
/// unit = char_string ;
/// receiver = node_name | 'Vector__XXX' ;
/// ```
///
/// Signal definition.
/// Format: `SG_ <SignalName> [M|m<MultiplexerIdentifier>] : <StartBit>|<Length>@<Endianness><Signed> (<Factor>,<Offset>) [<Min>|<Max>] "[Unit]" [ReceivingNodes]`
/// Length in bits.
/// Signed: + = unsigned; - = signed
/// Endianness: 1 = little-endian, Intel; 0 = big-endian, Motorola
/// M: If M than this signals contains a multiplexer identifier.
/// MultiplexerIdentifier: Signal definition is only used if the value of the multiplexer signal equals to this value.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub multiplexer: Option<DbcSignalMultiplexer>,
    pub start_bit: u32,
    pub size: u32,
    pub byte_order: ByteOrder,
    pub value_type: ValueType,
    pub factor: f64,
    pub offset: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub unit: Option<CharString>,
    pub receivers: Option<Vec<String>>,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let multiplexer = match &self.multiplexer {
            Some(m) => format!("{m} "),
            None => "".to_string(),
        };
        let value_type = match &self.value_type {
            ValueType::Signed => "-",
            ValueType::Unsigned => "+",
        };
        let byte_order = &self.byte_order.to_string();
        let min_max = match (&self.min, &self.max) {
            (Some(min), Some(max)) => format!("[{min}|{max}]"),
            _ => "".to_string(),
        };
        let unit = match &self.unit {
            Some(u) => format!("\"{u}\""),
            None => "".to_string(),
        };
        let mut receivers_str = String::new();
        if let Some(nodes) = &self.receivers {
            receivers_str = nodes.join(",");
        };

        write!(
            f,
            "SG_ {} {}: {}|{}@{}{} ({},{}) {} {} {}",
            self.name,
            multiplexer,
            self.start_bit,
            self.size,
            byte_order,
            value_type,
            self.factor,
            self.offset,
            min_max,
            unit,
            receivers_str
        )
    }
}

fn parser_signal_multiplexer(input: &str) -> IResult<&str, DbcSignalMultiplexer, DbcParseError> {
    alt((
        map(tag("M"), |_| DbcSignalMultiplexer::M),
        map(tuple((tag("m"), integer_value)), |(_, num)| {
            DbcSignalMultiplexer::MultiplexerIdentifier(num)
        }),
    ))(input)
}

fn parser_signal_start_bit(input: &str) -> IResult<&str, u32, DbcParseError> {
    unsigned_integer(input)
}

fn parser_signal_size(input: &str) -> IResult<&str, u32, DbcParseError> {
    unsigned_integer(input)
}

fn parser_signal_byte_order(input: &str) -> IResult<&str, ByteOrder, DbcParseError> {
    alt((
        map(tag("1"), |_| ByteOrder::LittleEndian),
        map(tag("0"), |_| ByteOrder::BigEndian),
    ))(input)
}

fn parser_signal_value_type(input: &str) -> IResult<&str, ValueType, DbcParseError> {
    alt((
        map(tag("+"), |_| ValueType::Unsigned),
        map(tag("-"), |_| ValueType::Signed),
    ))(input)
}

fn parser_signal_factor_offset(input: &str) -> IResult<&str, (f64, f64), DbcParseError> {
    let (remain, (factor, offset)) = delimited(
        spacey(tag("(")),
        separated_pair(number_value, spacey(tag(",")), number_value),
        spacey(tag(")")),
    )(input)?;

    Ok((remain, (factor, offset)))
}

fn parser_signal_min_max(input: &str) -> IResult<&str, (f64, f64), DbcParseError> {
    let (remain, (min_value, max_value)) = delimited(
        spacey(tag("[")),
        separated_pair(number_value, spacey(tag("|")), number_value),
        spacey(tag("]")),
    )(input)?;

    Ok((remain, (min_value, max_value)))
}

fn parser_signal_unit(input: &str) -> IResult<&str, CharString, DbcParseError> {
    parser_char_string(input)
}

fn parser_signal_receivers(input: &str) -> IResult<&str, Vec<String>, DbcParseError> {
    let (remain, nodes) = spacey(separated_list0(tag(","), spacey(parser_node_name)))(input)?;
    Ok((remain, nodes.into_iter().map(String::from).collect()))
}

pub fn parser_signal(input: &str) -> IResult<&str, Signal, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("SG_")),
            spacey(parser_signal_name),
            spacey(opt(parser_signal_multiplexer)),
            spacey(tag(":")),
            spacey(parser_signal_start_bit),
            spacey(tag("|")),
            spacey(parser_signal_size),
            spacey(tag("@")),
            spacey(parser_signal_byte_order),
            spacey(parser_signal_value_type),
            spacey(parser_signal_factor_offset),
            spacey(opt(parser_signal_min_max)),
            spacey(opt(parser_signal_unit)),
            spacey(opt(parser_signal_receivers)),
            many0(line_ending),
        )),
        |(
            _,
            name,
            multiplexer,
            _,
            start_bit,
            _,
            size,
            _,
            byte_order,
            value_type,
            factor_offset,
            min_max,
            unit,
            receiving_nodes,
            _,
        )| Signal {
            name: String::from(name),
            multiplexer,
            start_bit,
            size,
            byte_order,
            value_type,
            factor: factor_offset.0,
            offset: factor_offset.1,
            min: min_max.map(|(min, _)| min),
            max: min_max.map(|(_, max)| max),
            unit,
            receivers: receiving_nodes,
        },
    )(input);

    match res {
        Ok((remain, signal)) => {
            log::info!("parse signal: {:?}", signal);
            Ok((remain, signal))
        }
        Err(e) => {
            log::trace!("parse signal failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadSignal))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbc_signal_multiplexer_01() {
        assert_eq!(
            parser_signal_multiplexer(r#"M"#),
            Ok(("", DbcSignalMultiplexer::M)),
        );
    }

    #[test]
    fn test_dbc_signal_multiplexer_02() {
        assert_eq!(
            parser_signal_multiplexer(r#"m0"#),
            Ok(("", DbcSignalMultiplexer::MultiplexerIdentifier(0))),
        );
    }

    #[test]
    fn test_dbc_signal_multiplexer_03() {
        assert_eq!(
            parser_signal_multiplexer(r#"m123"#),
            Ok(("", DbcSignalMultiplexer::MultiplexerIdentifier(123))),
        );
    }

    #[test]
    fn test_dbc_signal_01() {
        let ret = parser_signal(
            r#" SG_ AY1 : 32|16@1+ (0.000127465,-4.1768) [-4.1768|4.1765] "g"  ABS

"#,
        );
        match ret {
            Ok((_remain, signal)) => {
                assert_eq!(
                    signal,
                    Signal {
                        name: "AY1".into(),
                        multiplexer: None,
                        start_bit: 32,
                        size: 16,
                        byte_order: ByteOrder::LittleEndian,
                        value_type: ValueType::Unsigned,
                        factor: 0.000127465,
                        offset: -4.1768,
                        min: Some(-4.1768),
                        max: Some(4.1765),
                        unit: Some(CharString("g".into())),
                        receivers: Some(vec!["ABS".into()]),
                    }
                );
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_dbc_signal_02() {
        let ret = parser_signal(
            r#" SG_ S2 m0 : 8|8@1- (1.0,0.0) [0.0|0.0] "" Vector__XXX

"#,
        );

        match ret {
            Ok((_remain, signal)) => {
                assert_eq!(
                    signal,
                    Signal {
                        name: "S2".into(),
                        multiplexer: Some(DbcSignalMultiplexer::MultiplexerIdentifier(0)),
                        start_bit: 8,
                        size: 8,
                        byte_order: ByteOrder::LittleEndian,
                        value_type: ValueType::Signed,
                        factor: 1.0,
                        offset: 0.0,
                        min: Some(0.0),
                        max: Some(0.0),
                        unit: Some(CharString("".into())),
                        receivers: Some(vec!["Vector__XXX".into()]),
                    }
                );
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_dbc_signal_03() {
        let ret = parser_signal(
            r#" SG_ S2 m0 : 8|8@1- (1,0) [0|0] "" Vector__XXX

"#,
        );

        match ret {
            Ok((_remain, signal)) => {
                assert_eq!(
                    signal,
                    Signal {
                        name: "S2".into(),
                        multiplexer: Some(DbcSignalMultiplexer::MultiplexerIdentifier(0)),
                        start_bit: 8,
                        size: 8,
                        byte_order: ByteOrder::LittleEndian,
                        value_type: ValueType::Signed,
                        factor: 1.0,
                        offset: 0.0,
                        min: Some(0.0),
                        max: Some(0.0),
                        unit: Some(CharString("".into())),
                        receivers: Some(vec!["Vector__XXX".into()]),
                    }
                );
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }

    #[test]
    fn test_dbc_signal_04() {
        let ret = parser_signal(
            r#"  SG_ Signal1 : 32|32@1+ (100,0) [0|100] "%"  Node1,Node2

"#,
        );

        match ret {
            Ok((_remain, signal)) => {
                assert_eq!(
                    signal,
                    Signal {
                        name: "Signal1".into(),
                        multiplexer: None,
                        start_bit: 32,
                        size: 32,
                        byte_order: ByteOrder::LittleEndian,
                        value_type: ValueType::Unsigned,
                        factor: 100.0,
                        offset: 0.0,
                        min: Some(0.0),
                        max: Some(100.0),
                        unit: Some(CharString("%".into())),
                        receivers: Some(vec!["Node1".into(), "Node2".into()]),
                    }
                );
            }
            Err(err) => panic!("err = {:?}", err),
        }
    }
}
