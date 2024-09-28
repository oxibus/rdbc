use super::common_parsers::*;
use super::error::DbcParseError;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(PartialEq, Debug, Clone)]
pub struct NetworkCommnet {
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
    Network(NetworkCommnet),
    Node(NodeComment),
    Message(MessageComment),
    Signal(SignalComment),
    EnvironmentVariable(EnvironmentVariableComment),
}

pub fn parser_network_comment(input: &str) -> IResult<&str, NetworkCommnet, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("CM_")),
            multispacey(string_literal),
            multispacey(tag(";")),
        )),
        |(_, comment, _)| NetworkCommnet { comment },
    )(input);

    match res {
        Ok((remain, comment)) => {
            log::info!("parse comment: {:?}", comment);
            Ok((remain, comment))
        }
        Err(e) => {
            log::trace!("parse comment failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadNetworkComment))
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
                NetworkCommnet {
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
                NetworkCommnet {
                    comment: "DBC Template with single line description".into()
                }
            )),
        );
    }
}
