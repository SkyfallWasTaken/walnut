use std::collections::HashMap;

#[derive(Debug)]
pub enum Node {
    Element {
        tag: String,
        attributes: HashMap<String, String>,
        children: Vec<Node>,
    },
    Text(String),
}
