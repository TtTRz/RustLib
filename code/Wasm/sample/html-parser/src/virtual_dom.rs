use htmlstream::HTMLTagState;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    tag: Option<String>,
    inner_html: Option<String>,
    attributes: Vec<HashMap<String, String>>,
    node_type: NodeType,
    children: Vec<NodeType>,
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
            children: vec![],
        }
    }
}
#[derive(Debug)]
pub struct Parser {
    state: ParserState,
}

#[derive(Debug)]
pub enum ParserState {
    Closing,
    Opening,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            state: ParserState::Closing,
        }
    }
}

impl Parser {
    fn new() -> Self {
        Self { ..Self::default() }
    }
}

#[derive(Debug)]
pub struct VD {
    state: bool,
    parser: Parser,
}

pub trait VirtualDom {
    fn new() -> Self;
    fn parse_html(&mut self, html: &str);
}

impl VirtualDom for VD {
    fn new() -> Self {
        Self { ..Self::default() }
    }
    fn parse_html(&mut self, html: &str) {
        let root_node = Node::new();
        for (pos, tag) in htmlstream::tag_iter(html) {
            let tag_state = tag.state;
            match self.parser.state {
                ParserState::Closing => {}
                ParserState::Opening => {}
            }
            // tag.state
            // for (pos, attr) in htmlstream::attr_iter(&tag.attributes) {
            //         log(&format!("            {:?} {:?}", pos, attr));
            // }
        }
    }
}

impl Default for VD {
    fn default() -> Self {
        Self {
            state: false,
            parser: Parser::new(),
        }
    }
}
