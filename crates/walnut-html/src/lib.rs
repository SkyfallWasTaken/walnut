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

    tree
}

fn visit_nodes<'a>(pair: Pair<'_, Rule>, parent: &mut NodeMut<'_, Node>) {
    let mut tag_stack = vec!["html"];
    let mut node_stack = vec![parent];

    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::opening => {
                let mut item = item.into_inner();
                tag_stack.push(item.next().unwrap().as_str().trim());

                let attrs = item.next().unwrap().into_inner();
                for attr in attrs {
                    let mut attr = attr.into_inner();
                    let lowercase_key = attr.next().unwrap().to_string().to_lowercase();
                    let key = lowercase_key.as_str().trim();
                    let value = attr.next().unwrap().as_str();
                    node_stack
                        .last_mut()
                        .unwrap()
                        .value()
                        .attrs
                        .insert(key.to_string(), value.to_string());
                }
            }
            Rule::closing => {
                let closing_tag = item.into_inner().next().unwrap().as_str().trim();
                if let Some(opening_tag) = tag_stack.pop() {
                    if opening_tag != closing_tag {
                        // Handle mismatched tags if needed
                        println!(
                            "Warning: Mismatched tags. Expected {}, found {}",
                            opening_tag, closing_tag
                        );
                    }
                    // Pop the current node from the stack as we're closing its tag
                    if node_stack.len() > 1 {
                        let closed_node = node_stack.pop().unwrap();
                        node_stack
                            .last_mut()
                            .unwrap()
                            .append(closed_node.value().clone());
                    }
                } else {
                    // Handle the case where there's a closing tag without a matching opening tag
                    println!("Warning: Unexpected closing tag {}", closing_tag);
                }
            }
            Rule::content => visit_nodes(item, node_stack.last_mut().unwrap()),
            Rule::text => {
                let chunk = item.as_str();
                // Add the text content to the current node
                node_stack.last_mut().unwrap().value().text.push_str(chunk);
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
