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
use html5ever::rcdom::{Document, Doctype, Text as SysText, Comment, Element as SysElement, RcDom,
                       Handle};
use std::rc::Weak;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;
// This is not proper HTML serialization, of course.

#[derive(Debug)]
struct Node {
    parent: RefCell<Option<Weak<Node>>>,
    node: String,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(node: &str) -> Node {
        Node {
            parent: RefCell::new(None),
            node: node.to_string(),
            children: Vec::new(),
        }
    }
}

fn walk(indent: usize, handle: Handle) {
    let handle = handle.borrow();
    // FIXME: don't allocate
    print!("{}", repeat(" ").take(indent).collect::<String>());
    let mut node = "".to_string();

    match handle.node {
        Document => {
            println!("#Document");
        }
        Doctype(ref name, ref public, ref system) => {
            println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system);
        }
        SysText(ref text) => {
            println!("#text: {}", escape_default(text));
        }
        Comment(ref text) => {
            node = escape_default(text).to_string();
            println!("<!-- {} -->", escape_default(text));
        }
        SysElement(ref name, _, ref attrs) => {
            node = name.local.to_string();
            print!("<{}", name.local);
            for attr in attrs.iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }
    }

    for child in handle.children.iter() {
        walk(indent + 4, child.clone());
    }
}

// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}



fn main() {
    let mut rc = Rc::new(Node {
        parent: RefCell::new(None),
        node: "asfd".to_string(),
        children: Vec::new(),
    });
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
