use std::collections::HashMap;

use ego_tree::{NodeMut, Tree};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "stream.pest"]
pub struct StreamParser;

#[derive(Debug, Clone)]
pub struct Node {
    pub tag: String,
    pub text: String,
    pub attrs: HashMap<String, String>,
}

pub fn parse(html: &str) -> Tree<Node> {
    let stream = StreamParser::parse(Rule::stream, html)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

    let mut tree = Tree::new(Node {
        tag: "html".to_string(),
        text: "".to_string(),
        attrs: HashMap::new(),
    });
    visit_nodes(stream, &mut tree.root_mut());

    dbg!(tree)
}

fn visit_nodes<'a>(pair: Pair<'_, Rule>, parent: &mut NodeMut<'_, Node>) {
    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::opening => {
                let opening_tag = item.into_inner().next().unwrap().as_str().trim();
                // Add text before the opening tag
                todo!("add text before opening tag");

                let mut item = item.into_inner();
                parent.last_child().unwrap().append(Node {
                    tag: opening_tag.to_string(),
                    text: "".to_string(),
                    attrs: HashMap::new(),
                });

                let attrs = item.next().unwrap().into_inner();
                for attr in attrs {
                    let mut attr = attr.into_inner();
                    let lowercase_key = attr.next().unwrap().to_string().to_lowercase();
                    let key = lowercase_key.as_str().trim();
                    let value = attr.next().unwrap().as_str();
                    todo!("add attributes to current node");
                }
            }
            Rule::closing => {
                let closing_tag = item.into_inner().next().unwrap().as_str().trim();
                todo!()
            }
            Rule::content => todo!(),
            Rule::text => {
                let chunk = item.as_str();
                // Add the text content to the current node
                todo!()
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let html = include_str!("../cern.html");
        let dom = parse(html);
        dbg!(dom);
    }
}
