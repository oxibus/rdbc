use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::{IResult, Parser};

use super::char_string::{parser_char_string, CharString};
use super::common_parsers::{
    multispacey, parser_env_var_name, parser_message_id, parser_node_name, parser_signal_name,
};
use super::error::DbcParseError;

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NetworkComment {
    pub comment: CharString,
}

impl fmt::Display for NetworkComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"CM_ "{}";"#, self.comment)
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeComment {
    pub node_name: String,
    pub comment: CharString,
}

impl fmt::Display for NodeComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"CM_ BU_ {} "{}";"#, self.node_name, self.comment)
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageComment {
    pub message_id: u32,
    pub comment: CharString,
}

impl fmt::Display for MessageComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"CM_ BO_ {} "{}";"#, self.message_id, self.comment)
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignalComment {
    pub message_id: u32,
    pub signal_name: String,
    pub comment: CharString,
}

impl fmt::Display for SignalComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"CM_ SG_ {} {} "{}";"#,
            self.message_id, self.signal_name, self.comment
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentVariableComment {
    pub environment_variable_name: String,
    pub comment: CharString,
}

impl fmt::Display for EnvironmentVariableComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"CM_ EV_ {} "{}";"#,
            self.environment_variable_name, self.comment
        )
    }
}

/// The comment section contains the object comments. For each object having a
/// comment, an entry with the object's type identification is defined in this section.
///
/// ```text
/// comments = {comment} ;
/// comment = 'CM_' (char_string |
/// 'BU_' node_name char_string |
/// 'BO_' message_id char_string |
/// 'SG_' message_id signal_name char_string |
/// 'EV_' env_var_name char_string)
/// ';' ;
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Comment {
    Network(NetworkComment),
    Node(NodeComment),
    Message(MessageComment),
    Signal(SignalComment),
    EnvironmentVariable(EnvironmentVariableComment),
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comment::Network(comment) => write!(f, "{comment}"),
            Comment::Node(comment) => write!(f, "{comment}"),
            Comment::Message(comment) => write!(f, "{comment}"),
            Comment::Signal(comment) => write!(f, "{comment}"),
            Comment::EnvironmentVariable(comment) => write!(f, "{comment}"),
        }
    }
}

pub fn parser_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = alt((
        parser_network_comment,
        parser_node_comment,
        parser_message_comment,
        parser_signal_comment,
        parser_environment_variable_comment,
    ))
    .parse(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse comment: {comment:?}");
            Ok((remain, comment))
        }
        Err(e) => {
            log::trace!("parse comment failed, e = {e:?}");
            Err(nom::Err::Error(DbcParseError::BadComment))
        }
    }
}

pub fn parser_network_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        (
            multispacey(tag("CM_")),
            multispacey(parser_char_string),
            multispacey(tag(";")),
        ),
        |(_, comment, _)| NetworkComment { comment },
    )
    .parse(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse network comment: {comment:?}");
            Ok((remain, Comment::Network(comment)))
        }
        Err(e) => {
            log::trace!("parse network comment failed, e = {e:?}");
            Err(nom::Err::Error(DbcParseError::BadNetworkComment))
        }
    }
}

pub fn parser_node_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        (
            multispacey(tag("CM_")),
            multispacey(tag("BU_")),
            multispacey(parser_node_name),
            multispacey(parser_char_string),
            multispacey(tag(";")),
        ),
        |(_, _, node_name, comment, _)| NodeComment {
            node_name: node_name.to_string(),
            comment,
        },
    )
    .parse(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse node comment: {comment:?}");
            Ok((remain, Comment::Node(comment)))
        }
        Err(e) => {
            log::trace!("parse node comment failed, e = {e:?}");
            Err(nom::Err::Error(DbcParseError::BadNodeComment))
        }
    }
}

pub fn parser_message_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        (
            multispacey(tag("CM_")),
            multispacey(tag("BO_")),
            multispacey(parser_message_id),
            multispacey(parser_char_string),
            multispacey(tag(";")),
        ),
        |(_, _, message_id, comment, _)| MessageComment {
            message_id,
            comment,
        },
    )
    .parse(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse message comment: {comment:?}");
            Ok((remain, Comment::Message(comment)))
        }
        Err(e) => {
            log::trace!("parse message comment failed, e = {e:?}");
            Err(nom::Err::Error(DbcParseError::BadMessageComment))
        }
    }
}

pub fn parser_signal_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        (
            multispacey(tag("CM_")),
            multispacey(tag("SG_")),
            multispacey(parser_message_id),
            multispacey(parser_signal_name),
            multispacey(parser_char_string),
            multispacey(tag(";")),
        ),
        |(_, _, message_id, signal_name, comment, _)| SignalComment {
            message_id,
            signal_name: signal_name.to_string(),
            comment,
        },
    )
    .parse(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse signal comment: {comment:?}");
            Ok((remain, Comment::Signal(comment)))
        }
        Err(e) => {
            log::trace!("parse signal comment failed, e = {e:?}");
            Err(nom::Err::Error(DbcParseError::BadSignalComment))
        }
    }
}

