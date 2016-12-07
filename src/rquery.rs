// let $ = rquery::load("<html></html>");
// $("div#test").attr("asdf")->&str

extern crate regex;

use std::io::Read;
use std::iter::Iterator;
use std::collections::HashMap;
use std::default::Default;
use regex::Regex;

use tendril::SliceExt;
use tendril::TendrilSink;

use html5ever::parse_document;
use html5ever::Attribute;
use html5ever::QualName;

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
enum CondItem<'a> {
    Tag(&'a str),
    Class(&'a str),
    Id(&'a str),
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
    fn has_class(attrs: &HashMap<&str, &str>, class: &str) -> bool {
        if let Some(&class_string) = attrs.get("class") {
            return class_string.split(" ").any(|str| str == class);
        } else {
            return false;
        }
    }
    fn has_attr(attrs: &HashMap<&str, &str>, name: &str, value: &str) -> bool {
        if let Some(&attr_value) = attrs.get(name) {
            return attr_value == value;
        } else {
            return false;
        }
    }
    fn test(&self, tag: &str, attrs: &HashMap<&str, &str>) -> bool {
        match self {
            &CondItem::Tag(name) => name == tag,
            &CondItem::Class(class) => Self::has_class(attrs, class),
            &CondItem::Id(id) => Self::has_attr(attrs, "id", id),
            &CondItem::Attr { name, value } => Self::has_attr(attrs, name, value),
        }
    }
}

#[derive(Debug)]
struct Cond<'a> {
    vec: Vec<CondItem<'a>>,
}

impl<'a> Cond<'a> {
    fn new(str: &'a str) -> Cond<'a> {
        let re = Regex::new(r"[.#]?[^.#]+").unwrap();
        let vec = re.find_iter(str)
            .map(|(start, end)| CondItem::new(&str[start..end]))
            .collect::<Vec<CondItem>>();
        Cond { vec: vec }
    }
    fn test(&self, tag: &str, attrs: &HashMap<&str, &str>) -> bool {
        self.vec.iter().all(|ref item| item.test(&tag, &attrs))
    }
}

fn test(handle: Handle, cond_vec: &Vec<Cond>, cond_pos: usize) -> bool {
    let cond = &cond_vec[cond_pos];
    let handle = handle.borrow();
    if let Element(ref name, _, ref attrs) = handle.node {
        let name = name.local.as_ref();
        let mut attrMap: HashMap<&str, &str> = HashMap::new();
        for &Attribute { ref name, ref value } in attrs {
            attrMap.insert(name.local.as_ref(), value.as_ref());
        }
        let ret = cond.test(&name, &attrMap);
        println!("{:?}", ret);
        match (ret, cond_pos) {
            (true, 0) => true,
            (_, _) => false,
        }
    } else {
        return false;
    }
}

fn walk(handle: Handle, cond_vec: &Vec<Cond>) {
    test(handle.clone(), &cond_vec, 0);
    for child in handle.borrow().children.iter() {
        walk(child.clone(), cond_vec);
    }
    // let cur = handle.borrow();
    // match cur.node {
    //     Element(ref name, _, ref attrs) => println!("Element {}", name.local),
    //     Text(ref text) => println!("Text {:?}", text),
    //     _ => {}
    // }
    // for child in cur.children.iter() {
    //     walk(child.clone(), cond_vec);
    // }
}

pub fn selector(str: &str, dom: &RcDom) {
    let re = Regex::new(r"[.#\w]+").unwrap();
    // let selector = Cond::new();
    let cond_vec = re.find_iter(str)
        .map(|(start, end)| Cond::new(&str[start..end]))
        .collect::<Vec<Cond>>();
    println!("{:?}", cond_vec);
    walk(dom.document.clone(), &cond_vec);

    // let vec:Vec<Cond> = Vec::new();
    // for (start, end) in re.find_iter(str) {
    // let slice = &str[start..end];
    // let cond = Cond::new(slice);
    // cond.parse(slice);
    // println!("{:?}", cond);
    // }
    // println!("{:?}", res);
}
