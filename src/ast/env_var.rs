use super::common_parsers::*;
use super::error::DbcParseError;
use nom::bytes::complete::tag;
use nom::character::complete::hex_digit1;
use nom::character::complete::line_ending;
use nom::character::complete::u32;
use nom::combinator::map;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum EnvVarType {
    Integer,
    Float,
    String,
    Data,
}

impl EnvVarType {
    pub fn from_char(c: char) -> Option<EnvVarType> {
        match c {
            '0' => Some(EnvVarType::Integer),
            '1' => Some(EnvVarType::Float),
            's' => Some(EnvVarType::String),
            'd' => Some(EnvVarType::Data),
            _ => None,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            EnvVarType::Integer => '0',
            EnvVarType::Float => '1',
            EnvVarType::String => 's',
            EnvVarType::Data => 'd',
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum EnvVarAccessType {
    Unrestricted = 0x0000,
    Read = 0x0001,
    Write = 0x0002,
    ReadWrite = 0x0003,
}

/// Environment variable
///
/// ```text
/// environment_variables = {environment_variable}
/// environment_variable = 'EV_' env_var_name ':' env_var_type '[' mini-
/// mum '|' maximum ']' unit initial_value ev_id access_type
/// access_node {',' access_node } ';' ;
///
/// env_var_name = DBC_identifier ;
///
/// env_var_type = '0' | '1' | '2' ; (* 0=integer, 1=float, 2=string *)
///
/// minimum = double ;
/// maximum = double ;
///
/// initial_value = double ;
///
/// ev_id = unsigned_integer ; (* obsolete *)
///
/// access_type = 'DUMMY_NODE_VECTOR0' | 'DUMMY_NODE_VECTOR1' |
///     'DUMMY_NODE_VECTOR2' | 'DUMMY_NODE_VECTOR3' |
///     'DUMMY_NODE_VECTOR8000' | 'DUMMY_NODE_VECTOR8001' |
///     'DUMMY_NODE_VECTOR8002' | 'DUMMY_NODE_VECTOR8003'; (*
///     0=unrestricted, 1=read, 2=write, 3=readWrite, if the value be-
///     hind 'DUMMY_NODE_VECTOR' is OR-ed with 0x8000, the value type
///     is always string. *)
///
/// access_node = node_name | 'VECTOR__XXX' ;
/// ```
///
/// example:
///
/// ```text
/// EV_ UnrestrictedEnvVar: 0 [0|0] "Nm" 0 1 DUMMY_NODE_VECTOR8000  Node0;
/// EV_ RWEnvVar_wData: 0 [0|1234] "" 60 2 DUMMY_NODE_VECTOR3  Node2;
/// EV_ WriteOnlyEnvVar: 1 [0|1234] "" 60 3 DUMMY_NODE_VECTOR2  Node2;
/// ```
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    pub env_var_name: String,
    pub env_var_type: EnvVarType,
    pub minimum: f64,
    pub maximum: f64,
    pub unit: String,
    pub initial_value: f64,
    pub ev_id: u32,
    pub access_type: u16,
    pub access_nodes: Vec<String>,
}

impl fmt::Display for EnvironmentVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EV_ {}: ", self.env_var_name)?;
        match self.env_var_type {
            EnvVarType::Integer => write!(f, "0")?,
            EnvVarType::Float => write!(f, "1")?,
            EnvVarType::String => write!(f, "0")?,
            EnvVarType::Data => write!(f, "0")?,
        }
        write!(f, " [{}|{}]", self.minimum, self.maximum)?;
        write!(f, " \"{}\" ", self.unit)?;
        write!(f, "{} ", self.initial_value)?;
        write!(f, "{} ", self.ev_id)?;
        write!(f, "DUMMY_NODE_VECTOR")?;
        if self.env_var_type == EnvVarType::String {
            write!(f, "{:X}", self.access_type.clone() as u16 | 0x8000)?;
        } else {
            write!(f, "{:X}", self.access_type.clone() as u16)?;
        }
        write!(f, " ")?;
        if self.access_nodes.is_empty() {
            write!(f, "Vector__XXX")?;
        } else {
            write!(f, "{}", self.access_nodes.join(","))?;
        }
        write!(f, ";")
    }
}

pub fn parser_env_var_type(input: &str) -> IResult<&str, u32, DbcParseError> {
    u32(input)
}

pub fn parser_minimum(input: &str) -> IResult<&str, f64, DbcParseError> {
    number_value(input)
}

pub fn parser_maximum(input: &str) -> IResult<&str, f64, DbcParseError> {
    number_value(input)
}

pub fn parser_unit(input: &str) -> IResult<&str, String, DbcParseError> {
    char_string(input)
}

pub fn parser_initial_value(input: &str) -> IResult<&str, f64, DbcParseError> {
    number_value(input)
}

pub fn parser_env_id(input: &str) -> IResult<&str, u32, DbcParseError> {
    u32(input)
}

pub fn parser_access_type(input: &str) -> IResult<&str, &str, DbcParseError> {
    hex_digit1(input)
}

