use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum AttributeValue {
    UnsignedInteger(u32),
    SignedInteger(u32),
    Double(f64),
    String(String),
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeValue::UnsignedInteger(v) => write!(f, "{}", v),
            AttributeValue::SignedInteger(v) => write!(f, "{}", v),
            AttributeValue::Double(v) => write!(f, "{}", v),
            AttributeValue::String(v) => write!(f, "\"{}\"", v),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeDefinitionDefault {
    attribute_name: String,
    attribute_value: AttributeValue,
}

impl fmt::Display for AttributeDefinitionDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_DEF_ \"{}\" {};",
            self.attribute_name, self.attribute_value
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct RelationAttributeDefinitionDefault {
    attribute_name: String,
    attribute_value: AttributeValue,
}

impl fmt::Display for RelationAttributeDefinitionDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BA_DEF_DEF_REL_ \"{}\" {};",
            self.attribute_name, self.attribute_value
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AttributeDefault {
    Attribute(AttributeDefinitionDefault),
    RelationAttribute(RelationAttributeDefinitionDefault),
}

impl fmt::Display for AttributeDefault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeDefault::Attribute(v) => write!(f, "{}", v),
            AttributeDefault::RelationAttribute(v) => write!(f, "{}", v),
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
                attribute_value: AttributeValue::String("Val0".to_string())
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
                attribute_value: AttributeValue::SignedInteger(288)
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
                attribute_value: AttributeValue::String("MyVar".to_string())
            })
            .to_string(),
            r#"BA_DEF_DEF_REL_ "ControlUnitEnvVarAttr" "MyVar";"#
        );
    }
}
