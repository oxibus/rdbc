use super::bit_timing::parser_bit_timing;
use super::bit_timing::BitTiming;
use super::common_parsers::*;
use super::error::DbcParseError;
use super::message::*;
use super::new_symbols::parser_new_symbols;
use super::new_symbols::NewSymbols;
use super::nodes::parser_nodes;
use super::nodes::Nodes;
use super::value_tables::*;
use super::version::parser_version;
use super::version::Version;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub struct NetworkAst {
    // VERSION "xxx"
    pub version: Version,

    // NS_:
    pub new_symbols: NewSymbols,

    // BS_:
    pub bit_timing: Option<BitTiming>,

    // BU_:
    pub nodes: Nodes,

    // VAL_TABLE_
    pub value_tables: Option<Vec<ValueTable>>,

    // BO_
    pub messages: Vec<Message>,
}

impl fmt::Display for NetworkAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n", self.version)?;
        writeln!(f, "{}", self.new_symbols)?;
        if let Some(bc) = &self.bit_timing {
            writeln!(f, "{}", bc)?;
        }
        writeln!(f, "{}", self.nodes)?;
        for message in &self.messages {
            writeln!(f, "{}", message)?;
        }
        Ok(())
    }
}

pub fn dbc_value(input: &str) -> IResult<&str, NetworkAst, DbcParseError> {
    map(
        multispacey(tuple((
            multispacey(parser_version),
            multispacey(parser_new_symbols),
            multispacey(parser_bit_timing),
            multispacey(parser_nodes),
            multispacey(parser_value_tables),
            multispacey(many0(parser_dbc_message)),
        ))),
        |(version, new_symbols, bit_timing, nodes, value_tables, messages)| NetworkAst {
            version,
            new_symbols,
            bit_timing,
            nodes,
            value_tables,
            messages,
        },
    )(input)
}

