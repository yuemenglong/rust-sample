// let $ = rquery::load("<html></html>");
// $("div#test").attr("asdf")->&str

extern crate regex;

use std::io::Read;
use std::default::Default;
use regex::Regex;
use tendril::SliceExt;
use tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::RcDom;
use html5ever::rcdom::Handle;
use html5ever::rcdom::{Text, Element};
// use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, Handle};

pub fn load<R: Read>(input: &mut R) -> RcDom {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_bytes(Default::default())
        .read_from(input)
        .unwrap();
    return dom;
}

#[derive(Debug)]
enum Item<'a> {
    Tag(&'a str),
    Class(&'a str),
    Id(&'a str),
    Attr { name: &'a str },
}

impl<'a> Item<'a> {
    fn new(str: &'a str) -> Item<'a> {
        match &str[0..1] {
            "." => Item::Class(&str[1..]),
            "#" => Item::Id(&str[1..]),
            _ => Item::Tag(&str),
        }
    }
}

#[derive(Debug)]
struct Cond<'a> {
    vec: Vec<Item<'a>>,
}

impl<'a> Cond<'a> {
    fn new(str: &'a str) -> Cond<'a> {
        let re = Regex::new(r"[.#]?[^.#]+").unwrap();
        let vec = re.find_iter(str)
            .map(|(start, end)| Item::new(&str[start..end]))
            .collect::<Vec<Item>>();
        Cond { vec: vec }
    }
}

fn walk(handle: Handle, cond_vec:&Vec<Cond>, level:usize) {
    let cur = handle.borrow();
    match cur.node {
        Element(ref name, _, ref attrs) => println!("Element {}", name.local),
        Text(ref text) => println!("Text {:?}", text),
        _ => {}
    }
    for child in cur.children.iter() {
        walk(child.clone(), cond_vec, level);
    }
}

pub fn selector(str: &str, dom: &RcDom) {
    let re = Regex::new(r"[.#\w]+").unwrap();
    // let selector = Cond::new();
    let cond_vec = re.find_iter(str)
        .map(|(start, end)| Cond::new(&str[start..end]))
        .collect::<Vec<Cond>>();
    println!("{:?}", cond_vec);
    walk(dom.document.clone(), &cond_vec, cond_vec.len() - 1);

    // let vec:Vec<Cond> = Vec::new();
    // for (start, end) in re.find_iter(str) {
    // let slice = &str[start..end];
    // let cond = Cond::new(slice);
    // cond.parse(slice);
    // println!("{:?}", cond);
    // }
    // println!("{:?}", res);
}
