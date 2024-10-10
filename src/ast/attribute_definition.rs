use super::common_parsers::*;
use super::error::DbcParseError;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeIntegerValueType {
    pub minimum: i32,
    pub maximum: i32,
}

impl fmt::Display for AttributeIntegerValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INT {} {}", self.minimum, self.maximum)
    }
}

pub fn parser_attribute_integer_value_type(
    input: &str,
) -> IResult<&str, AttributeValueType, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("INT")),
            multispacey(signed_integer),
            multispacey(signed_integer),
        )),
        |(_, minimum, maximum)| AttributeIntegerValueType { minimum, maximum },
    )(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute integer value type: {:?}", value);
            Ok((remain, AttributeValueType::Integer(value)))
        }
        Err(e) => {
            log::trace!("parse attribute integer value type failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadAttributeIntegerValueType))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeHexValueType {
    pub minimum: i32,
    pub maximum: i32,
}

impl fmt::Display for AttributeHexValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HEX {} {}", self.minimum, self.maximum)
    }
}

pub fn parser_attribute_hex_value_type(
    input: &str,
) -> IResult<&str, AttributeValueType, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("HEX")),
            multispacey(signed_integer),
            multispacey(signed_integer),
        )),
        |(_, minimum, maximum)| AttributeHexValueType { minimum, maximum },
    )(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute hex value type: {:?}", value);
            Ok((remain, AttributeValueType::Hex(value)))
        }
        Err(e) => {
            log::trace!("parse attribute hex value type failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadAttributeHexValueType))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeFloatValueType {
    pub minimum: f64,
    pub maximum: f64,
}

impl fmt::Display for AttributeFloatValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FLOAT {} {}", self.minimum, self.maximum)
    }
}

pub fn parser_attribute_float_value_type(
    input: &str,
) -> IResult<&str, AttributeValueType, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("FLOAT")),
            multispacey(number_value),
            multispacey(number_value),
        )),
        |(_, minimum, maximum)| AttributeFloatValueType { minimum, maximum },
    )(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute float value type: {:?}", value);
            Ok((remain, AttributeValueType::Float(value)))
        }
        Err(e) => {
            log::trace!("parse attribute float value type failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadAttributeFloatValueType))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeStringValueType {}

impl fmt::Display for AttributeStringValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "STRING")
    }
}

pub fn parser_attribute_string_value_type(
    input: &str,
) -> IResult<&str, AttributeValueType, DbcParseError> {
    let res: Result<(&str, &str), nom::Err<DbcParseError>> = tag("STRING")(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute string value type: {:?}", value);
            Ok((
                remain,
                AttributeValueType::String(AttributeStringValueType {}),
            ))
        }
        Err(e) => {
            log::trace!("parse attribute string value type failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadAttributeStringValueType))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeEnumValueType {
    pub values: Vec<String>,
}

impl fmt::Display for AttributeEnumValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ENUM {}",
            self.values
                .iter()
                .map(|v| format!("\"{v}\""))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

pub fn parser_attribute_enum_value_type(
    input: &str,
) -> IResult<&str, AttributeValueType, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("ENUM")),
            multispacey(separated_list0(tag(","), spacey(char_string))),
        )),
        |(_, values)| AttributeEnumValueType { values },
    )(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute enum value type: {:?}", value);
            Ok((remain, AttributeValueType::Enum(value)))
        }
        Err(e) => {
            log::trace!("parse attribute enum value type failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadAttributeEnumValueType))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AttributeValueType {
    Integer(AttributeIntegerValueType),
    Hex(AttributeHexValueType),
    Float(AttributeFloatValueType),
    String(AttributeStringValueType),
    Enum(AttributeEnumValueType),
}

impl fmt::Display for AttributeValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeValueType::Integer(v) => write!(f, "{}", v),
            AttributeValueType::Hex(v) => write!(f, "{}", v),
            AttributeValueType::Float(v) => write!(f, "{}", v),
            AttributeValueType::String(v) => write!(f, "{}", v),
            AttributeValueType::Enum(v) => write!(f, "{}", v),
        }
    }
}

