use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

use super::attribute::parser_attribute_name;
use super::attribute_default::{parser_attribute_value, AttributeValue};
use super::common_parsers::*;
use super::error::DbcParseError;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAttributeValue {
    pub attribute_name: String,
    pub attribute_value: AttributeValue,
}

impl fmt::Display for NetworkAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_ \"{}\" {};",
            self.attribute_name, self.attribute_value
        )
    }
}

pub fn parser_network_attribute_value(
    input: &str,
) -> IResult<&str, ObjectAttributeValue, DbcParseError> {
    let res = map(
        (
            multispacey(tag("BA_")),
            multispacey(parser_attribute_name),
            multispacey(parser_attribute_value),
            multispacey(tag(";")),
        ),
        |(_, attribute_name, attribute_value, _)| NetworkAttributeValue {
            attribute_name: attribute_name.to_string(),
            attribute_value,
        },
    )
    .parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse network attribute value: {:?}", value);
            Ok((remain, ObjectAttributeValue::Network(value)))
        }
        Err(e) => {
            log::trace!("parse network attribute value failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadNetworkAttributeValue))
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NodeAttributeValue {
    pub attribute_name: String,
    pub node_name: String,
    pub attribute_value: AttributeValue,
}

impl fmt::Display for NodeAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_ \"{}\" BU_ {} {};",
            self.attribute_name, self.node_name, self.attribute_value
        )
    }
}

pub fn parser_node_attribute_value(
    input: &str,
) -> IResult<&str, ObjectAttributeValue, DbcParseError> {
    let res = map(
        (
            multispacey(tag("BA_")),
            multispacey(parser_attribute_name),
            multispacey(tag("BU_")),
            multispacey(parser_node_name),
            multispacey(parser_attribute_value),
            multispacey(tag(";")),
        ),
        |(_, attribute_name, _, node_name, attribute_value, _)| NodeAttributeValue {
            attribute_name: attribute_name.to_string(),
            node_name: node_name.to_string(),
            attribute_value,
        },
    )
    .parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse node attribute value: {:?}", value);
            Ok((remain, ObjectAttributeValue::Node(value)))
        }
        Err(e) => {
            log::trace!("parse node attribute value failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadNodeAttributeValue))
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttributeValue {
    pub attribute_name: String,
    pub message_id: u32,
    pub attribute_value: AttributeValue,
}

impl fmt::Display for MessageAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_ \"{}\" BO_ {} {};",
            self.attribute_name, self.message_id, self.attribute_value
        )
    }
}

pub fn parser_message_attribute_value(
    input: &str,
) -> IResult<&str, ObjectAttributeValue, DbcParseError> {
    let res = map(
        (
            multispacey(tag("BA_")),
            multispacey(parser_attribute_name),
            multispacey(tag("BO_")),
            multispacey(parser_message_id),
            multispacey(parser_attribute_value),
            multispacey(tag(";")),
        ),
        |(_, attribute_name, _, message_id, attribute_value, _)| MessageAttributeValue {
            attribute_name: attribute_name.to_string(),
            message_id,
            attribute_value,
        },
    )
    .parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse message attribute value: {:?}", value);
            Ok((remain, ObjectAttributeValue::Message(value)))
        }
        Err(e) => {
            log::trace!("parse message attribute value failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadMessageAttributeValue))
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SignalAttributeValue {
    pub attribute_name: String,
    pub message_id: u32,
    pub signal_name: String,
    pub attribute_value: AttributeValue,
}

impl fmt::Display for SignalAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_ \"{}\" SG_ {} {} {};",
            self.attribute_name, self.message_id, self.signal_name, self.attribute_value
        )
    }
}

pub fn parser_signal_attribute_value(
    input: &str,
) -> IResult<&str, ObjectAttributeValue, DbcParseError> {
    let res = map(
        (
            multispacey(tag("BA_")),
            multispacey(parser_attribute_name),
            multispacey(tag("SG_")),
            multispacey(parser_message_id),
            multispacey(parser_signal_name),
            multispacey(parser_attribute_value),
            multispacey(tag(";")),
        ),
        |(_, attribute_name, _, message_id, signal_name, attribute_value, _)| {
            SignalAttributeValue {
                attribute_name: attribute_name.to_string(),
                message_id,
                signal_name: signal_name.to_string(),
                attribute_value,
            }
        },
    )
    .parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse signal attribute value: {:?}", value);
            Ok((remain, ObjectAttributeValue::Signal(value)))
        }
        Err(e) => {
            log::trace!("parse signal attribute value failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadSignalAttributeValue))
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentVariableAttributeValue {
    pub attribute_name: String,
    pub env_var_name: String,
    pub attribute_value: AttributeValue,
}

impl fmt::Display for EnvironmentVariableAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_ \"{}\" EV_ {} {};",
            self.attribute_name, self.env_var_name, self.attribute_value
        )
    }
}

