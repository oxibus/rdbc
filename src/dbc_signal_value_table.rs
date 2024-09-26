use super::dbc_common_parsers::*;
use super::dbc_error::DbcParseError;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

/// VAL_TABLE_ ABS_fault_info 2 "active faults stored" 1 "inactive faults stored" 0 "no faults stored" ;
/// VAL_TABLE_ vt_WheelSpeedQualifier 5 "InvalidUnderVoltage" 4 "NotCalculated" 3 "ReducedMonitored" 2 "Faulty" 1 "Normal" 0 "NotInitialized" ;
#[derive(PartialEq, Debug, Clone)]
pub struct DbcSignalValueTableListItem {
    pub num: i64,
    pub str: String,
}

impl fmt::Display for DbcSignalValueTableListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \"{}\"", self.num, self.str)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct DbcSignalValueTableList {
    pub values: Vec<DbcSignalValueTableListItem>,
}

impl fmt::Display for DbcSignalValueTableList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

/// Value table definition for signals.
/// Format: `VAL_TABLE_ <ValueTableName> <ValueTableDefinition>;`
/// ValueTableDefinition: List of `IntValue "StringValue"` Pairs, seperated by whitespaces
#[derive(PartialEq, Debug, Clone)]
pub struct DbcSignalValueTable {
    pub name: String,
    pub values: DbcSignalValueTableList,
}

impl fmt::Display for DbcSignalValueTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VAL_TABLE_ {} {};", self.name, self.values)
    }
}

fn dbc_signal_value_table_list_item(
    input: &str,
) -> IResult<&str, DbcSignalValueTableListItem, DbcParseError> {
    map(
        tuple((spacey(integer_value), spacey(string_literal))),
        |(num, str)| DbcSignalValueTableListItem { num, str },
    )(input)
}

fn dbc_signal_value_table_list(
    input: &str,
) -> IResult<&str, DbcSignalValueTableList, DbcParseError> {
    map(many0(spacey(dbc_signal_value_table_list_item)), |values| {
        DbcSignalValueTableList { values }
    })(input)
}

pub fn dbc_signal_value_table_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_object_name(input)
}

pub fn dbc_signal_value_table(input: &str) -> IResult<&str, DbcSignalValueTable, DbcParseError> {
    map(
        tuple((
            multispacey(tag("VAL_TABLE_")),
            spacey(dbc_signal_value_table_name),
            spacey(dbc_signal_value_table_list),
            spacey(tag(";")),
            many0(line_ending),
        )),
        |(_, name, values, _, _)| DbcSignalValueTable {
            name: name.to_string(),
            values,
        },
    )(input)
}

pub fn dbc_signal_value_tables(
    input: &str,
) -> IResult<&str, Option<Vec<DbcSignalValueTable>>, DbcParseError> {
    map(opt(many0(dbc_signal_value_table)), |values| match values {
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

#[test]
fn test_dbc_signal_value_table_list_item_01() {
    assert_eq!(
        dbc_signal_value_table_list_item(r#"2 "active faults stored""#),
        Ok((
            "",
            DbcSignalValueTableListItem {
                num: 2,
                str: "active faults stored".to_string()
            }
        )),
    );
}

#[test]
fn test_dbc_signal_value_table_list_01() {
    assert_eq!(
        dbc_signal_value_table_list(r#"2 "active faults stored""#),
        Ok((
            "",
            DbcSignalValueTableList {
                values: vec![DbcSignalValueTableListItem {
                    num: 2,
                    str: "active faults stored".to_string()
                }]
            }
        )),
    );
}

#[test]
fn test_dbc_signal_value_table_list_02() {
    assert_eq!(
        dbc_signal_value_table_list(
            r#" 2 "active faults stored" 1 "inactive faults stored" 0 "no faults stored" "#
        ),
        Ok((
            "",
            DbcSignalValueTableList {
                values: vec![
                    DbcSignalValueTableListItem {
                        num: 2,
                        str: "active faults stored".to_string()
                    },
                    DbcSignalValueTableListItem {
                        num: 1,
                        str: "inactive faults stored".to_string()
                    },
                    DbcSignalValueTableListItem {
                        num: 0,
                        str: "no faults stored".to_string()
                    }
                ]
            }
        )),
    );
}

#[test]
fn test_dbc_signal_value_table_01() {
    assert_eq!(
        dbc_signal_value_table(
            r#" VAL_TABLE_ ABS_fault_info 2 "active faults stored" 1 "inactive faults stored" 0 "no faults stored" ; "#
        ),
        Ok((
            "",
            DbcSignalValueTable {
                name: "ABS_fault_info".to_string(),
                values: DbcSignalValueTableList {
                    values: vec![
                        DbcSignalValueTableListItem {
                            num: 2,
                            str: "active faults stored".to_string()
                        },
                        DbcSignalValueTableListItem {
                            num: 1,
                            str: "inactive faults stored".to_string()
                        },
                        DbcSignalValueTableListItem {
                            num: 0,
                            str: "no faults stored".to_string()
                        }
                    ]
                }
            }
        )),
    );
}
