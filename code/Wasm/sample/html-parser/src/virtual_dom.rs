use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    tag: Option<String>,
    inner_html: Option<String>,
    attributes: Vec<HashMap<String, String>>,
    node_type: NodeType,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    ElementNode,
    FragmentNode,
    TextNode,
    None,
}

impl Node {
    fn new() -> Self {
        Self { ..Node::default() }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            tag: None,
            inner_html: None,
            attributes: vec![],
            node_type: NodeType::None,
        }
    }
}

#[derive(Debug)]
pub struct VD {
    status: bool,
}

pub trait VirtualDom {
    fn new() -> Self;
    fn parse_html(html: &str);

    fn init(&mut self);
}

impl VirtualDom for VD {
    fn new() -> Self {
        Self { ..VD::default() }
    }
    fn parse_html(html: &str) {
        for (pos, tag) in htmlstream::tag_iter(html) {
            // for (pos, attr) in htmlstream::attr_iter(&tag.attributes) {
            //         log(&format!("            {:?} {:?}", pos, attr));
            // }
        }
    }
    fn init(&mut self) {

    }
}

impl Default for VD {
    fn default() -> Self {
        Self { status: false }
    }
}