pub fn parser_env_var(input: &str) -> IResult<&str, EnvironmentVariable, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("EV_")),
            spacey(parser_env_var_name),
            spacey(tag(":")),
            spacey(parser_env_var_type),
            spacey(tag("[")),
            spacey(parser_minimum),
            spacey(tag("|")),
            spacey(parser_maximum),
            spacey(tag("]")),
            spacey(parser_unit),
            spacey(parser_initial_value),
            spacey(parser_env_id),
            spacey(pair(tag("DUMMY_NODE_VECTOR"), parser_access_type)),
            spacey(separated_list0(tag(","), spacey(parser_node_name))),
            spacey(tag(";")),
            many0(line_ending),
        )),
        |(
            _,
            env_var_name,
            _,
            env_var_type,
            _,
            minimum,
            _,
            maximum,
            _,
            unit,
            initial_value,
            ev_id,
            (_, access_type),
            access_nodes,
            _,
            _,
        )| {
            let mut env_var_type = if env_var_type == 0 {
                EnvVarType::Integer
            } else {
                EnvVarType::Float
            };
            let access_type = u16::from_str_radix(access_type, 16).expect("invalid access type");

            if access_type & 0x8000 != 0 {
                env_var_type = EnvVarType::String;
            }

            EnvironmentVariable {
                env_var_name: env_var_name.to_string(),
                env_var_type,
                minimum,
                maximum,
                unit: unit.to_string(),
                initial_value,
                ev_id,
                access_type,
                access_nodes: access_nodes.iter().map(|s| s.to_string()).collect(),
            }
        },
    )(input);

    match res {
        Ok((remain, val)) => Ok((remain, val)),
        Err(e) => {
            log::trace!("parse environment variable failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadEnvironmentVariable))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_env_var_01() {
        assert_eq!(
            parser_env_var(r#"EV_ RWEnvVar_wData: 0 [0|1234] "" 60 2 DUMMY_NODE_VECTOR3  Node2;"#),
            Ok((
                "",
                EnvironmentVariable {
                    env_var_name: "RWEnvVar_wData".to_string(),
                    env_var_type: EnvVarType::Integer,
                    minimum: 0.0,
                    maximum: 1234.0,
                    unit: "".to_string(),
                    initial_value: 60.0,
                    ev_id: 2,
                    access_type: 3,
                    access_nodes: vec!["Node2".to_string()],
                }
            ))
        )
    }

    #[test]
    fn test_parser_env_var_02() {
        assert_eq!(
            parser_env_var(r#"EV_ WriteOnlyEnvVar: 1 [0|1234] "" 60 3 DUMMY_NODE_VECTOR2  Node2;"#),
            Ok((
                "",
                EnvironmentVariable {
                    env_var_name: "WriteOnlyEnvVar".to_string(),
                    env_var_type: EnvVarType::Float,
                    minimum: 0.0,
                    maximum: 1234.0,
                    unit: "".to_string(),
                    initial_value: 60.0,
                    ev_id: 3,
                    access_type: 2,
                    access_nodes: vec!["Node2".to_string()],
                }
            ))
        )
    }

    #[test]
    fn test_parser_env_var_03() {
        assert_eq!(
            parser_env_var(
                r#"EV_ UnrestrictedEnvVar: 0 [0|0] "Nm" 0 1 DUMMY_NODE_VECTOR8000  Node0;"#
            ),
            Ok((
                "",
                EnvironmentVariable {
                    env_var_name: "UnrestrictedEnvVar".to_string(),
                    env_var_type: EnvVarType::String,
                    minimum: 0.0,
                    maximum: 0.0,
                    unit: "Nm".to_string(),
                    initial_value: 0.0,
                    ev_id: 1,
                    access_type: 0x8000,
                    access_nodes: vec!["Node0".to_string()],
                }
            ))
        )
    }

    #[test]
    fn test_environment_variable_string_01() {
        assert_eq!(
            EnvironmentVariable {
                env_var_name: "RWEnvVar_wData".to_string(),
                env_var_type: EnvVarType::Integer,
                minimum: 0.0,
                maximum: 1234.0,
                unit: "".to_string(),
                initial_value: 60.0,
                ev_id: 2,
                access_type: 3,
                access_nodes: vec!["Node2".to_string()],
            }
            .to_string(),
            r#"EV_ RWEnvVar_wData: 0 [0|1234] "" 60 2 DUMMY_NODE_VECTOR3 Node2;"#
        );
    }

    #[test]
    fn test_environment_variable_string_02() {
        assert_eq!(
            EnvironmentVariable {
                env_var_name: "UnrestrictedEnvVar".to_string(),
                env_var_type: EnvVarType::String,
                minimum: 0.0,
                maximum: 0.0,
                unit: "Nm".to_string(),
                initial_value: 0.0,
                ev_id: 1,
                access_type: 0x8000,
                access_nodes: vec!["Node0".to_string()],
            }
            .to_string(),
            r#"EV_ UnrestrictedEnvVar: 0 [0|0] "Nm" 0 1 DUMMY_NODE_VECTOR8000 Node0;"#
        );
    }

    #[test]
    fn test_environment_variable_string_03() {
        assert_eq!(
            EnvironmentVariable {
                env_var_name: "WriteOnlyEnvVar".to_string(),
                env_var_type: EnvVarType::Float,
                minimum: 0.0,
                maximum: 1234.0,
                unit: "".to_string(),
                initial_value: 60.0,
                ev_id: 3,
                access_type: 2,
                access_nodes: vec!["Node2".to_string()],
            }
            .to_string(),
            r#"EV_ WriteOnlyEnvVar: 1 [0|1234] "" 60 3 DUMMY_NODE_VECTOR2 Node2;"#
        );
    }

    #[test]
    fn test_environment_variable_string_04() {
        assert_eq!(
            EnvironmentVariable {
                env_var_name: "WriteOnlyEnvVar".to_string(),
                env_var_type: EnvVarType::Float,
                minimum: 0.0,
                maximum: 1234.0,
                unit: "".to_string(),
                initial_value: 60.0,
                ev_id: 3,
                access_type: 2,
                access_nodes: vec!["Node2".to_string(), "Node3".to_string()],
            }
            .to_string(),
            r#"EV_ WriteOnlyEnvVar: 1 [0|1234] "" 60 3 DUMMY_NODE_VECTOR2 Node2,Node3;"#
        );
    }
}
