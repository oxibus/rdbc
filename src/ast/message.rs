use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::many0;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

use super::common_parsers::{
    dbc_identifier, multispacey, parser_message_id, parser_node_name, spacey, unsigned_integer,
};
use super::error::DbcParseError;
use super::signal::{parser_signal, Signal};

/// Message definition.
/// Format: `BO_ <CAN-ID> <MessageName>: <MessageSize> <SendingNode>`
/// `MessageSize` in bytes.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    /// The message's CAN-ID. The CAN-ID has to be unique within the DBC file. If the
    /// most significant bit of the CAN-ID is set, the ID is an extended CAN ID.
    /// The extended CAN ID can be determined by masking out the most significant bit
    /// with the mask 0x7FFFFFFF.
    pub id: u32,

    /// The names defined in this section have to be unique within the set of messages.
    pub name: String,

    /// The `message_size` specifies the size of the message in bytes.
    pub size: u32,

    /// The transmitter name specifies the name of the node transmitting the message.
    /// The sender name has to be defined in the set of node names in the node section.
    /// If the massage shall have no sender, the string `Vector__XXX` has to be given
    /// here.
    pub transmitter: String,
}

/// The message section defines the names of all frames in the cluster as well as their
/// properties and the signals transferred on the frames.
///
/// ```text
/// messages = {message} ;
/// message = BO_ message_id message_name ':' message_size transmitter {signal} ;
/// message_id = unsigned_integer ;
/// message_name = DBC_identifier ;
/// message_size = unsigned_integer ;
/// transmitter = node_name | 'Vector__XXX' ;
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub header: MessageHeader,
    pub signals: Vec<Signal>,
}

impl fmt::Display for MessageHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BO_ {} {}: {} {}",
            self.id, self.name, self.size, self.transmitter
        )
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.header)?;
        for signal in &self.signals {
            writeln!(f, "\t{signal}")?;
        }
        Ok(())
    }
}

fn parser_message_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_identifier(input)
}

fn parser_message_size(input: &str) -> IResult<&str, u32, DbcParseError> {
    unsigned_integer(input)
}

fn parser_transmitter(input: &str) -> IResult<&str, &str, DbcParseError> {
    alt((parser_node_name, tag("Vector__XXX"))).parse(input)
}

fn parser_message_header(input: &str) -> IResult<&str, MessageHeader, DbcParseError> {
    let res = map(
        (
            multispacey(tag("BO_")),
            spacey(parser_message_id),
            spacey(parser_message_name),
            spacey(tag(":")),
            spacey(parser_message_size),
            spacey(parser_transmitter),
        ),
        |(_, id, message_name, _, size, sending_node_name)| MessageHeader {
            id,
            name: String::from(message_name),
            size,
            transmitter: String::from(sending_node_name),
        },
    )
    .parse(input);

    match res {
        Ok((remain, header)) => {
            log::info!("parse message header: {header:?}");
            Ok((remain, header))
        }
        Err(e) => {
            log::trace!("parse message header failed, e = {e:?}");
            Err(nom::Err::Error(DbcParseError::BadMessageHeader))
        }
    }
}

pub fn parser_dbc_message(input: &str) -> IResult<&str, Message, DbcParseError> {
    map(
        (
            parser_message_header,
            many0(parser_signal),
            many0(line_ending),
        ),
        |(header, signals, _)| Message { header, signals },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbc_message_header_01() {
        assert_eq!(
            parser_message_header("BO_ 2348941054 Normal: 8 Vector__XXX"),
            Ok((
                "",
                MessageHeader {
                    id: 2_348_941_054,
                    name: "Normal".into(),
                    size: 8,
                    transmitter: "Vector__XXX".into(),
                }
            )),
        );
    }

    #[test]
    fn test_dbc_message_header_02() {
        assert_eq!(
            parser_message_header("BO_ 2147487969 CANMultiplexed: 2 Node0"),
            Ok((
                "",
                MessageHeader {
                    id: 2_147_487_969,
                    name: "CANMultiplexed".into(),
                    size: 2,
                    transmitter: "Node0".into(),
                }
            )),
        );
    }

    #[test]
    fn test_dbc_message_header_03() {
        assert_eq!(
            parser_message_header("BO_ 1234 CANMessage: 8 Node0"),
            Ok((
                "",
                MessageHeader {
                    id: 1234,
                    name: "CANMessage".into(),
                    size: 8,
                    transmitter: "Node0".into(),
                }
            )),
        );
    }

    #[test]
    fn test_dbc_message_header_04() {
        assert_eq!(
            parser_message_header("BO_ 835 BREMSE_33: 8 ABS"),
            Ok((
                "",
                MessageHeader {
                    id: 835,
                    name: "BREMSE_33".into(),
                    size: 8,
                    transmitter: "ABS".into(),
                }
            )),
        );
    }

    #[test]
    fn test_dbc_message_header_05() {
        assert_eq!(
            parser_message_header("BO_ 117 DRS_RX_ID0: 8 ABS"),
            Ok((
                "",
                MessageHeader {
                    id: 117,
                    name: "DRS_RX_ID0".into(),
                    size: 8,
                    transmitter: "ABS".into(),
                }
            )),
        );
    }

    #[test]
    fn test_dbc_message_header_06() {
        assert_eq!(
            parser_message_header("BO_ 1 M1: 8 FOO"),
            Ok((
                "",
                MessageHeader {
                    id: 1,
                    name: "M1".into(),
                    size: 8,
                    transmitter: "FOO".into(),
                }
            )),
        );
    }

    #[test]
    fn test_dbc_message_header_07() {
        assert_eq!(
            parser_message_header("BO_ 1234 INV2EventMsg1: 8 Inv2"),
            Ok((
                "",
                MessageHeader {
                    id: 1234,
                    name: "INV2EventMsg1".into(),
                    size: 8,
                    transmitter: "Inv2".into(),
                }
            )),
        );
    }

    #[test]
    fn test_dbc_message_header_08() {
        assert_eq!(
            parser_message_header("BO_ 83 Message_2: 8 ECU2"),
            Ok((
                "",
                MessageHeader {
                    id: 83,
                    name: "Message_2".into(),
                    size: 8,
                    transmitter: "ECU2".into(),
                }
            )),
        );
    }

    #[test]
    fn test_dbc_message_header_09() {
        assert_eq!(
            parser_message_header("BO_ 2147483705 TheMessage: 8 Vector__XXX"),
            Ok((
                "",
                MessageHeader {
                    id: 2_147_483_705,
                    name: "TheMessage".into(),
                    size: 8,
                    transmitter: "Vector__XXX".into(),
                }
            )),
        );
    }

    #[test]
    fn test_dbc_message_header_10() {
        assert_eq!(
            parser_message_header("BO_ 1 Message1: 1 Vector__XXX"),
            Ok((
                "",
                MessageHeader {
                    id: 1,
                    name: "Message1".into(),
                    size: 1,
                    transmitter: "Vector__XXX".into(),
                }
            )),
        );
    }
}
