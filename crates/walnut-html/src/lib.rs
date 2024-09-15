use ego_tree::Tree;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "stream.pest"]
pub struct StreamParser;

#[derive(Debug)]
pub struct Node {}

pub fn parse(html: &str) -> Tree<Node> {
    let stream = StreamParser::parse(Rule::stream, html)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

    visit_nodes(stream);

    todo!()
}

fn visit_nodes(pair: Pair<'_, Rule>) {
    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::opening => {
                let tag = item.into_inner().next().unwrap();
                dbg!(tag.as_str());
            }
            Rule::closing => {
                let tag = item.into_inner().next().unwrap();
                dbg!(tag.as_str());
            }
            Rule::content => visit_nodes(item),
            Rule::text => {
                dbg!(item.as_str());
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
