pub mod ast;

use ego_tree::Tree;
use std::collections::HashMap;
use winnow::{
    ascii::newline,
    combinator::{opt, repeat, terminated},
    error::{ErrMode, ErrorKind, ParserError},
    stream::Stream,
    token::{any, one_of},
    PResult, Parser,
};

fn parse_open_tag(input: &mut &str) -> PResult<String> {
    if let Some(start) = input.find('<') {
        if let Some(end) = input[start..].find('>') {
            let stripped = &input[start + 1..start + end];
            return Ok(dbg!(stripped.to_lowercase()));
        }
    }
    dbg!("uh oh");
    Err(ErrMode::from_error_kind(input, ErrorKind::Slice))
}

fn parse_node(input: &mut &str) -> PResult<ast::Node> {
    let tag = parse_open_tag.parse_next(input)?;
    Ok(dbg!(ast::Node::Element {
        tag,
        attributes: HashMap::new(),
        children: Vec::new(),
    }))
}

pub fn parse_list(input: &mut &str) -> PResult<Vec<ast::Node>> {
    let mut list = Vec::new();
    while let Ok(output) = parse_node.parse_next(input) {
        list.push(dbg!(output));
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let mut input = "<h1>Hi";
        let output = parse_list.parse_next(&mut input).unwrap();
        assert_eq!(
            output,
            vec![ast::Node::Element {
                tag: "h1".to_string(),
                attributes: HashMap::new(),
                children: vec![]
            }]
        );
    }
}
