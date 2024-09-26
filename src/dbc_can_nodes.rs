use super::dbc_common_parsers::*;
use super::dbc_error::DbcParseError;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

/// List of all CAN-Nodes, seperated by whitespaces.
/// BU_: ABS DRS_MM5_10
#[derive(PartialEq, Debug, Clone)]
pub struct DbcCanNodes(pub Vec<String>);

impl fmt::Display for DbcCanNodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BU_:",)?;
        for node in &self.0 {
            write!(f, " {node}")?;
        }
        Ok(())
    }
}

pub fn dbc_can_nodes(input: &str) -> IResult<&str, DbcCanNodes, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("BU_")),
            spacey(tag(":")),
            many0(spacey(dbc_node_name)),
            many0(line_ending),
        )),
        |(_, _, names, _)| DbcCanNodes(names.into_iter().map(String::from).collect()),
    )(input);
    match res {
        Ok((remain, can_nodes)) => {
            log::info!("parse can nodes: {:?}", can_nodes.0);
            Ok((remain, can_nodes))
        }
        Err(e) => {
            log::trace!("parse can nodes failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadCanNodes))
        }
    }
}

#[test]
fn test_dbc_can_nodes() {
    assert_eq!(
        dbc_can_nodes(
            r#"BU_: ABS DRS_MM5_10

"#
        ),
        Ok(("", DbcCanNodes(vec!["ABS".into(), "DRS_MM5_10".into()]))),
    );

    assert_eq!(
        dbc_can_nodes(r#"BU_:Matrix"#),
        Ok(("", DbcCanNodes(vec!["Matrix".into()]))),
    );

    assert_eq!(
        dbc_can_nodes(r#"BU_: Node2 Node1 Node0"#),
        Ok((
            "",
            DbcCanNodes(vec!["Node2".into(), "Node1".into(), "Node0".into()])
        )),
    );
}
