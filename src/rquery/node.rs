use std::string::String;

use html5ever::Attribute;
use html5ever::rcdom::Handle;
use html5ever::rcdom::{Document as SysDocument, Doctype as SysDoctype, Text as SysText,
                       Comment as SysComment, Element as SysElement};
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

pub struct Node {
    pub parent: RefCell<Option<Weak<Node>>>,
    pub content: NodeEnum,
    pub children: RefCell<Vec<Rc<Node>>>,
}
impl Node {
    fn new(content: NodeEnum) -> Node {
        Node {
            parent: RefCell::new(None),
            content: content,
            children: RefCell::new(Vec::new()),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.content {
            NodeEnum::Document => write!(f, "$Document"),
            NodeEnum::Doctype(ref name, ref public, ref system) => {
                write!(f, "<!DOCTYPE {} \"{}\" \"{}\">", name, public, system)
            }
            NodeEnum::Text(ref text) => write!(f, "$Text: {}", text),
            NodeEnum::Comment(ref comment) => write!(f, "<!-- {} -->", comment),
            NodeEnum::Element(ref tag, ref map) => {
                let _ = write!(f, "<{}", tag);
                for (name, value) in map {
                    let _ = write!(f, " {}='{}'", name, value);
                }
                write!(f, " />")
            }
        }
    }
}

#[derive(Debug)]
pub enum NodeEnum {
    Document,
    Doctype(String, String, String),
    Text(String),
    Comment(String),
    Element(String, HashMap<String, String>),
}

pub fn parse(handle: Handle) -> Rc<Node> {
    let handle = handle.borrow();
    let content = match handle.node {
        SysDocument => NodeEnum::Document,
        SysDoctype(ref name, ref public, ref system) => {
            NodeEnum::Doctype(name.to_string(), public.to_string(), system.to_string())
        }
        SysText(ref text) => NodeEnum::Text(text.to_string()),
        SysComment(ref text) => NodeEnum::Comment(text.to_string()),
        SysElement(ref name, _, ref attrs) => {
            let mut map = HashMap::<String, String>::new();
            for attr in attrs {
                let &Attribute { ref name, ref value } = attr;
                map.insert(name.local.to_string(), value.to_string());
            }
            NodeEnum::Element(name.local.to_string(), map)
        }
    };
    let node = Rc::new(Node::new(content));
    for child in handle.children.iter() {
        let child_node = parse(child.clone());
        *child_node.parent.borrow_mut() = Some(Rc::downgrade(&node));
        node.children.borrow_mut().push(child_node);
    }
    node
}
