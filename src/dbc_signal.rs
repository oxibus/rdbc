use super::dbc_common_parsers::*;
use super::dbc_error::DbcParseError;
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
use std::fmt;

/*
 SG_ S7 m1 : 40|24@1- (1,0) [0|0] "" Vector__XXX
 SG_ S8 m2 : 40|8@1- (1,0) [0|0] "" Vector__XXX
 SG_ S6 M : 32|8@1- (1,0) [0|0] "" Vector__XXX
 SG_ S3 m0 : 16|16@1- (1,0) [0|0] "" Vector__XXX
 SG_ S2 m0 : 8|8@1- (1,0) [0|0] "" Vector__XXX
*/
#[derive(PartialEq, Debug, Clone)]
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
#[derive(PartialEq, Debug, Clone)]
pub enum DbcSignalEndianness {
    LittleEndian,
    BigEndian,
}

impl fmt::Display for DbcSignalEndianness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbcSignalEndianness::LittleEndian => write!(f, "1"),
            DbcSignalEndianness::BigEndian => write!(f, "0"),
        }
    }
}

/// Signed: + = unsigned; - = signed
#[derive(PartialEq, Debug, Clone)]
pub enum DbcSignalSigned {
    Signed,
    Unsigned,
}

impl fmt::Display for DbcSignalSigned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbcSignalSigned::Signed => write!(f, "-"),
            DbcSignalSigned::Unsigned => write!(f, "+"),
        }
    }
}

/// Signal definition.
/// Format: `SG_ <SignalName> [M|m<MultiplexerIdentifier>] : <StartBit>|<Length>@<Endianness><Signed> (<Factor>,<Offset>) [<Min>|<Max>] "[Unit]" [ReceivingNodes]`
/// Length in bits.
/// Signed: + = unsigned; - = signed
/// Endianness: 1 = little-endian, Intel; 0 = big-endian, Motorola
/// M: If M than this signals contains a multiplexer identifier.
/// MultiplexerIdentifier: Signal definition is only used if the value of the multiplexer signal equals to this value.
#[derive(PartialEq, Debug, Clone)]
pub struct DbcSignal {
    pub name: String,
    pub multiplexer: Option<DbcSignalMultiplexer>,
    pub start_bit: i64,
    pub length: i64,
    pub endianness: DbcSignalEndianness,
    pub signed: DbcSignalSigned,
    pub factor: f64,
    pub offset: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub unit: Option<String>,
    pub receiving_nodes: Option<Vec<String>>,
}

impl fmt::Display for DbcSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let multiplexer = match &self.multiplexer {
            Some(m) => format!("{m} "),
            None => "".to_string(),
        };
        let signed = match &self.signed {
            DbcSignalSigned::Signed => "-",
            DbcSignalSigned::Unsigned => "+",
        };
        let endianness = &self.endianness.to_string();
        let min_max = match (&self.min, &self.max) {
            (Some(min), Some(max)) => format!("[{min}|{max}]"),
            _ => "".to_string(),
        };
        let unit = match &self.unit {
            Some(u) => format!("\"{u}\""),
            None => "".to_string(),
        };
        let mut receiving_nodes_str = String::new();
        if let Some(nodes) = &self.receiving_nodes {
            receiving_nodes_str = nodes.join(",");
        };

        write!(
            f,
            "SG_ {} {}: {}|{}@{}{} ({},{}) {} {} {}",
            self.name,
            multiplexer,
            self.start_bit,
            self.length,
            endianness,
            signed,
            self.factor,
            self.offset,
            min_max,
            unit,
            receiving_nodes_str
        )
    }
}

fn dbc_signal_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_object_name(input)
}

fn dbc_signal_multiplexer(input: &str) -> IResult<&str, DbcSignalMultiplexer, DbcParseError> {
    alt((
        map(tag("M"), |_| DbcSignalMultiplexer::M),
        map(tuple((tag("m"), integer_value)), |(_, num)| {
            DbcSignalMultiplexer::MultiplexerIdentifier(num)
        }),
    ))(input)
}

fn dbc_signal_endianness(input: &str) -> IResult<&str, DbcSignalEndianness, DbcParseError> {
    alt((
        map(tag("1"), |_| DbcSignalEndianness::LittleEndian),
        map(tag("0"), |_| DbcSignalEndianness::BigEndian),
    ))(input)
}

fn dbc_signal_signed(input: &str) -> IResult<&str, DbcSignalSigned, DbcParseError> {
    alt((
        map(tag("+"), |_| DbcSignalSigned::Unsigned),
        map(tag("-"), |_| DbcSignalSigned::Signed),
    ))(input)
}

fn dbc_signal_factor_offset(input: &str) -> IResult<&str, (f64, f64), DbcParseError> {
    let (remain, (factor, offset)) = delimited(
        spacey(tag("(")),
        separated_pair(number_value, spacey(tag(",")), number_value),
        spacey(tag(")")),
    )(input)?;

    Ok((remain, (factor, offset)))
}

fn dbc_signal_min_max(input: &str) -> IResult<&str, (f64, f64), DbcParseError> {
    let (remain, (min_value, max_value)) = delimited(
        spacey(tag("[")),
        separated_pair(number_value, spacey(tag("|")), number_value),
        spacey(tag("]")),
    )(input)?;

    Ok((remain, (min_value, max_value)))
}

