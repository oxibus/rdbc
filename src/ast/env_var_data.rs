use super::common_parsers::*;
use super::error::DbcParseError;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::u32;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

/// Environment variables data section
///
/// ```text
/// environment_variables_data = environment_variable_data ;
/// environment_variable_data = 'ENVVAR_DATA_' env_var_name ':' data_size ';' ;
/// data_size = unsigned_integer ;
/// ```
///
/// example:
///
/// ```text
/// ENVVAR_DATA_ RWEnvVar_wData: 10;
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct EnvironmentVariableData {
    pub env_var_name: String,
    pub data_size: u32,
}

impl fmt::Display for EnvironmentVariableData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ENVVAR_DATA_ {}: {};", self.env_var_name, self.data_size)
    }
}

fn parser_data_size(input: &str) -> IResult<&str, u32, DbcParseError> {
    u32(input)
}

pub fn parser_env_var_data(input: &str) -> IResult<&str, EnvironmentVariableData, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("ENVVAR_DATA_")),
            spacey(parser_env_var_name),
            spacey(tag(":")),
            spacey(parser_data_size),
            spacey(tag(";")),
            many0(line_ending),
        )),
        |(_, env_var_name, _, data_size, _, _)| EnvironmentVariableData {
            env_var_name: env_var_name.to_string(),
            data_size,
        },
    )(input);

    match res {
        Ok((remain, val)) => Ok((remain, val)),
        Err(e) => {
            log::trace!("parse environment variable data failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadEnvironmentVariableData))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_variable_data_string_01() {
        assert_eq!(
            EnvironmentVariableData {
                env_var_name: "RWEnvVar_wData".to_string(),
                data_size: 10
            }
            .to_string(),
            "ENVVAR_DATA_ RWEnvVar_wData: 10;"
        );
    }

    #[test]
    fn test_parser_environment_variable_data_01() {
        assert_eq!(
            parser_env_var_data("ENVVAR_DATA_ RWEnvVar_wData: 10;"),
            Ok((
                "",
                EnvironmentVariableData {
                    env_var_name: "RWEnvVar_wData".to_string(),
                    data_size: 10
                }
            ))
        );
    }
}