pub fn parser_environment_variable_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        (
            multispacey(tag("CM_")),
            multispacey(tag("EV_")),
            multispacey(parser_env_var_name),
            multispacey(parser_char_string),
            multispacey(tag(";")),
        ),
        |(_, _, environment_variable_name, comment, _)| EnvironmentVariableComment {
            environment_variable_name: environment_variable_name.to_string(),
            comment,
        },
    )
    .parse(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse environment variable comment: {comment:?}");
            Ok((remain, Comment::EnvironmentVariable(comment)))
        }
        Err(e) => {
            log::trace!("parse environment variable comment failed, e = {e:?}");
            Err(nom::Err::Error(
                DbcParseError::BadEnvironmentVariableComment,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_comment_01() {
        assert_eq!(
            parser_comment(r#"CM_ "comment";"#),
            Ok((
                "",
                Comment::Network(NetworkComment {
                    comment: CharString("comment".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_comment_02() {
        assert_eq!(
            parser_comment(r#"CM_ BU_ Node0 "The 0th Node";"#),
            Ok((
                "",
                Comment::Node(NodeComment {
                    node_name: "Node0".into(),
                    comment: CharString("The 0th Node".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_network_comment_01() {
        assert_eq!(
            parser_network_comment(r#"CM_ "comment";"#),
            Ok((
                "",
                Comment::Network(NetworkComment {
                    comment: CharString("comment".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_network_comment_02() {
        assert_eq!(
            parser_network_comment(r#"CM_ "DBC Template with single line description";"#),
            Ok((
                "",
                Comment::Network(NetworkComment {
                    comment: CharString("DBC Template with single line description".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_node_comment_01() {
        assert_eq!(
            parser_node_comment(r#"CM_ BU_ Node0 "The 0th Node";"#),
            Ok((
                "",
                Comment::Node(NodeComment {
                    node_name: "Node0".into(),
                    comment: CharString("The 0th Node".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_node_comment_02() {
        assert_eq!(
            parser_node_comment(r#"CM_ BU_ TestNode "";"#),
            Ok((
                "",
                Comment::Node(NodeComment {
                    node_name: "TestNode".into(),
                    comment: CharString(String::new())
                })
            )),
        );
    }

    #[test]
    fn test_parser_node_comment_03() {
        assert_eq!(
            parser_node_comment(r#"CM_ BU_ BAR "fam \"1\"";"#),
            Ok((
                "",
                Comment::Node(NodeComment {
                    node_name: "BAR".into(),
                    comment: CharString(r#"fam \"1\""#.into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_node_comment_04() {
        assert_eq!(
            parser_node_comment(r#"CM_ BU_ DRIVER "// The driver controller driving the car //";"#),
            Ok((
                "",
                Comment::Node(NodeComment {
                    node_name: "DRIVER".into(),
                    comment: CharString("// The driver controller driving the car //".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_message_comment_01() {
        assert_eq!(
            parser_message_comment(
                r#"CM_ BO_ 496 "Example message used as template in MotoHawk models.";"#
            ),
            Ok((
                "",
                Comment::Message(MessageComment {
                    message_id: 496,
                    comment: CharString(
                        "Example message used as template in MotoHawk models.".into()
                    )
                })
            )),
        );
    }

    #[test]
    fn test_parser_message_comment_02() {
        assert_eq!(
            parser_message_comment(r#"CM_ BO_ 472 "No sender message.";"#),
            Ok((
                "",
                Comment::Message(MessageComment {
                    message_id: 472,
                    comment: CharString("No sender message.".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_message_comment_03() {
        assert_eq!(
            parser_message_comment(
                r#"CM_ BO_ 2303364386 "This cumulative distance calculation is updated when the trigger is active.";"#
            ),
            Ok((
                "",
                Comment::Message(MessageComment {
                    message_id: 2_303_364_386,
                    comment: CharString("This cumulative distance calculation is updated when the trigger is active.".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_signal_comment_01() {
        assert_eq!(
            parser_signal_comment(
                r#"CM_ SG_ 586 whlspeed_RL_Bremse2 "Radgeschwindigkeit / wheel speed direct RL";"#
            ),
            Ok((
                "",
                Comment::Signal(SignalComment {
                    message_id: 586,
                    signal_name: "whlspeed_RL_Bremse2".into(),
                    comment: CharString("Radgeschwindigkeit / wheel speed direct RL".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_signal_comment_02() {
        assert_eq!(
            parser_signal_comment(
                r#"CM_ SG_ 834 WheelQuality_FL "Bit matrix
Bit0 ( 1) Signal Reduced Monitored
Bit1 ( 2) Reduced Accuracy
Bit2 ( 4) Interfered
Bit3 ( 8) Suspicious Plausibility
Bit4 (16) Suspicious Lost
Bit5 (32) Not Initialized
Bit6 (64) Invalid Generic
Bit7 (128) Invalid Individual";"#
            ),
            Ok((
                "",
                Comment::Signal(SignalComment {
                    message_id: 834,
                    signal_name: "WheelQuality_FL".into(),
                    comment: CharString(
                        "Bit matrix
Bit0 ( 1) Signal Reduced Monitored
Bit1 ( 2) Reduced Accuracy
Bit2 ( 4) Interfered
Bit3 ( 8) Suspicious Plausibility
Bit4 (16) Suspicious Lost
Bit5 (32) Not Initialized
Bit6 (64) Invalid Generic
Bit7 (128) Invalid Individual"
                            .into()
                    )
                })
            )),
        );
    }

    #[test]
    fn test_parser_environmental_variable_comment_01() {
        assert_eq!(
            parser_environment_variable_comment(r#"CM_ EV_ EMC_Azimuth "Elevation Head";"#),
            Ok((
                "",
                Comment::EnvironmentVariable(EnvironmentVariableComment {
                    environment_variable_name: "EMC_Azimuth".into(),
                    comment: CharString("Elevation Head".into())
                })
            )),
        );
    }

    #[test]
    fn test_parser_environmental_variable_comment_02() {
        assert_eq!(
            parser_environment_variable_comment(
                r#"CM_ EV_ RWEnvVar_wData "This a comment for an environment variable";"#
            ),
            Ok((
                "",
                Comment::EnvironmentVariable(EnvironmentVariableComment {
                    environment_variable_name: "RWEnvVar_wData".into(),
                    comment: CharString("This a comment for an environment variable".into())
                })
            )),
        );
    }
}
