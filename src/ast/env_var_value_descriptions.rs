use super::common_parsers::*;
use super::error::DbcParseError;
use super::value_descriptions::parser_value_descriptions;
use super::value_descriptions::ValueDescriptions;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use serde::{Deserialize, Serialize};
use std::fmt;

/// VAL_ env_var_name [value_descriptions];
/// VAL_ RWEnvVar_wData 2 "Value2" 1 "Value1" 0 "Value0" ;
/// VAL_ WriteOnlyEnvVar 2 "Value2" 1 "Value1" 0 "Value0" ;
/// VAL_ ReadOnlyEnvVar 2 "Value2" 1 "Value1" 0 "Value0" ;
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentVariableValueDescriptions {
    pub env_var_name: String,
    pub value_descriptions: ValueDescriptions,
}

impl fmt::Display for EnvironmentVariableValueDescriptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VAL_ {} {};", self.env_var_name, self.value_descriptions)
    }
}

pub fn parser_env_var_value_descriptions(
    input: &str,
) -> IResult<&str, EnvironmentVariableValueDescriptions, DbcParseError> {
    let res = map(
        (
            multispacey(tag("VAL_")),
            spacey(parser_env_var_name),
            spacey(parser_value_descriptions),
            spacey(tag(";")),
            many0(line_ending),
        ),
        |(_, env_var_name, value_descriptions, _, _)| EnvironmentVariableValueDescriptions {
            env_var_name: env_var_name.to_string(),
            value_descriptions,
        },
    )
    .parse(input);

    match res {
        Ok((remain, val)) => Ok((remain, val)),
        Err(e) => {
            log::trace!(
                "parse environment variable value descriptions failed, e = {:?}",
                e
            );
            Err(nom::Err::Error(
                DbcParseError::BadEnvironmentVariableValueDescriptions,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::char_string::CharString;
    use crate::ast::value_descriptions::ValueDescriptionItem;

    #[test]
    fn test_parser_environment_variable_value_descriptions_01() {
        assert_eq!(
            parser_env_var_value_descriptions(
                r#"VAL_ RWEnvVar_wData 2 "Value2" 1 "Value1" 0 "Value0" ;"#
            ),
            Ok((
                "",
                EnvironmentVariableValueDescriptions {
                    env_var_name: "RWEnvVar_wData".to_string(),
                    value_descriptions: ValueDescriptions {
                        values: vec![
                            ValueDescriptionItem {
                                num: 2,
                                str: CharString("Value2".to_string())
                            },
                            ValueDescriptionItem {
                                num: 1,
                                str: CharString("Value1".to_string())
                            },
                            ValueDescriptionItem {
                                num: 0,
                                str: CharString("Value0".to_string())
                            }
                        ]
                    }
                }
            ))
        );
    }

    #[test]
    fn test_environment_variable_value_descriptions_string_01() {
        assert_eq!(
            EnvironmentVariableValueDescriptions {
                env_var_name: "RWEnvVar_wData".to_string(),
                value_descriptions: ValueDescriptions {
                    values: vec![
                        ValueDescriptionItem {
                            num: 2,
                            str: CharString("Value2".to_string())
                        },
                        ValueDescriptionItem {
                            num: 1,
                            str: CharString("Value1".to_string())
                        },
                        ValueDescriptionItem {
                            num: 0,
                            str: CharString("Value0".to_string())
                        }
                    ]
                }
            }
            .to_string(),
            r#"VAL_ RWEnvVar_wData 2 "Value2" 1 "Value1" 0 "Value0";"#,
        );
    }
}
