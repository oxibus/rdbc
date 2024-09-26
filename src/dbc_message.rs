use super::dbc_common_parsers::*;
use super::dbc_error::DbcParseError;
use super::dbc_signal::dbc_signal;
use super::dbc_signal::DbcSignal;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

/// Message definition.
/// Format: `BO_ <CAN-ID> <MessageName>: <MessageLength> <SendingNode>`
/// MessageLength in bytes.
#[derive(PartialEq, Debug, Clone)]
pub struct DbcMessageHeader {
    pub can_id: i64,
    pub name: String,
    pub length: i64,
    pub sending_node: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct DbcMessage {
    pub header: DbcMessageHeader,
    pub signals: Vec<DbcSignal>,
}

impl fmt::Display for DbcMessageHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BO_ {} {}: {} {}",
            self.can_id, self.name, self.length, self.sending_node
        )
    }
}

impl fmt::Display for DbcMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.header)?;
        for signal in &self.signals {
            writeln!(f, "\t{}", signal)?;
        }
        Ok(())
    }
}

fn dbc_message_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_object_name(input)
}

fn dbc_message_header(input: &str) -> IResult<&str, DbcMessageHeader, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("BO_")),
            spacey(integer_value), // can id
            spacey(dbc_message_name),
            spacey(tag(":")),
            spacey(integer_value), // length
            spacey(dbc_node_name),
        )),
        |(_, can_id, message_name, _, length, sending_node_name)| DbcMessageHeader {
            can_id,
            name: String::from(message_name),
            length,
            sending_node: String::from(sending_node_name),
        },
    )(input);

    match res {
        Ok((remain, header)) => {
            log::info!("parse message header: {:?}", header);
            Ok((remain, header))
        }
        Err(e) => {
            log::trace!("parse message header failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadMessageHeader))
        }
    }
}

pub fn dbc_message(input: &str) -> IResult<&str, DbcMessage, DbcParseError> {
    map(
        tuple((dbc_message_header, many0(dbc_signal), many0(line_ending))),
        |(header, signals, _)| DbcMessage { header, signals },
    )(input)
}

#[test]
fn test_dbc_message_header_01() {
    assert_eq!(
        dbc_message_header(r#"BO_ 2348941054 Normal: 8 Vector__XXX"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 2348941054,
                name: "Normal".into(),
                length: 8,
                sending_node: "Vector__XXX".into(),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header_02() {
    assert_eq!(
        dbc_message_header(r#"BO_ 2147487969 CANMultiplexed: 2 Node0"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 2147487969,
                name: "CANMultiplexed".into(),
                length: 2,
                sending_node: "Node0".into(),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header_03() {
    assert_eq!(
        dbc_message_header(r#"BO_ 1234 CANMessage: 8 Node0"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 1234,
                name: "CANMessage".into(),
                length: 8,
                sending_node: "Node0".into(),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header_04() {
    assert_eq!(
        dbc_message_header(r#"BO_ 835 BREMSE_33: 8 ABS"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 835,
                name: "BREMSE_33".into(),
                length: 8,
                sending_node: "ABS".into(),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header_05() {
    assert_eq!(
        dbc_message_header(r#"BO_ 117 DRS_RX_ID0: 8 ABS"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 117,
                name: "DRS_RX_ID0".into(),
                length: 8,
                sending_node: "ABS".into(),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header_06() {
    assert_eq!(
        dbc_message_header(r#"BO_ 1 M1: 8 FOO"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 1,
                name: "M1".into(),
                length: 8,
                sending_node: "FOO".into(),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header_07() {
    assert_eq!(
        dbc_message_header(r#"BO_ 1234 INV2EventMsg1: 8 Inv2"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 1234,
                name: "INV2EventMsg1".into(),
                length: 8,
                sending_node: "Inv2".into(),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header_08() {
    assert_eq!(
        dbc_message_header(r#"BO_ 83 Message_2: 8 ECU2"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 83,
                name: "Message_2".into(),
                length: 8,
                sending_node: "ECU2".into(),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header_09() {
    assert_eq!(
        dbc_message_header(r#"BO_ 2147483705 TheMessage: 8 Vector__XXX"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 2147483705,
                name: "TheMessage".into(),
                length: 8,
                sending_node: "Vector__XXX".into(),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header_10() {
    assert_eq!(
        dbc_message_header(r#"BO_ 1 Message1: 1 Vector__XXX"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 1,
                name: "Message1".into(),
                length: 1,
                sending_node: "Vector__XXX".into(),
            }
        )),
    );
}
