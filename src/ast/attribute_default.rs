use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::{IResult, Parser};

use super::attribute::parser_attribute_name;
use super::char_string::{parser_char_string, CharString};
use super::common_parsers::{multispacey, number_value};
use super::error::DbcParseError;

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeValue {
    Double(f64),
    String(CharString),
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeValue::Double(v) => write!(f, "{v}"),
            AttributeValue::String(v) => write!(f, r#""{v}""#),
        }
    }
}

pub fn parser_attribute_value_double(input: &str) -> IResult<&str, AttributeValue, DbcParseError> {
    map(number_value, AttributeValue::Double).parse(input)
}

pub fn parser_attribute_value_string(input: &str) -> IResult<&str, AttributeValue, DbcParseError> {
    map(parser_char_string, AttributeValue::String).parse(input)
}

pub fn parser_attribute_value(input: &str) -> IResult<&str, AttributeValue, DbcParseError> {
    let res = alt((parser_attribute_value_double, parser_attribute_value_string)).parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute value: {value:?}");
            Ok((remain, value))
        }
        Err(e) => {
            log::trace!("parse attribute value failed, e = {e:?}");
            Err(e)
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeDefinitionDefault {
    attribute_name: String,
    attribute_value: AttributeValue,
}

impl fmt::Display for AttributeDefinitionDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"BA_DEF_DEF_ "{}" {};"#,
            self.attribute_name, self.attribute_value
        )
    }
}

pub fn parser_attribute_definition_default(
    input: &str,
) -> IResult<&str, AttributeDefault, DbcParseError> {
    let res = map(
        (
            multispacey(tag("BA_DEF_DEF_")),
            multispacey(parser_attribute_name),
            multispacey(parser_attribute_value),
            multispacey(tag(";")),
        ),
        |(_, attribute_name, attribute_value, _)| AttributeDefinitionDefault {
            attribute_name: attribute_name.to_string(),
            attribute_value,
        },
    )
    .parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute default: {value:?}");
            Ok((remain, AttributeDefault::Attribute(value)))
        }
        Err(e) => {
            log::trace!("parse attribute default failed, e = {e:?}");
            Err(nom::Err::Error(
                DbcParseError::BadAttributeDefinitionDefault,
            ))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RelationAttributeDefinitionDefault {
    attribute_name: String,
    attribute_value: AttributeValue,
}

impl fmt::Display for RelationAttributeDefinitionDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"BA_DEF_DEF_REL_ "{}" {};"#,
            self.attribute_name, self.attribute_value
        )
    }
}

pub fn parser_relation_attribute_definition_default(
    input: &str,
) -> IResult<&str, AttributeDefault, DbcParseError> {
    let res = map(
        (
            multispacey(tag("BA_DEF_DEF_REL_")),
            multispacey(parser_attribute_name),
            multispacey(parser_attribute_value),
            multispacey(tag(";")),
        ),
        |(_, attribute_name, attribute_value, _)| RelationAttributeDefinitionDefault {
            attribute_name: attribute_name.to_string(),
            attribute_value,
        },
    )
    .parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse relation attribute default: {value:?}");
            Ok((remain, AttributeDefault::RelationAttribute(value)))
        }
        Err(e) => {
            log::trace!("parse relation attribute default failed, e = {e:?}");
            Err(nom::Err::Error(
                DbcParseError::BadRelationAttributeDefinitionDefault,
            ))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeDefault {
    Attribute(AttributeDefinitionDefault),
    RelationAttribute(RelationAttributeDefinitionDefault),
}

impl fmt::Display for AttributeDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeDefault::Attribute(v) => write!(f, "{v}"),
            AttributeDefault::RelationAttribute(v) => write!(f, "{v}"),
        }
    }
}