fn dbc_signal_receiving_nodes(input: &str) -> IResult<&str, Vec<String>, DbcParseError> {
    let (remain, nodes) = spacey(separated_list0(tag(","), spacey(dbc_node_name)))(input)?;
    Ok((remain, nodes.into_iter().map(String::from).collect()))
}

pub fn dbc_signal(input: &str) -> IResult<&str, DbcSignal, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("SG_")),
            spacey(dbc_signal_name),
            spacey(opt(dbc_signal_multiplexer)),
            spacey(tag(":")),
            spacey(integer_value), // start bit
            tag("|"),
            integer_value, // length
            tag("@"),
            dbc_signal_endianness,
            spacey(dbc_signal_signed),
            spacey(dbc_signal_factor_offset),
            spacey(opt(dbc_signal_min_max)),
            spacey(opt(string_literal)), // "[Unit]"
            spacey(opt(dbc_signal_receiving_nodes)),
            many0(line_ending),
        )),
        |(
            _,
            name,
            multiplexer,
            _,
            start_bit,
            _,
            length,
            _,
            endianness,
            signed,
            factor_offset,
            min_max,
            unit,
            receiving_nodes,
            _,
        )| DbcSignal {
            name: String::from(name),
            multiplexer,
            start_bit,
            length,
            endianness,
            signed,
            factor: factor_offset.0,
            offset: factor_offset.1,
            min: min_max.map(|(min, _)| min),
            max: min_max.map(|(_, max)| max),
            unit,
            receiving_nodes,
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

#[test]
fn test_dbc_signal_multiplexer_01() {
    assert_eq!(
        dbc_signal_multiplexer(r#"M"#),
        Ok(("", DbcSignalMultiplexer::M)),
    );
}

#[test]
fn test_dbc_signal_multiplexer_02() {
    assert_eq!(
        dbc_signal_multiplexer(r#"m0"#),
        Ok(("", DbcSignalMultiplexer::MultiplexerIdentifier(0))),
    );
}

#[test]
fn test_dbc_signal_multiplexer_03() {
    assert_eq!(
        dbc_signal_multiplexer(r#"m123"#),
        Ok(("", DbcSignalMultiplexer::MultiplexerIdentifier(123))),
    );
}

#[test]
fn test_dbc_signal_01() {
    assert_eq!(
        dbc_signal(
            r#" SG_ AY1 : 32|16@1+ (0.000127465,-4.1768) [-4.1768|4.1765] "g"  ABS

"#
        ),
        Ok((
            "",
            DbcSignal {
                name: "AY1".into(),
                multiplexer: None,
                start_bit: 32,
                length: 16,
                endianness: DbcSignalEndianness::LittleEndian,
                signed: DbcSignalSigned::Unsigned,
                factor: 0.000127465,
                offset: -4.1768,
                min: Some(-4.1768),
                max: Some(4.1765),
                unit: Some("g".into()),
                receiving_nodes: Some(vec!["ABS".into()]),
            }
        )),
    );
}

#[test]
fn test_dbc_signal_02() {
    assert_eq!(
        dbc_signal(
            r#" SG_ S2 m0 : 8|8@1- (1.0,0.0) [0.0|0.0] "" Vector__XXX

"#
        ),
        Ok((
            "",
            DbcSignal {
                name: "S2".into(),
                multiplexer: Some(DbcSignalMultiplexer::MultiplexerIdentifier(0)),
                start_bit: 8,
                length: 8,
                endianness: DbcSignalEndianness::LittleEndian,
                signed: DbcSignalSigned::Signed,
                factor: 1.0,
                offset: 0.0,
                min: Some(0.0),
                max: Some(0.0),
                unit: Some("".into()),
                receiving_nodes: Some(vec!["Vector__XXX".into()]),
            }
        )),
    );
}

#[test]
fn test_dbc_signal_03() {
    assert_eq!(
        dbc_signal(
            r#" SG_ S2 m0 : 8|8@1- (1,0) [0|0] "" Vector__XXX

"#
        ),
        Ok((
            "",
            DbcSignal {
                name: "S2".into(),
                multiplexer: Some(DbcSignalMultiplexer::MultiplexerIdentifier(0)),
                start_bit: 8,
                length: 8,
                endianness: DbcSignalEndianness::LittleEndian,
                signed: DbcSignalSigned::Signed,
                factor: 1.0,
                offset: 0.0,
                min: Some(0.0),
                max: Some(0.0),
                unit: Some("".into()),
                receiving_nodes: Some(vec!["Vector__XXX".into()]),
            }
        )),
    );
}

#[test]
fn test_dbc_signal_04() {
    assert_eq!(
        dbc_signal(
            r#"  SG_ Signal1 : 32|32@1+ (100,0) [0|100] "%"  Node1,Node2

"#
        ),
        Ok((
            "",
            DbcSignal {
                name: "Signal1".into(),
                multiplexer: None,
                start_bit: 32,
                length: 32,
                endianness: DbcSignalEndianness::LittleEndian,
                signed: DbcSignalSigned::Unsigned,
                factor: 100.0,
                offset: 0.0,
                min: Some(0.0),
                max: Some(100.0),
                unit: Some("%".into()),
                receiving_nodes: Some(vec!["Node1".into(), "Node2".into()]),
            }
        )),
    );
}