pub fn parser_environment_variable_attribute_value(
    input: &str,
) -> IResult<&str, ObjectAttributeValue, DbcParseError> {
    let res = map(
        (
            multispacey(tag("BA_")),
            multispacey(parser_attribute_name),
            multispacey(tag("EV_")),
            multispacey(parser_env_var_name),
            multispacey(parser_attribute_value),
            multispacey(tag(";")),
        ),
        |(_, attribute_name, _, env_var_name, attribute_value, _)| {
            EnvironmentVariableAttributeValue {
                attribute_name: attribute_name.to_string(),
                env_var_name: env_var_name.to_string(),
                attribute_value,
            }
        },
    )
    .parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse environment variable attribute value: {:?}", value);
            Ok((remain, ObjectAttributeValue::EnvironmentVariable(value)))
        }
        Err(e) => {
            log::trace!(
                "parse environment variable attribute value failed, e = {:?}",
                e
            );
            Err(nom::Err::Error(
                DbcParseError::BadEnvironmentVariableAttributeValue,
            ))
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ObjectAttributeValue {
    Network(NetworkAttributeValue),
    Node(NodeAttributeValue),
    Message(MessageAttributeValue),
    Signal(SignalAttributeValue),
    EnvironmentVariable(EnvironmentVariableAttributeValue),
}

impl fmt::Display for ObjectAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectAttributeValue::Network(v) => write!(f, "{}", v),
            ObjectAttributeValue::Node(v) => write!(f, "{}", v),
            ObjectAttributeValue::Message(v) => write!(f, "{}", v),
            ObjectAttributeValue::Signal(v) => write!(f, "{}", v),
            ObjectAttributeValue::EnvironmentVariable(v) => write!(f, "{}", v),
        }
    }
}