pub fn parser_attribute_default(input: &str) -> IResult<&str, AttributeDefault, DbcParseError> {
    let res = alt((
        parser_attribute_definition_default,
        parser_relation_attribute_definition_default,
    ))
    .parse(input);

    match res {
        Ok((remain, value)) => {
            log::info!("parse attribute default: {value:?}");
            Ok((remain, value))
        }
        Err(e) => {
            log::trace!("parse attribute default failed, e = {e:?}");
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_default_string_01() {
        assert_eq!(
            AttributeDefault::Attribute(AttributeDefinitionDefault {
                attribute_name: "attribute_name".to_string(),
                attribute_value: AttributeValue::Double(0.0)
            })
            .to_string(),
            r#"BA_DEF_DEF_ "attribute_name" 0;"#
        );
    }

    #[test]
    fn test_attribute_default_string_02() {
        assert_eq!(
            AttributeDefault::Attribute(AttributeDefinitionDefault {
                attribute_name: "SGEnumAttribute".to_string(),
                attribute_value: AttributeValue::String(CharString("Val0".to_string()))
            })
            .to_string(),
            r#"BA_DEF_DEF_ "SGEnumAttribute" "Val0";"#
        );
    }

    #[test]
    fn test_attribute_default_string_03() {
        assert_eq!(
            AttributeDefault::Attribute(AttributeDefinitionDefault {
                attribute_name: "GlobalEnvVar_Val".to_string(),
                attribute_value: AttributeValue::Double(288.0)
            })
            .to_string(),
            r#"BA_DEF_DEF_ "GlobalEnvVar_Val" 288;"#
        );
    }

    #[test]
    fn test_attribute_default_string_04() {
        assert_eq!(
            AttributeDefault::Attribute(AttributeDefinitionDefault {
                attribute_name: "FloatAttribute".to_string(),
                attribute_value: AttributeValue::Double(25.25)
            })
            .to_string(),
            r#"BA_DEF_DEF_ "FloatAttribute" 25.25;"#
        );
    }

    #[test]
    fn test_attribute_default_string_05() {
        assert_eq!(
            AttributeDefault::RelationAttribute(RelationAttributeDefinitionDefault {
                attribute_name: "ControlUnitEnvVarAttr".to_string(),
                attribute_value: AttributeValue::String(CharString("MyVar".to_string()))
            })
            .to_string(),
            r#"BA_DEF_DEF_REL_ "ControlUnitEnvVarAttr" "MyVar";"#
        );
    }

    #[test]
    fn test_parser_attribute_definition_default_01() {
        assert_eq!(
            parser_attribute_definition_default(r#"BA_DEF_DEF_ "attribute_name" 0;"#),
            Ok((
                "",
                AttributeDefault::Attribute(AttributeDefinitionDefault {
                    attribute_name: "attribute_name".to_string(),
                    attribute_value: AttributeValue::Double(0.0)
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_default_01() {
        assert_eq!(
            parser_attribute_default(r#"BA_DEF_DEF_ "attribute_name" 0;"#),
            Ok((
                "",
                AttributeDefault::Attribute(AttributeDefinitionDefault {
                    attribute_name: "attribute_name".to_string(),
                    attribute_value: AttributeValue::Double(0.0)
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_default_02() {
        assert_eq!(
            parser_attribute_default(r#"BA_DEF_DEF_REL_ "ControlUnitEnvVarAttr" "MyVar";"#),
            Ok((
                "",
                AttributeDefault::RelationAttribute(RelationAttributeDefinitionDefault {
                    attribute_name: "ControlUnitEnvVarAttr".to_string(),
                    attribute_value: AttributeValue::String(CharString("MyVar".to_string()))
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_default_03() {
        assert_eq!(
            parser_attribute_default(r#"BA_DEF_DEF_  "RWEnvVar_wData_Val" 5;"#),
            Ok((
                "",
                AttributeDefault::Attribute(AttributeDefinitionDefault {
                    attribute_name: "RWEnvVar_wData_Val".to_string(),
                    attribute_value: AttributeValue::Double(5.0)
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_default_04() {
        assert_eq!(
            parser_attribute_default(r#"BA_DEF_DEF_  "GlobalEnvVar_Val" 288;"#),
            Ok((
                "",
                AttributeDefault::Attribute(AttributeDefinitionDefault {
                    attribute_name: "GlobalEnvVar_Val".to_string(),
                    attribute_value: AttributeValue::Double(288.0)
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_default_05() {
        assert_eq!(
            parser_attribute_default(r#"BA_DEF_DEF_  "SGEnumAttribute" "Val0";"#),
            Ok((
                "",
                AttributeDefault::Attribute(AttributeDefinitionDefault {
                    attribute_name: "SGEnumAttribute".to_string(),
                    attribute_value: AttributeValue::String(CharString("Val0".to_string()))
                })
            ))
        );
    }

    #[test]
    fn test_parser_attribute_default_06() {
        assert_eq!(
            parser_attribute_default(r#"BA_DEF_DEF_  "FloatAttribute" 25.25;"#),
            Ok((
                "",
                AttributeDefault::Attribute(AttributeDefinitionDefault {
                    attribute_name: "FloatAttribute".to_string(),
                    attribute_value: AttributeValue::Double(25.25)
                })
            ))
        );
    }
}