pub fn parser_attribute_value_type(
    input: &str,
) -> IResult<&str, AttributeValueType, DbcParseError> {
    let res = alt((
        parser_attribute_integer_value_type,
        parser_attribute_hex_value_type,
        parser_attribute_float_value_type,
        parser_attribute_string_value_type,
        parser_attribute_enum_value_type,
    ))(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute value type: {:?}", value);
            Ok((remain, value))
        }
        Err(e) => {
            log::trace!("parse attribute value type failed, e = {:?}", e);
            Err(e)
        }
    }
}

/// example:
///
/// ```text
/// BA_DEF_  "FloatAttribute" FLOAT 0 50.5;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct NetworkAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

impl fmt::Display for NetworkAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_ \"{}\" {};",
            self.attribute_name, self.attribute_value_type
        )
    }
}

/// example:
///
/// ```text
/// BA_DEF_ BU_  "BUIntAttribute" INT 0 100;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct NodeAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

impl fmt::Display for NodeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_ BU_ \"{}\" {};",
            self.attribute_name, self.attribute_value_type
        )
    }
}

/// example:
///
/// ```text
/// BA_DEF_ BO_  "BOStringAttribute" STRING ;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct MessageAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

impl fmt::Display for MessageAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_ BO_ \"{}\" {};",
            self.attribute_name, self.attribute_value_type
        )
    }
}

/// example:
///
/// ```text
/// BA_DEF_ SG_  "SGEnumAttribute" ENUM  "Val0","Val1","Val2";
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct SignalAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

impl fmt::Display for SignalAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_ SG_ \"{}\" {};",
            self.attribute_name, self.attribute_value_type
        )
    }
}

/// example:
///
/// ```text
/// BA_DEF_ EV_  "RWEnvVar_wData_Val" INT 0 10;
/// BA_DEF_ EV_  "GlobalEnvVar_Val" HEX 256 320;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct EnvironmentVariableAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

impl fmt::Display for EnvironmentVariableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_ EV_ \"{}\" {};",
            self.attribute_name, self.attribute_value_type
        )
    }
}

/// Control Unit -- Environment Variable
///
/// example:
///
/// ```text
/// BA_DEF_REL_ BU_EV_REL_  "ControlUnitEnvVarAttr" STRING ;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct ControlUnitEnvironmentVariableAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

impl fmt::Display for ControlUnitEnvironmentVariableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_REL_ BU_EV_REL_ \"{}\" {};",
            self.attribute_name, self.attribute_value_type
        )
    }
}

/// Node -- Tx Message
///
/// example:
///
/// ```text
/// BA_DEF_REL_ BU_BO_REL_  "attribute_name" STRING ;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct NodeTxMessageAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

impl fmt::Display for NodeTxMessageAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_REL_ BU_BO_REL_ \"{}\" {};",
            self.attribute_name, self.attribute_value_type
        )
    }
}

/// Node -- Mapped Rx Signal
///
/// example:
///
/// ```text
/// BA_DEF_REL_ BU_SG_REL_  "attribute_name" STRING ;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct NodeMappedRxSignalAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

impl fmt::Display for NodeMappedRxSignalAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_REL_ BU_SG_REL_ \"{}\" {};",
            self.attribute_name, self.attribute_value_type
        )
    }
}

