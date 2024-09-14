pub mod ast;

use ego_tree::Tree;
use std::collections::HashMap;
use winnow::combinator::opt;
use winnow::combinator::repeat;
use winnow::combinator::terminated;
use winnow::token::take_until;
use winnow::{
    ascii::newline,
    combinator::terminated,
    error::{ErrMode, ErrorKind, ParserError},
    stream::Stream,
    token::{any, one_of},
    PResult, Parser,
};

fn parse_doc(input: &mut &str) -> PResult<Tree<ast::Node>> {
    let mut tree = Tree::new(ast::Node::Element {
        tag: "html".to_string(),
        attributes: HashMap::new(),
        children: Vec::new(),
    });
    repeat(0.., terminated(parse_open_tag, newline(input))).parse_next(input);

    Ok(tree)
}

fn parse_tag<'s>(input: &mut &'s str) -> PResult<&'s str> {
    input.take_until(1.., ">")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag() {
        let mut input = "<hi>";
        let output = parse_tag.parse_next(&mut input).unwrap();
        assert_eq!(output, "hi");
    }

    #[test]
    fn test_tag_fail1() {
        let mut input = "<hi";
        let output = parse_tag.parse_next(&mut input);
        assert!(output.is_err());
    }

    #[test]
    fn test_tag_fail2() {
        let mut input = "hi>";
        let output = parse_tag.parse_next(&mut input);
        assert!(output.is_err());
    }
}
