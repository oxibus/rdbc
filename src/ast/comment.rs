use super::common_parsers::*;
use super::error::DbcParseError;
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

pub fn parser_network_comment(input: &str) -> IResult<&str, NetworkComment, DbcParseError> {
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
            Ok((remain, comment))
        }
        Err(e) => {
            log::trace!("parse network comment failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadNetworkComment))
        }
    }
}

pub fn parser_node_comment(input: &str) -> IResult<&str, NodeComment, DbcParseError> {
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
            Ok((remain, comment))
        }
        Err(e) => {
            log::trace!("parse node comment failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadNodeComment))
        }
    }
}

pub fn parser_message_comment(input: &str) -> IResult<&str, MessageComment, DbcParseError> {
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
            Ok((remain, comment))
        }
        Err(e) => {
            log::trace!("parse message comment failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadMessageComment))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_network_comment_01() {
        assert_eq!(
            parser_network_comment(r#"CM_ "comment";"#),
            Ok((
                "",
                NetworkComment {
                    comment: "comment".into()
                }
            )),
        );
    }

    #[test]
    fn test_parser_network_comment_02() {
        assert_eq!(
            parser_network_comment(r#"CM_ "DBC Template with single line description";"#),
            Ok((
                "",
                NetworkComment {
                    comment: "DBC Template with single line description".into()
                }
            )),
        );
    }

    #[test]
    fn test_parser_node_comment_01() {
        assert_eq!(
            parser_node_comment(r#"CM_ BU_ Node0 "The 0th Node";"#),
            Ok((
                "",
                NodeComment {
                    node_name: "Node0".into(),
                    comment: "The 0th Node".into()
                }
            )),
        );
    }

    #[test]
    fn test_parser_node_comment_02() {
        assert_eq!(
            parser_node_comment(r#"CM_ BU_ TestNode "";"#),
            Ok((
                "",
                NodeComment {
                    node_name: "TestNode".into(),
                    comment: "".into()
                }
            )),
        );
    }

    #[test]
    fn test_parser_node_comment_03() {
        assert_eq!(
            parser_node_comment(r#"CM_ BU_ BAR "fam \"1\"";"#),
            Ok((
                "",
                NodeComment {
                    node_name: "BAR".into(),
                    comment: "fam \"1\"".into()
                }
            )),
        );
    }

    #[test]
    fn test_parser_node_comment_04() {
        assert_eq!(
            parser_node_comment(r#"CM_ BU_ DRIVER "// The driver controller driving the car //";"#),
            Ok((
                "",
                NodeComment {
                    node_name: "DRIVER".into(),
                    comment: "// The driver controller driving the car //".into()
                }
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
                MessageComment {
                    message_id: 496,
                    comment: "Example message used as template in MotoHawk models.".into()
                }
            )),
        );
    }

    #[test]
    fn test_parser_message_comment_02() {
        assert_eq!(
            parser_message_comment(r#"CM_ BO_ 472 "No sender message.";"#),
            Ok((
                "",
                MessageComment {
                    message_id: 472,
                    comment: "No sender message.".into()
                }
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
                MessageComment {
                    message_id: 2303364386,
                    comment: "This cumulative distance calculation is updated when the trigger is active.".into()
                }
            )),
        );
    }
}