/// example:
///
/// ```text
/// BA_DEF_ EV_  "RWEnvVar_wData_Val" INT 0 10;
/// BA_DEF_ EV_  "GlobalEnvVar_Val" HEX 256 320;
/// BA_DEF_ SG_  "SGEnumAttribute" ENUM  "Val0","Val1","Val2";
/// BA_DEF_ BU_  "BUIntAttribute" INT 0 100;
/// BA_DEF_ BO_  "BOStringAttribute" STRING ;
/// BA_DEF_  "FloatAttribute" FLOAT 0 50.5;
/// BA_DEF_REL_ BU_EV_REL_  "ControlUnitEnvVarAttr" STRING ;
/// BA_DEF_REL_ BU_BO_REL_  "attribute_name" STRING ;
/// BA_DEF_REL_ BU_SG_REL_  "attribute_name" STRING ;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub enum AttributeDefinition {
    Network(NetworkAttribute),
    Node(NodeAttribute),
    Message(MessageAttribute),
    Signal(SignalAttribute),
    EnvironmentVariable(EnvironmentVariableAttribute),
    ControlUnitEnvironmentVariable(ControlUnitEnvironmentVariableAttribute),
    NodeTxMessage(NodeTxMessageAttribute),
    NodeMappedRxSignal(NodeMappedRxSignalAttribute),
}

