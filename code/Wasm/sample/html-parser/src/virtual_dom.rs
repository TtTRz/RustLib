use htmlstream::HTMLTagState;
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
pub struct Parser {
    tag_state: HTMLTagState,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            tag_state: HTMLTagState::Closing,
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
        for (pos, tag) in htmlstream::tag_iter(html) {
            match self.parser.tag_state {
                // </div>
                HTMLTagState::Closing => {}
                // <div>
                HTMLTagState::Opening => {}
                // text
                HTMLTagState::Text => {}
                // <input />
                HTMLTagState::SelfClosing => {}
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
