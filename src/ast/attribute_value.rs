use super::attribute_default::AttributeValue;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
                attribute_value: AttributeValue::String("MessageAttribute".to_string())
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
}