impl fmt::Display for AttributeDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeDefinition::Network(v) => write!(f, "{}", v),
            AttributeDefinition::Node(v) => write!(f, "{}", v),
            AttributeDefinition::Message(v) => write!(f, "{}", v),
            AttributeDefinition::Signal(v) => write!(f, "{}", v),
            AttributeDefinition::EnvironmentVariable(v) => write!(f, "{}", v),
            AttributeDefinition::ControlUnitEnvironmentVariable(v) => write!(f, "{}", v),
            AttributeDefinition::NodeTxMessage(v) => write!(f, "{}", v),
            AttributeDefinition::NodeMappedRxSignal(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_definition_string_01() {
        assert_eq!(
            AttributeDefinition::Network(NetworkAttribute {
                attribute_name: "attribute_name".to_string(),
                attribute_value_type: AttributeValueType::Integer(AttributeIntegerValueType {
                    minimum: 0,
                    maximum: 100
                })
            })
            .to_string(),
            r#"BA_DEF_ "attribute_name" INT 0 100;"#
        );
    }

    #[test]
    fn test_attribute_definition_string_02() {
        assert_eq!(
            AttributeDefinition::Network(NetworkAttribute {
                attribute_name: "FloatAttribute".to_string(),
                attribute_value_type: AttributeValueType::Float(AttributeFloatValueType {
                    minimum: 0.0,
                    maximum: 50.5
                })
            })
            .to_string(),
            r#"BA_DEF_ "FloatAttribute" FLOAT 0 50.5;"#
        );
    }

    #[test]
    fn test_attribute_definition_string_03() {
        assert_eq!(
            AttributeDefinition::Node(NodeAttribute {
                attribute_name: "BUIntAttribute".to_string(),
                attribute_value_type: AttributeValueType::Integer(AttributeIntegerValueType {
                    minimum: 0,
                    maximum: 100
                })
            })
            .to_string(),
            r#"BA_DEF_ BU_ "BUIntAttribute" INT 0 100;"#
        );
    }

    #[test]
    fn test_attribute_definition_string_04() {
        assert_eq!(
            AttributeDefinition::Message(MessageAttribute {
                attribute_name: "BOStringAttribute".to_string(),
                attribute_value_type: AttributeValueType::String(AttributeStringValueType {})
            })
            .to_string(),
            r#"BA_DEF_ BO_ "BOStringAttribute" STRING;"#
        );
    }

    #[test]
    fn test_attribute_definition_string_05() {
        assert_eq!(
            AttributeDefinition::Signal(SignalAttribute {
                attribute_name: "SGEnumAttribute".to_string(),
                attribute_value_type: AttributeValueType::Enum(AttributeEnumValueType {
                    values: vec!["Val0".to_string(), "Val1".to_string(), "Val2".to_string()]
                })
            })
            .to_string(),
            r#"BA_DEF_ SG_ "SGEnumAttribute" ENUM "Val0","Val1","Val2";"#
        );
    }

    #[test]
    fn test_attribute_definition_string_06() {
        assert_eq!(
            AttributeDefinition::EnvironmentVariable(EnvironmentVariableAttribute {
                attribute_name: "GlobalEnvVar_Val".to_string(),
                attribute_value_type: AttributeValueType::Hex(AttributeHexValueType {
                    minimum: 256,
                    maximum: 320
                })
            })
            .to_string(),
            r#"BA_DEF_ EV_ "GlobalEnvVar_Val" HEX 256 320;"#
        );
    }

    #[test]
    fn test_attribute_definition_string_07() {
        assert_eq!(
            AttributeDefinition::EnvironmentVariable(EnvironmentVariableAttribute {
                attribute_name: "RWEnvVar_wData_Val".to_string(),
                attribute_value_type: AttributeValueType::Integer(AttributeIntegerValueType {
                    minimum: 0,
                    maximum: 10
                })
            })
            .to_string(),
            r#"BA_DEF_ EV_ "RWEnvVar_wData_Val" INT 0 10;"#
        );
    }

    #[test]
    fn test_attribute_definition_string_08() {
        assert_eq!(
            AttributeDefinition::ControlUnitEnvironmentVariable(
                ControlUnitEnvironmentVariableAttribute {
                    attribute_name: "ControlUnitEnvVarAttr".to_string(),
                    attribute_value_type: AttributeValueType::String(AttributeStringValueType {})
                }
            )
            .to_string(),
            r#"BA_DEF_REL_ BU_EV_REL_ "ControlUnitEnvVarAttr" STRING;"#
        );
    }

    #[test]
    fn test_parser_attribute_integer_value_type_01() {
        assert_eq!(
            parser_attribute_integer_value_type("INT 0 100"),
            Ok((
                "",
                AttributeValueType::Integer(AttributeIntegerValueType {
                    minimum: 0,
                    maximum: 100
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_hex_value_type_01() {
        assert_eq!(
            parser_attribute_hex_value_type("HEX 256 320"),
            Ok((
                "",
                AttributeValueType::Hex(AttributeHexValueType {
                    minimum: 256,
                    maximum: 320
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_float_value_type_01() {
        assert_eq!(
            parser_attribute_float_value_type("FLOAT 0 50.5"),
            Ok((
                "",
                AttributeValueType::Float(AttributeFloatValueType {
                    minimum: 0.0,
                    maximum: 50.5
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_string_value_type_01() {
        assert_eq!(
            parser_attribute_string_value_type("STRING"),
            Ok(("", AttributeValueType::String(AttributeStringValueType {})))
        );
    }

    #[test]
    fn test_parser_attribute_enum_value_type_01() {
        assert_eq!(
            parser_attribute_enum_value_type(r#"ENUM  "Val0","Val1","Val2""#),
            Ok((
                "",
                AttributeValueType::Enum(AttributeEnumValueType {
                    values: vec!["Val0".to_string(), "Val1".to_string(), "Val2".to_string()]
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_value_type_01() {
        assert_eq!(
            parser_attribute_value_type("INT 0 100"),
            Ok((
                "",
                AttributeValueType::Integer(AttributeIntegerValueType {
                    minimum: 0,
                    maximum: 100
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_value_type_02() {
        assert_eq!(
            parser_attribute_value_type("HEX 256 320"),
            Ok((
                "",
                AttributeValueType::Hex(AttributeHexValueType {
                    minimum: 256,
                    maximum: 320
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_value_type_03() {
        assert_eq!(
            parser_attribute_value_type("FLOAT 0 50.5"),
            Ok((
                "",
                AttributeValueType::Float(AttributeFloatValueType {
                    minimum: 0.0,
                    maximum: 50.5
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_value_type_04() {
        assert_eq!(
            parser_attribute_value_type("STRING"),
            Ok(("", AttributeValueType::String(AttributeStringValueType {})))
        );
    }

    #[test]
    fn test_parser_attribute_value_type_05() {
        assert_eq!(
            parser_attribute_value_type(r#"ENUM  "Val0","Val1","Val2""#),
            Ok((
                "",
                AttributeValueType::Enum(AttributeEnumValueType {
                    values: vec!["Val0".to_string(), "Val1".to_string(), "Val2".to_string()]
                })
            ))
        );
    }
}
