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

fn walk(indent: usize, handle: Handle) -> Rc<Node> {
    let handle = handle.borrow();
    // FIXME: don't allocate
    print!("{}", repeat(" ").take(indent).collect::<String>());
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
        let childNode = walk(indent + 4, child.clone());
        *childNode.parent.borrow_mut() = Some(Rc::downgrade(&rc));
        rc.children.borrow_mut().push(childNode);
    }
    rc
}

// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

struct Person {
    age: i32,
}

fn main() {
    // let stdin = io::stdin();
    // let dom = parse_document(RcDom::default(), Default::default())
    //     .from_utf8()
    //     .read_from(&mut stdin.lock())
    //     .unwrap();
    // walk(0, dom.document);

    // if !dom.errors.is_empty() {
    //     println!("\nParse errors:");
    //     for err in dom.errors.into_iter() {
    //         println!("    {}", err);
    //     }
    // }
}
