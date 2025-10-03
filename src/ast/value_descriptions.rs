use std::fmt;

use nom::character::complete::i64;
use nom::combinator::map;
use nom::multi::many0;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

use super::char_string::{parser_char_string, CharString};
use super::common_parsers::spacey;
use super::error::DbcParseError;

/// A value description defines a textual description for a single value. This value may
/// either be a signal raw value transferred on the bus or the value of an environment
/// variable in a remaining bus simulation.
///
/// ```text
/// value_description = unsigned_integer char_string ;
/// ```
///
/// example:
///
/// ```text
/// VAL_TABLE_ ABS_fault_info 2 "active faults stored" 1 "inactive faults stored" 0 "no faults stored" ;
/// VAL_TABLE_ vt_WheelSpeedQualifier 5 "InvalidUnderVoltage" 4 "NotCalculated" 3 "ReducedMonitored" 2 "Faulty" 1 "Normal" 0 "NotInitialized" ;
///
/// VAL_ message_id signal_name [value_descriptions];
/// VAL_ 2147487969 Value1 3 "Three" 2 "Two" 1 "One" 0 "Zero" ;
/// VAL_ 2147487969 Value0 2 "Value2" 1 "Value1" 0 "Value0" ;
///
/// VAL_ env_var_name [value_descriptions];
/// VAL_ RWEnvVar_wData 2 "Value2" 1 "Value1" 0 "Value0" ;
/// VAL_ WriteOnlyEnvVar 2 "Value2" 1 "Value1" 0 "Value0" ;
/// VAL_ ReadOnlyEnvVar 2 "Value2" 1 "Value1" 0 "Value0" ;
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ValueDescriptionItem {
    pub num: i64,
    pub str: CharString,
}

impl fmt::Display for ValueDescriptionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{} "{}""#, self.num, self.str)
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ValueDescriptions {
    pub values: Vec<ValueDescriptionItem>,
}

impl fmt::Display for ValueDescriptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{item}")?;
        }
        Ok(())
    }
}

pub fn parser_value_description_item(
    input: &str,
) -> IResult<&str, ValueDescriptionItem, DbcParseError> {
    map((spacey(i64), spacey(parser_char_string)), |(num, str)| {
        ValueDescriptionItem { num, str }
    })
    .parse(input)
}

pub fn parser_value_descriptions(input: &str) -> IResult<&str, ValueDescriptions, DbcParseError> {
    map(many0(spacey(parser_value_description_item)), |values| {
        ValueDescriptions { values }
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_value_description_item_01() {
        assert_eq!(
            parser_value_description_item(r#"2 "active faults stored""#),
            Ok((
                "",
                ValueDescriptionItem {
                    num: 2,
                    str: CharString("active faults stored".to_string())
                }
            )),
        );
    }

    #[test]
    fn test_parser_value_descriptions_01() {
        assert_eq!(
            parser_value_descriptions(r#"2 "active faults stored""#),
            Ok((
                "",
                ValueDescriptions {
                    values: vec![ValueDescriptionItem {
                        num: 2,
                        str: CharString("active faults stored".to_string())
                    }]
                }
            )),
        );
    }

    #[test]
    fn test_parser_value_descriptions_02() {
        assert_eq!(
            parser_value_descriptions(
                r#" 2 "active faults stored" 1 "inactive faults stored" 0 "no faults stored" "#
            ),
            Ok((
                "",
                ValueDescriptions {
                    values: vec![
                        ValueDescriptionItem {
                            num: 2,
                            str: CharString("active faults stored".to_string())
                        },
                        ValueDescriptionItem {
                            num: 1,
                            str: CharString("inactive faults stored".to_string())
                        },
                        ValueDescriptionItem {
                            num: 0,
                            str: CharString("no faults stored".to_string())
                        }
                    ]
                }
            )),
        );
    }

    #[test]
    fn test_value_description_string_01() {
        assert_eq!(
            ValueDescriptionItem {
                num: 2,
                str: CharString("active faults stored".to_string())
            }
            .to_string(),
            r#"2 "active faults stored""#
        );
    }

    #[test]
    fn test_value_descriptions_string_01() {
        assert_eq!(
            ValueDescriptions {
                values: vec![
                    ValueDescriptionItem {
                        num: 2,
                        str: CharString("active faults stored".to_string())
                    },
                    ValueDescriptionItem {
                        num: 1,
                        str: CharString("inactive faults stored".to_string())
                    },
                    ValueDescriptionItem {
                        num: 0,
                        str: CharString("no faults stored".to_string())
                    }
                ]
            }
            .to_string(),
            r#"2 "active faults stored" 1 "inactive faults stored" 0 "no faults stored""#
        );
    }

    #[test]
    fn test_value_descriptions_string_02() {
        assert_eq!(ValueDescriptions { values: vec![] }.to_string(), "");
    }
}
