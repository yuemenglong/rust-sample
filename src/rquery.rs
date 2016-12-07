// let $ = rquery::load("<html></html>");
// $("div#test").attr("asdf")->&str

extern crate regex;

use std::io::Read;
use std::iter::Iterator;
use std::collections::HashMap;
use std::default::Default;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use regex::Regex;

use tendril::SliceExt;
use tendril::TendrilSink;

use html5ever::parse_document;
use html5ever::Attribute;
use html5ever::QualName;

use html5ever::rcdom::RcDom;
// use html5ever::rcdom::Handle;
// use html5ever::rcdom::Node;
// use html5ever::rcdom::{Text, Element};
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, Handle, Node};

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
    HasAttr(&'a str),
    Attr { name: &'a str, value: &'a str },
}

impl<'a> CondItem<'a> {
    fn new(str: &'a str) -> CondItem<'a> {
        match &str[0..1] {
            "." => CondItem::Class(&str[1..]),
            "#" => CondItem::Id(&str[1..]),
            "[" => {
            },
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
        let re = Regex::new(r"([.#]?[^.#\[\]]+)|(\[\w+\])").unwrap();
        let vec = re.find_iter(str)
            .map(|(start, end)| CondItem::new(&str[start..end]))
            .collect::<Vec<CondItem>>();
        Cond { vec: vec }
    }
    fn test(&self, tag: &str, attrs: &HashMap<&str, &str>) -> bool {
        self.vec.iter().all(|ref item| item.test(&tag, &attrs))
    }
}

trait NodeKit {
    fn debug(&self);
    fn get_attr_map(&self) -> Option<HashMap<&str, &str>>;
}

fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

impl NodeKit for Node {
    fn debug(&self) {
        match self.node {
            Document => println!("Document"),
            Doctype(ref name, ref public, ref system) => {
                println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system)
            }
            Text(ref text) => println!("Text: {}", escape_default(text)),
            Comment(ref text) => println!("<!-- {} -->", escape_default(text)),
            Element(ref name, _, ref attrs) => {
                print!("<{}", name.local);
                for attr in attrs.iter() {
                    print!(" {}=\"{}\"", attr.name.local, attr.value);
                }
                println!(">");
            }
        }
    }
    fn get_attr_map(&self) -> Option<HashMap<&str, &str>> {
        if let Element(ref name, _, ref attrs) = self.node {
            let mut attr_map: HashMap<&str, &str> = HashMap::new();
            for &Attribute { ref name, ref value } in attrs {
                attr_map.insert(name.local.as_ref(), value.as_ref());
            }
            Some(attr_map)
        } else {
            None
        }
    }
}

fn test(rc: Rc<RefCell<Node>>, cond_vec: &Vec<Cond>, cond_pos: usize) -> bool {
    let node = rc.borrow();
    let cond = &cond_vec[cond_pos];
    if let Element(ref name, _, ref attrs) = node.node {
        let name = name.local.as_ref();
        let attr_map = node.get_attr_map().unwrap();
        let ret = cond.test(name, &attr_map);
        // println!("{:?}", ret);
        match (ret, cond_pos, &node.parent) {
            // 当前成功且已经完成所有cond，返回成功
            (true, 0, _) => true,
            // 当前成功，但还有cond需要测试，却已经到达顶层
            (true, _, &None) => false,
            // 当前成功，还有cond需要测试，未到达顶层，继续向上层测试
            (true, _, &Some(ref parent)) => {
                test(parent.upgrade().unwrap().clone(), cond_vec, cond_pos - 1)
            }
            (_, _, _) => false,
        }
    } else {
        return false;
    }
}

fn walk(rc: Rc<RefCell<Node>>, cond_vec: &Vec<Cond>, res_vec: &mut Vec<Rc<RefCell<Node>>>) {
    if test(rc.clone(), cond_vec, cond_vec.len() - 1) {
        res_vec.push(rc.clone());
    }
    let node = rc.borrow();
    for child in node.children.iter() {
        walk(child.deref().clone(), cond_vec, res_vec);
    }
}

pub fn selector(selector: &str, dom: &RcDom) {
    let re = Regex::new(r"[.#\[\]\w]+").unwrap();
    let cond_vec = re.find_iter(selector)
        .map(|(start, end)| Cond::new(&selector[start..end]))
        .collect();
    println!("{:?}", selector);
    println!("{:?}", cond_vec);
    let mut res_vec = Vec::new();
    let rc = dom.document.deref().clone();
    walk(rc, &cond_vec, &mut res_vec);
    for node in res_vec {
        node.borrow().debug();
    }
    // println!("{:?}", res_vec);
}