pub fn parser_object_attribute_value(
    input: &str,
) -> IResult<&str, ObjectAttributeValue, DbcParseError> {
    let res = alt((
        parser_network_attribute_value,
        parser_node_attribute_value,
        parser_message_attribute_value,
        parser_signal_attribute_value,
        parser_environment_variable_attribute_value,
    ))
    .parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute value: {:?}", value);
            Ok((remain, value))
        }
        Err(e) => {
            log::trace!("parse attribute value failed, e = {:?}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::char_string::CharString;

    #[test]
    fn test_object_attribute_value_string_01() {
        assert_eq!(
            ObjectAttributeValue::Network(NetworkAttributeValue {
                attribute_name: "FloatAttribute".to_string(),
                attribute_value: AttributeValue::Double(45.9)
            })
            .to_string(),
            r#"BA_ "FloatAttribute" 45.9;"#
        );
    }

    #[test]
    fn test_object_attribute_value_string_02() {
        assert_eq!(
            ObjectAttributeValue::Node(NodeAttributeValue {
                attribute_name: "BUIntAttribute".to_string(),
                node_name: "Node0".to_string(),
                attribute_value: AttributeValue::Double(100.0)
            })
            .to_string(),
            r#"BA_ "BUIntAttribute" BU_ Node0 100;"#
        );
    }

    #[test]
    fn test_object_attribute_value_string_03() {
        assert_eq!(
            ObjectAttributeValue::Message(MessageAttributeValue {
                attribute_name: "BOStringAttribute".to_string(),
                message_id: 1234,
                attribute_value: AttributeValue::String(CharString("MessageAttribute".to_string()))
            })
            .to_string(),
            r#"BA_ "BOStringAttribute" BO_ 1234 "MessageAttribute";"#
        );
    }

    #[test]
    fn test_object_attribute_value_string_04() {
        assert_eq!(
            ObjectAttributeValue::Signal(SignalAttributeValue {
                attribute_name: "SGEnumAttribute".to_string(),
                message_id: 1234,
                signal_name: "Signal0".to_string(),
                attribute_value: AttributeValue::Double(2.0)
            })
            .to_string(),
            r#"BA_ "SGEnumAttribute" SG_ 1234 Signal0 2;"#
        );
    }

    #[test]
    fn test_object_attribute_value_string_05() {
        assert_eq!(
            ObjectAttributeValue::EnvironmentVariable(EnvironmentVariableAttributeValue {
                attribute_name: "RWEnvVar_wData_Val".to_string(),
                env_var_name: "RWEnvVar_wData".to_string(),
                attribute_value: AttributeValue::Double(3.0)
            })
            .to_string(),
            r#"BA_ "RWEnvVar_wData_Val" EV_ RWEnvVar_wData 3;"#
        );
    }

    #[test]
    fn test_parser_network_attribute_value_01() {
        assert_eq!(
            parser_network_attribute_value(r#"BA_ "FloatAttribute" 45.9;"#),
            Ok((
                "",
                ObjectAttributeValue::Network(NetworkAttributeValue {
                    attribute_name: "FloatAttribute".to_string(),
                    attribute_value: AttributeValue::Double(45.9)
                })
            ))
        );
    }

    #[test]
    fn test_parser_node_attribute_value_01() {
        assert_eq!(
            parser_node_attribute_value(r#"BA_ "BUIntAttribute" BU_ Node0 100;"#),
            Ok((
                "",
                ObjectAttributeValue::Node(NodeAttributeValue {
                    attribute_name: "BUIntAttribute".to_string(),
                    node_name: "Node0".to_string(),
                    attribute_value: AttributeValue::Double(100.0)
                })
            ))
        );
    }

    #[test]
    fn test_parser_message_attribute_value_01() {
        assert_eq!(
            parser_message_attribute_value(
                r#"BA_ "BOStringAttribute" BO_ 1234 "MessageAttribute";"#
            ),
            Ok((
                "",
                ObjectAttributeValue::Message(MessageAttributeValue {
                    attribute_name: "BOStringAttribute".to_string(),
                    message_id: 1234,
                    attribute_value: AttributeValue::String(CharString(
                        "MessageAttribute".to_string()
                    ))
                })
            ))
        );
    }

    #[test]
    fn test_parser_signal_attribute_value_01() {
        assert_eq!(
            parser_signal_attribute_value(r#"BA_ "SGEnumAttribute" SG_ 1234 Signal0 2;"#),
            Ok((
                "",
                ObjectAttributeValue::Signal(SignalAttributeValue {
                    attribute_name: "SGEnumAttribute".to_string(),
                    message_id: 1234,
                    signal_name: "Signal0".to_string(),
                    attribute_value: AttributeValue::Double(2.0)
                })
            ))
        );
    }

    #[test]
    fn test_parser_environment_variable_attribute_value_01() {
        assert_eq!(
            parser_environment_variable_attribute_value(
                r#"BA_ "RWEnvVar_wData_Val" EV_ RWEnvVar_wData 3;"#
            ),
            Ok((
                "",
                ObjectAttributeValue::EnvironmentVariable(EnvironmentVariableAttributeValue {
                    attribute_name: "RWEnvVar_wData_Val".to_string(),
                    env_var_name: "RWEnvVar_wData".to_string(),
                    attribute_value: AttributeValue::Double(3.0)
                })
            ))
        );
    }

    #[test]
    fn test_parser_object_attribute_value_01() {
        assert_eq!(
            parser_object_attribute_value(r#"BA_ "FloatAttribute" 45.9;"#),
            Ok((
                "",
                ObjectAttributeValue::Network(NetworkAttributeValue {
                    attribute_name: "FloatAttribute".to_string(),
                    attribute_value: AttributeValue::Double(45.9)
                })
            ))
        );
    }

    #[test]
    fn test_parser_object_attribute_value_02() {
        assert_eq!(
            parser_object_attribute_value(r#"BA_ "BUIntAttribute" BU_ Node0 100;"#),
            Ok((
                "",
                ObjectAttributeValue::Node(NodeAttributeValue {
                    attribute_name: "BUIntAttribute".to_string(),
                    node_name: "Node0".to_string(),
                    attribute_value: AttributeValue::Double(100.0)
                })
            ))
        );
    }

    #[test]
    fn test_parser_object_attribute_value_03() {
        assert_eq!(
            parser_object_attribute_value(
                r#"BA_ "BOStringAttribute" BO_ 1234 "MessageAttribute";"#
            ),
            Ok((
                "",
                ObjectAttributeValue::Message(MessageAttributeValue {
                    attribute_name: "BOStringAttribute".to_string(),
                    message_id: 1234,
                    attribute_value: AttributeValue::String(CharString(
                        "MessageAttribute".to_string()
                    ))
                })
            ))
        );
    }

    #[test]
    fn test_parser_object_attribute_value_04() {
        assert_eq!(
            parser_object_attribute_value(r#"BA_ "SGEnumAttribute" SG_ 1234 Signal0 2;"#),
            Ok((
                "",
                ObjectAttributeValue::Signal(SignalAttributeValue {
                    attribute_name: "SGEnumAttribute".to_string(),
                    message_id: 1234,
                    signal_name: "Signal0".to_string(),
                    attribute_value: AttributeValue::Double(2.0)
                })
            ))
        );
    }

    #[test]
    fn test_parser_object_attribute_value_05() {
        assert_eq!(
            parser_object_attribute_value(r#"BA_ "RWEnvVar_wData_Val" EV_ RWEnvVar_wData 3;"#),
            Ok((
                "",
                ObjectAttributeValue::EnvironmentVariable(EnvironmentVariableAttributeValue {
                    attribute_name: "RWEnvVar_wData_Val".to_string(),
                    env_var_name: "RWEnvVar_wData".to_string(),
                    attribute_value: AttributeValue::Double(3.0)
                })
            ))
        );
    }
}
