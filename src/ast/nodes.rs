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
///
/// The node section defines the names of all participating nodes. The names defined
/// in this section have to be unique within this section.
///
/// ```text
/// nodes = 'BU_:' {node_name} ;
/// node_name = DBC_identifier ;
/// ```
///
/// example:
///
/// ```text
/// BU_: ABS DRS_MM5_10
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Nodes(pub Vec<String>);

impl fmt::Display for Nodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BU_:",)?;
        for node in &self.0 {
            write!(f, " {node}")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

pub fn parser_nodes(input: &str) -> IResult<&str, Nodes, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("BU_")),
            spacey(tag(":")),
            many0(spacey(dbc_node_name)),
            many0(line_ending),
        )),
        |(_, _, names, _)| Nodes(names.into_iter().map(String::from).collect()),
    )(input);
    match res {
        Ok((remain, can_nodes)) => {
            log::info!("parse nodes: {:?}", can_nodes.0);
            Ok((remain, can_nodes))
        }
        Err(e) => {
            log::trace!("parse nodes failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadCanNodes))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbc_can_nodes_01() {
        assert_eq!(
            parser_nodes(
                r#"BU_: ABS DRS_MM5_10

"#
            ),
            Ok(("", Nodes(vec!["ABS".into(), "DRS_MM5_10".into()]))),
        );
    }

    #[test]
    fn test_dbc_can_nodes_02() {
        assert_eq!(
            parser_nodes(r#"BU_:Matrix"#),
            Ok(("", Nodes(vec!["Matrix".into()]))),
        );
    }

    #[test]
    fn test_dbc_can_nodes_03() {
        assert_eq!(
            parser_nodes(r#"BU_: Node2 Node1 Node0"#),
            Ok((
                "",
                Nodes(vec!["Node2".into(), "Node1".into(), "Node0".into()])
            )),
        );
    }

    #[test]
    fn test_nodes_string_01() {
        assert_eq!(
            Nodes(vec!["ABS".into(), "DRS_MM5_10".into()]).to_string(),
            "BU_: ABS DRS_MM5_10\n",
        );
    }

    #[test]
    fn test_nodes_string_02() {
        assert_eq!(Nodes(vec![]).to_string(), "BU_:\n",);
    }
}
