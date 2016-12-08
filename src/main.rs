mod rquery;

extern crate regex;
extern crate tendril;
extern crate html5ever;

// use tendril::{StrTendril, SliceExt};
// use tendril::{ByteTendril, ReadExt};
use tendril::SliceExt;
use std::io::Read;
use std::rc::Rc;
use std::borrow::Borrow;

// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// #[macro_use] extern crate html5ever_atoms;
// extern crate tendril;

use std::io;
use std::collections::HashMap;
use std::iter::repeat;
use std::default::Default;
use std::string::String;
use regex::Regex;

use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};

// This is not proper HTML serialization, of course.

// fn walk(indent: usize, handle: Handle) {
//     let node = handle.borrow();
//     // FIXME: don't allocate
//     print!("{}", repeat(" ").take(indent).collect::<String>());
//     match node.node {
//         Document => println!("#Document"),

//         Doctype(ref name, ref public, ref system) => {
//             println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system)
//         }

//         Text(ref text) => println!("#text: {}", escape_default(text)),

//         Comment(ref text) => println!("<!-- {} -->", escape_default(text)),

//         Element(ref name, _, ref attrs) => {
//             // assert!(name.ns == ns!(html));
//             print!("<{}", name.local);
//             for attr in attrs.iter() {
//                 //     assert!(attr.name.ns == ns!());
//                 print!(" {}=\"{}\"", attr.name.local, attr.value);
//             }
//             println!(">");
//         }
//     }

//     for child in node.children.iter() {
//         walk(indent + 4, child.clone());
//     }
// }

// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

static HTML: &'static str = "
<html>
<head>
</head>
<body>
<div id='id' class='class'>
    <div id='inner'></div>
</div>
</body>
</html>
";

struct Person<'a> {
    name: &'a str,
}

impl Person {
    fn get_field_map(&self) -> HashMap<&str, &str> {
        let mut ret: HashMap<&str, &str> = HashMap::new();
        ret.insert("name", self.name);
        ret
    }
}

fn get_name(rc: Rc<Person>) -> Option<&str> {
    let p: &Person = rc.borrow();
    match p.get_field_map().get("name") {
        Some(&str) => Some(str),
        _ => None,
    }
}

fn get_name2(p:&Person>) -> Option<&str> {
    match p.get_field_map().get("name") {
        Some(&str) => Some(str),
        _ => None,
    }
}

fn main() {
    let rc = Rc::new(Person { name: "hi" });
    get_name(rc.clone());
    return;
}
    // let stdin = io::stdin();
    // let mut input = "<html></html>";
    // let mut bytes = "<html><head></head><body></body></html>".as_bytes();
    // let dom = rquery::load(&mut bytes);
    // let dom = parse_document(RcDom::default(), Default::default())
    //     .from_bytes(Default::default())
    //     .read_from(&mut bytes)
    //     .unwrap();
    // .from_utf8()
    // .read_from(&mut stdin.lock())
    // .unwrap();
    // walk(0, dom.document);
    // .process("<html></html>".to_tendril());
    // println!("{:?}", dom.document);
    // if !dom.errors.is_empty() {
    //     println!("\nParse errors:");
    //     for err in dom.errors.into_iter() {
    //         println!("    {}", err);
    //     }
    // }
// }