pub fn parse_dbc(input: &str) -> Result<NetworkAst, DbcParseError> {
    let (_remain, result) = all_consuming(dbc_value)(input).map_err(|nom_err| {
        log::error!("nom_err: {}", nom_err);
        match nom_err {
            nom::Err::Incomplete(_) => unreachable!(),
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    })?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::signal;

    #[test]
    fn test_dbc_01() {
        assert_eq!(
            parse_dbc(
                r#"VERSION "1.0"


NS_:
    BS_
    CM_

BS_:
BU_: ABS DRS_MM5_10

BO_ 117 DRS_RX_ID0: 8 ABS

BO_ 112 MM5_10_TX1: 8 DRS_MM5_10
 SG_ Yaw_Rate : 0|16@1+ (0.005,-163.84) [-163.84|163.83] "°/s"  ABS
 SG_ AY1 : 32|16@1+ (0.000127465,-4.1768) [-4.1768|4.1765] "g"  ABS

"#
            ),
            Ok(NetworkAst {
                version: Version("1.0".into()),
                new_symbols: NewSymbols(vec!["BS_".into(), "CM_".into()]),
                bit_timing: Some(BitTiming { value: None }),
                nodes: Nodes(vec!["ABS".into(), "DRS_MM5_10".into()]),
                value_tables: None,
                messages: vec![
                    Message {
                        header: MessageHeader {
                            id: 117,
                            name: "DRS_RX_ID0".into(),
                            size: 8,
                            transmitter: "ABS".into(),
                        },
                        signals: vec![],
                    },
                    Message {
                        header: MessageHeader {
                            id: 112,
                            name: "MM5_10_TX1".into(),
                            size: 8,
                            transmitter: "DRS_MM5_10".into(),
                        },
                        signals: vec![
                            signal::Signal {
                                name: "Yaw_Rate".into(),
                                multiplexer: None,
                                start_bit: 0,
                                length: 16,
                                endianness: signal::DbcSignalEndianness::LittleEndian,
                                signed: signal::DbcSignalSigned::Unsigned,
                                factor: 0.005,
                                offset: -163.84,
                                min: Some(-163.84),
                                max: Some(163.83),
                                unit: Some("°/s".into()),
                                receiving_nodes: Some(vec!["ABS".into()]),
                            },
                            signal::Signal {
                                name: "AY1".into(),
                                multiplexer: None,
                                start_bit: 32,
                                length: 16,
                                endianness: signal::DbcSignalEndianness::LittleEndian,
                                signed: signal::DbcSignalSigned::Unsigned,
                                factor: 0.000127465,
                                offset: -4.1768,
                                min: Some(-4.1768),
                                max: Some(4.1765),
                                unit: Some("g".into()),
                                receiving_nodes: Some(vec!["ABS".into()]),
                            }
                        ],
                    },
                ],
            }),
        );
    }

    #[test]
    fn test_dbc_02() {
        assert_eq!(
            parse_dbc(
                r#"VERSION "1.0"


NS_:
    BS_
    CM_

BS_:
BU_: ABS DRS_MM5_10

VAL_TABLE_ ABS_fault_info 2 "active faults stored" 1 "inactive faults stored" 0 "no faults stored" ;
VAL_TABLE_ vt_WheelSpeedQualifier 5 "InvalidUnderVoltage" 4 "NotCalculated" 3 "ReducedMonitored" 2 "Faulty" 1 "Normal" 0 "NotInitialized" ;


BO_ 117 DRS_RX_ID0: 8 ABS

BO_ 112 MM5_10_TX1: 8 DRS_MM5_10
 SG_ Yaw_Rate : 0|16@1+ (0.005,-163.84) [-163.84|163.83] "°/s"  ABS
 SG_ AY1 : 32|16@1+ (0.000127465,-4.1768) [-4.1768|4.1765] "g"  ABS

"#
            ),
            Ok(NetworkAst {
                version: Version("1.0".into()),
                new_symbols: NewSymbols(vec!["BS_".into(), "CM_".into()]),
                bit_timing: Some(BitTiming { value: None }),
                nodes: Nodes(vec!["ABS".into(), "DRS_MM5_10".into()]),
                value_tables: Some(vec![
                    ValueTable {
                        name: "ABS_fault_info".to_string(),
                        values: ValueDescriptions {
                            values: vec![
                                ValueDescriptionItem {
                                    num: 2,
                                    str: "active faults stored".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 1,
                                    str: "inactive faults stored".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 0,
                                    str: "no faults stored".to_string()
                                }
                            ]
                        }
                    },
                    ValueTable {
                        name: "vt_WheelSpeedQualifier".to_string(),
                        values: ValueDescriptions {
                            values: vec![
                                ValueDescriptionItem {
                                    num: 5,
                                    str: "InvalidUnderVoltage".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 4,
                                    str: "NotCalculated".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 3,
                                    str: "ReducedMonitored".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 2,
                                    str: "Faulty".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 1,
                                    str: "Normal".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 0,
                                    str: "NotInitialized".to_string()
                                }
                            ]
                        }
                    }
                ]),
                messages: vec![
                    Message {
                        header: MessageHeader {
                            id: 117,
                            name: "DRS_RX_ID0".into(),
                            size: 8,
                            transmitter: "ABS".into(),
                        },
                        signals: vec![],
                    },
                    Message {
                        header: MessageHeader {
                            id: 112,
                            name: "MM5_10_TX1".into(),
                            size: 8,
                            transmitter: "DRS_MM5_10".into(),
                        },
                        signals: vec![
                            signal::Signal {
                                name: "Yaw_Rate".into(),
                                multiplexer: None,
                                start_bit: 0,
                                length: 16,
                                endianness: signal::DbcSignalEndianness::LittleEndian,
                                signed: signal::DbcSignalSigned::Unsigned,
                                factor: 0.005,
                                offset: -163.84,
                                min: Some(-163.84),
                                max: Some(163.83),
                                unit: Some("°/s".into()),
                                receiving_nodes: Some(vec!["ABS".into()]),
                            },
                            signal::Signal {
                                name: "AY1".into(),
                                multiplexer: None,
                                start_bit: 32,
                                length: 16,
                                endianness: signal::DbcSignalEndianness::LittleEndian,
                                signed: signal::DbcSignalSigned::Unsigned,
                                factor: 0.000127465,
                                offset: -4.1768,
                                min: Some(-4.1768),
                                max: Some(4.1765),
                                unit: Some("g".into()),
                                receiving_nodes: Some(vec!["ABS".into()]),
                            }
                        ],
                    },
                ],
            }),
        );
    }
}
