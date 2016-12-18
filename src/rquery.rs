use std::default::Default;
use std::string::String;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::Attribute;
use html5ever::rcdom::{RcDom, Handle};
use html5ever::rcdom::{Document as SysDocument, Doctype as SysDoctype, Text as SysText,
                       Comment as SysComment, Element as SysElement};
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::ops::Deref;
use std::fmt;
use regex::Regex;
// This is not proper HTML serialization, of course.

#[derive(Debug)]
struct Node {
    parent: RefCell<Option<Weak<Node>>>,
    content: NodeEnum,
    children: RefCell<Vec<Rc<Node>>>,
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.content {
            NodeEnum::Document => write!(f, "Document"),
            NodeEnum::Doctype(ref name, ref public, ref system) => {
                write!(f, "<!DOCTYPE {} \"{}\" \"{}\">", name, public, system)
            }
            NodeEnum::Text(ref text) => write!(f, "{}", text),
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
enum NodeEnum {
    Document,
    Doctype(String, String, String),
    Text(String),
    Comment(String),
    Element(String, HashMap<String, String>),
}

fn parse(handle: Handle) -> Rc<Node> {
    // fn escape_default(s: &str) -> String {
    //     s.chars().flat_map(|c| c.escape_default()).collect()
    // }

    let handle = handle.borrow();
    // FIXME: don't allocate
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
    let rc = Rc::new(Node::new(content));
    for child in handle.children.iter() {
        let child_node = parse(child.clone());
        *child_node.parent.borrow_mut() = Some(Rc::downgrade(&rc));
        rc.children.borrow_mut().push(child_node);
    }
    rc
}

#[derive(Debug)]
enum CondItem<'a> {
    Tag(&'a str),
    Class(&'a str),
    Id(&'a str),
    Attr { name: &'a str, value: &'a str }, // HasAttr(&'a str),
}

impl<'a> CondItem<'a> {
    fn new(str: &'a str) -> CondItem<'a> {
        match &str[0..1] {
            "." => CondItem::Class(&str[1..]),
            "#" => CondItem::Id(&str[1..]),
            _ => CondItem::Tag(&str),
        }
    }
    fn has_class(attrs: &HashMap<String, String>, class: &str) -> bool {
        if let Some(ref class_string) = attrs.get("class") {
            return class_string.split(" ").any(|str| str == class);
        } else {
            return false;
        }
    }
    fn has_attr(attrs: &HashMap<String, String>, name: &str, value: &str) -> bool {
        if let Some(attr_value) = attrs.get(name) {
            return attr_value == value;
        } else {
            return false;
        }
    }
    fn test(&self, node: Rc<Node>) -> bool {
        if let NodeEnum::Element(ref tag, ref attrs) = node.content {
            match self {
                &CondItem::Tag(name) => name == tag,
                &CondItem::Class(class) => Self::has_class(attrs, class),
                &CondItem::Id(id) => Self::has_attr(attrs, "id", id),
                &CondItem::Attr { name, value } => Self::has_attr(attrs, name, value),
                // _ => false,
            }
        } else {
            false
        }
        // tag: &str, attrs: &HashMap<&str, &str>

    }
}

#[derive(Debug)]
struct Cond<'a> {
    vec: Vec<CondItem<'a>>,
}

impl<'a> Cond<'a> {
    fn new(str: &'a str) -> Cond<'a> {
        let re = Regex::new(r"([.#]?[^.#\[\]]+)|(\[\w+\])").unwrap();
        let vec = re.find_iter(str)
            .map(|(start, end)| CondItem::new(&str[start..end]))
            .collect::<Vec<CondItem>>();
        Cond { vec: vec }
    }
    fn test(&self, node: Rc<Node>) -> bool {
        self.vec.iter().all(move |ref item| item.test(node.clone()))
    }
}

#[derive(Debug)]
struct Selector<'a> {
    vec: Vec<Cond<'a>>,
}

impl<'a> Selector<'a> {
    fn new(str: &'a str) -> Selector<'a> {
        let re = Regex::new(r"[.#\[\]\w]+").unwrap();
        let vec = re.find_iter(str)
            .map(|(start, end)| Cond::new(&str[start..end]))
            .collect();
        Selector { vec: vec }
    }
    fn select(&self, root: Rc<Node>) -> Vec<Rc<Node>> {
        fn walk(root: Rc<Node>,
                node: Rc<Node>,
                cond_vec: &Vec<Cond>,
                res_vec: &mut Vec<Rc<Node>>) {
            if reverse_test(root.clone(), node.clone(), cond_vec, cond_vec.len() - 1) {
                res_vec.push(node.clone());
            }
            for child in node.children.borrow().iter() {
                walk(root.clone(), child.clone(), cond_vec, res_vec);
            }
        }
        fn reverse_test(root: Rc<Node>,
                        node: Rc<Node>,
                        cond_vec: &Vec<Cond>,
                        cond_pos: usize)
                        -> bool {
            // Compare Current Nod And Current Cond
            let cond = &cond_vec[cond_pos];
            let current_test = cond.test(node.clone());
            let parent = node.parent.borrow();
            let is_back_cond = cond_pos == cond_vec.len() - 1;
            let is_front_cond = cond_pos == 0;
            let is_root = node.deref() as *const _ == root.deref() as *const _;
            match (current_test, is_back_cond, is_front_cond, is_root) {
                // Current Is Fail And Not Match Back Cond, Fail
                (false, true, _, _) => false,
                // Current Is Fail But Has Match Back Cond, But Reach Root, Fail
                (false, false, _, true) => false,
                // Current Is Fail But Has Match Back Cond, And Not Reach Root, Recursive For Current Cond
                (false, false, _, false) => {
                    let parent = parent.as_ref().unwrap().upgrade().unwrap();
                    reverse_test(root, parent, cond_vec, cond_pos)
                }
                // Current Is Succ And Finish All Conds, Succ
                (true, _, true, _) => true,
                // Current Is Succ And Not Finish All Conds, But Reach Root, Fail
                (true, _, false, true) => false, 
                // Current Is Succ And Not Finish All Conds, And Not Reach Root, Recursive For Next Cond
                (true, _, false, false) => {
                    let parent = parent.as_ref().unwrap().upgrade().unwrap();
                    reverse_test(root, parent, cond_vec, cond_pos - 1)
                }
            }
        }
        let mut res_vec = Vec::new();
        // let () = root.parent.borrow_mut().deref();
        walk(root.clone(), root.clone(), &self.vec, &mut res_vec);
        res_vec
    }
}

pub struct SelectResult {
    res: Vec<Rc<Node>>,
}

impl SelectResult {
    fn new(res: Vec<Rc<Node>>) -> SelectResult {
        SelectResult { res: res }
    }
    fn from_node(node: Rc<Node>) -> SelectResult {
        SelectResult { res: vec![node] }
    }
    fn check(&self) {
        if self.res.len() > 1 {
            panic!("{}", "There Are More Than One Children");
        }
    }
    pub fn children(&self, selector: &str) -> SelectResult {
        let mut res = Vec::new();
        let selector = Selector::new(selector);
        for node in self.res.iter() {
            let vec = selector.select(node.clone());
            res.extend(vec);
        }
        SelectResult::new(res)
    }
    pub fn get(&self, idx: usize) -> SelectResult {
        SelectResult { res: vec![self.res[idx].clone()] }
    }
    pub fn attr(&self, name: &str) -> Option<&String> {
        self.check();
        if self.res.len() == 0 {
            return None;
        }
        match self.res[0].content {
            NodeEnum::Element(_, ref attrs) => attrs.get(name),
            _ => None,
        }
    }
    pub fn name(&self) -> Option<&String> {
        self.check();
        if self.res.len() == 0 {
            return None;
        }
        match self.res[0].content {
            NodeEnum::Element(ref tag, _) => Some(tag),
            _ => None,
        }
    }
    pub fn text(&self) -> Option<String> {
        fn recursive_text(node: Rc<Node>, res: &mut String) {
            match node.content {
                NodeEnum::Text(ref text) => res.push_str(text),
                _ => {}
            }
            for child in node.children.borrow().iter() {
                recursive_text(child.clone(), res);
            }
        }
        self.check();
        if self.res.len() == 0 {
            return None;
        }
        let mut res = String::new();
        recursive_text(self.res[0].clone(), &mut res);
        Some(res)
    }
    pub fn html(&self) -> Option<String> {
        self.check();
        if self.res.len() == 0 {
            return None;
        }
        fn recursive_inner(node: Rc<Node>, res: &mut String) {
            match node.content {
                NodeEnum::Element(ref tag, ref attrs) => {
                    res.push_str(format!("<{}", tag).as_ref());
                    for (name, value) in attrs {
                        res.push_str(format!(" {}='{}'", name, value).as_ref());
                    }
                    res.push_str(">");
                    for child in node.children.borrow().iter() {
                        recursive_inner(child.clone(), res);
                    }
                    res.push_str(format!("</{}>", tag).as_ref());
                }
                NodeEnum::Text(ref text) => {
                    res.push_str(text);
                }
                _ => {}
            }
        }
        let mut res = String::new();
        recursive_inner(self.res[0].clone(), &mut res);
        Some(res)
    }
}


impl fmt::Display for SelectResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.html() {
            Some(ref html) => write!(f, "{}", html),
            None => write!(f, ""),
        }
    }
}

impl IntoIterator for SelectResult {
    type Item = SelectResult;
    type IntoIter = ::std::vec::IntoIter<SelectResult>;

    fn into_iter(self) -> Self::IntoIter {
        self.res
            .iter()
            .map(|node| SelectResult::from_node(node.clone()))
            .collect::<Vec<SelectResult>>()
            .into_iter()
    }
}

pub fn load<R: Read>(input: &mut R) -> Box<Fn(&str) -> SelectResult> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(input)
        .unwrap();
    let root = parse(dom.document);
    create_context(root)
}

fn create_context(root: Rc<Node>) -> Box<Fn(&str) -> SelectResult> {
    Box::new(move |selector| {
        let selector = Selector::new(selector);
        SelectResult::new(selector.select(root.clone()))
    })
}
