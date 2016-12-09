// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate html5ever;
extern crate tendril;
extern crate regex;

use std::io;
use std::iter::repeat;
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
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::borrow::BorrowMut;
use std::io::Read;
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

#[derive(Debug)]
enum NodeEnum {
    Document,
    Doctype(String, String, String),
    Text(String),
    Comment(String),
    Element(String, HashMap<String, String>),
}

fn parse(handle: Handle) -> Rc<Node> {
    fn escape_default(s: &str) -> String {
        s.chars().flat_map(|c| c.escape_default()).collect()
    }

    let handle = handle.borrow();
    // FIXME: don't allocate
    let mut content = match handle.node {
        SysDocument => {
            println!("#Document");
            NodeEnum::Document
        }
        SysDoctype(ref name, ref public, ref system) => {
            println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system);
            NodeEnum::Doctype(name.to_string(), public.to_string(), system.to_string())
        }
        SysText(ref text) => {
            println!("#text: {}", escape_default(text));
            NodeEnum::Text(text.to_string())
        }
        SysComment(ref text) => {
            println!("<!-- {} -->", escape_default(text));
            NodeEnum::Comment(text.to_string())
        }
        SysElement(ref name, _, ref attrs) => {
            print!("<{}", name.local);
            let mut map = HashMap::<String, String>::new();
            for attr in attrs {
                let &Attribute { ref name, ref value } = attr;
                map.insert(name.local.to_string(), value.to_string());
            }
            for attr in attrs.iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
            NodeEnum::Element(name.local.to_string(), map)
        }
    };
    let rc = Rc::new(Node::new(content));
    for child in handle.children.iter() {
        let childNode = parse(child.clone());
        *childNode.parent.borrow_mut() = Some(Rc::downgrade(&rc));
        rc.children.borrow_mut().push(childNode);
    }
    rc
}

static HTML: &'static str = "
<html>
<head>
</head>
<body>
<div id='container'>
    <div class='cch'></div>
</div>
</body>
</html>
";


#[derive(Debug)]
enum CondItem<'a> {
    Tag(&'a str),
    Class(&'a str),
    Id(&'a str),
    HasAttr(&'a str),
    Attr { name: &'a str, value: &'a str },
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
        if let Some(&attr_value) = attrs.get(name) {
            let () = attr_value;
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
                _ => false,
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
        self.vec.iter().all(|ref item| item.test(node))
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
        // let mut res_vec = Vec::new();
        // let rc = dom.document.deref().clone();
        // walk(rc, &cond_vec, &mut res_vec);
        // for node in res_vec {
        //     node.borrow().debug();
        // }
        // println!("{:?}", res_vec);
    }
    fn traverse(&self, root: Rc<Node>) {}
}

fn load<R: Read>(input: &mut R) {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(input)
        .unwrap();
    let root = parse(dom.document);
}

fn create_context(root: Rc<Node>) {}

fn main() {
    // let stdin = io::stdin();
    let mut input = HTML.as_bytes();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut input)
        .unwrap();
    let root = parse(dom.document);
    println!("{:?}", root);
    // if !dom.errors.is_empty() {
    //     println!("\nParse errors:");
    //     for err in dom.errors.into_iter() {
    //         println!("    {}", err);
    //     }
    // }
}
