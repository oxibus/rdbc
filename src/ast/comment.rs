use super::common_parsers::*;
use super::error::DbcParseError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(PartialEq, Debug, Clone)]
pub struct NetworkComment {
    pub comment: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct NodeComment {
    pub node_name: String,
    pub comment: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct MessageComment {
    pub message_id: u32,
    pub comment: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SignalComment {
    pub message_id: u32,
    pub signal_name: String,
    pub comment: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct EnvironmentVariableComment {
    pub environment_variable_name: String,
    pub comment: String,
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
pub enum Comment {
    Network(NetworkComment),
    Node(NodeComment),
    Message(MessageComment),
    Signal(SignalComment),
    EnvironmentVariable(EnvironmentVariableComment),
}

pub fn parser_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = alt((
        parser_network_comment,
        parser_node_comment,
        parser_message_comment,
        parser_signal_comment,
        parser_environment_variable_comment,
    ))(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse comment: {:?}", comment);
            Ok((remain, comment))
        }
        Err(e) => {
            log::trace!("parse comment failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadComment))
        }
    }
}

pub fn parser_network_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("CM_")),
            multispacey(char_string),
            multispacey(tag(";")),
        )),
        |(_, comment, _)| NetworkComment { comment },
    )(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse network comment: {:?}", comment);
            Ok((remain, Comment::Network(comment)))
        }
        Err(e) => {
            log::trace!("parse network comment failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadNetworkComment))
        }
    }
}

pub fn parser_node_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("CM_")),
            multispacey(tag("BU_")),
            multispacey(parser_node_name),
            multispacey(char_string),
            multispacey(tag(";")),
        )),
        |(_, _, node_name, comment, _)| NodeComment {
            node_name: node_name.to_string(),
            comment,
        },
    )(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse node comment: {:?}", comment);
            Ok((remain, Comment::Node(comment)))
        }
        Err(e) => {
            log::trace!("parse node comment failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadNodeComment))
        }
    }
}

pub fn parser_message_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("CM_")),
            multispacey(tag("BO_")),
            multispacey(parser_message_id),
            multispacey(char_string),
            multispacey(tag(";")),
        )),
        |(_, _, message_id, comment, _)| MessageComment {
            message_id,
            comment,
        },
    )(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse message comment: {:?}", comment);
            Ok((remain, Comment::Message(comment)))
        }
        Err(e) => {
            log::trace!("parse message comment failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadMessageComment))
        }
    }
}

pub fn parser_signal_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("CM_")),
            multispacey(tag("SG_")),
            multispacey(parser_message_id),
            multispacey(parser_signal_name),
            multispacey(char_string),
            multispacey(tag(";")),
        )),
        |(_, _, message_id, signal_name, comment, _)| SignalComment {
            message_id,
            signal_name: signal_name.to_string(),
            comment,
        },
    )(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse signal comment: {:?}", comment);
            Ok((remain, Comment::Signal(comment)))
        }
        Err(e) => {
            log::trace!("parse signal comment failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadSignalComment))
        }
    }
}

pub fn parser_environment_variable_comment(input: &str) -> IResult<&str, Comment, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("CM_")),
            multispacey(tag("EV_")),
            multispacey(parser_env_var_name),
            multispacey(char_string),
            multispacey(tag(";")),
        )),
        |(_, _, environment_variable_name, comment, _)| EnvironmentVariableComment {
            environment_variable_name: environment_variable_name.to_string(),
            comment,
        },
    )(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse environment variable comment: {:?}", comment);
            Ok((remain, Comment::EnvironmentVariable(comment)))
        }
        Err(e) => {
            log::trace!("parse environment variable comment failed, e = {:?}", e);
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
                    comment: "comment".into()
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
                    comment: "The 0th Node".into()
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
                    comment: "comment".into()
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
                    comment: "DBC Template with single line description".into()
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
                    comment: "The 0th Node".into()
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
                    comment: "".into()
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
                    comment: r#"fam \"1\""#.into()
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
                    comment: "// The driver controller driving the car //".into()
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
                    comment: "Example message used as template in MotoHawk models.".into()
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
                    comment: "No sender message.".into()
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
                    message_id: 2303364386,
                    comment: "This cumulative distance calculation is updated when the trigger is active.".into()
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
                    comment: "Radgeschwindigkeit / wheel speed direct RL".into()
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
                    comment: r#"Bit matrix
Bit0 ( 1) Signal Reduced Monitored
Bit1 ( 2) Reduced Accuracy
Bit2 ( 4) Interfered
Bit3 ( 8) Suspicious Plausibility
Bit4 (16) Suspicious Lost
Bit5 (32) Not Initialized
Bit6 (64) Invalid Generic
Bit7 (128) Invalid Individual"#
                        .into()
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
                    comment: "Elevation Head".into()
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
                    comment: "This a comment for an environment variable".into()
                })
            )),
        );
    }
}
