use super::common_parsers::*;
use super::error::DbcParseError;
use super::value_descriptions::parser_value_descriptions;
use super::value_descriptions::ValueDescriptions;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The value table section defines the global value tables. The value descriptions in
/// value tables define value encodings for signal raw values. In commonly used DBC
/// files the global value tables aren't used, but the value descriptions are defined for
/// each signal independently.
///
/// ```text
/// value_tables = {value_table} ;
/// value_table = 'VAL_TABLE_' value_table_name {value_description} ';' ;
/// value_table_name = DBC_identifier ;
/// ```
///
/// Format: `VAL_TABLE_ <value_table_name> <value_description>;`
///
/// value_description: List of `IntValue "StringValue"` Pairs, seperated by whitespaces
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ValueTable {
    pub name: String,
    pub value_descriptions: ValueDescriptions,
}

impl fmt::Display for ValueTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VAL_TABLE_ {}", self.name)?;
        if !self.value_descriptions.values.is_empty() {
            write!(f, " {}", self.value_descriptions)?;
        }
        write!(f, ";")
    }
}

pub fn parser_value_table_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_object_name(input)
}

pub fn parser_value_table(input: &str) -> IResult<&str, ValueTable, DbcParseError> {
    map(
        tuple((
            multispacey(tag("VAL_TABLE_")),
            spacey(parser_value_table_name),
            spacey(parser_value_descriptions),
            spacey(tag(";")),
            many0(line_ending),
        )),
        |(_, name, values, _, _)| ValueTable {
            name: name.to_string(),
            value_descriptions: values,
        },
    )(input)
}

pub fn parser_value_tables(input: &str) -> IResult<&str, Option<Vec<ValueTable>>, DbcParseError> {
    map(opt(many0(parser_value_table)), |values| match values {
        Some(values) => {
            if values.is_empty() {
                None
            } else {
                Some(values)
            }
        }
        None => None,
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::value_descriptions::ValueDescriptionItem;

    #[test]
    fn test_parser_value_table_01() {
        assert_eq!(
            parser_value_table(
                r#" VAL_TABLE_ ABS_fault_info 2 "active faults stored" 1 "inactive faults stored" 0 "no faults stored" ; "#
            ),
            Ok((
                "",
                ValueTable {
                    name: "ABS_fault_info".to_string(),
                    value_descriptions: ValueDescriptions {
                        values: vec![
                            ValueDescriptionItem {
                                num: 2,
                                str: "active faults stored".to_string()
                            },
                            ValueDescriptionItem {
                                num: 1,
                                str: "inactive faults stored".to_string()
                            },
                            ValueDescriptionItem {
                                num: 0,
                                str: "no faults stored".to_string()
                            }
                        ]
                    }
                }
            )),
        );
    }

    #[test]
    fn test_value_table_string_01() {
        assert_eq!(
            ValueTable {
                name: "ABS_fault_info".to_string(),
                value_descriptions: ValueDescriptions {
                    values: vec![
                        ValueDescriptionItem {
                            num: 2,
                            str: "active faults stored".to_string()
                        },
                        ValueDescriptionItem {
                            num: 1,
                            str: "inactive faults stored".to_string()
                        },
                        ValueDescriptionItem {
                            num: 0,
                            str: "no faults stored".to_string()
                        }
                    ]
                }
            }
            .to_string(),
            "VAL_TABLE_ ABS_fault_info 2 \"active faults stored\" 1 \"inactive faults stored\" 0 \"no faults stored\";"
        );
    }

    #[test]
    fn test_value_table_string_02() {
        assert_eq!(
            ValueTable {
                name: "name".to_string(),
                value_descriptions: ValueDescriptions { values: vec![] }
            }
            .to_string(),
            "VAL_TABLE_ name;"
        );
    }
}
