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
        // 严格将第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此结果表明操作成功
        // 或失败。注意这里的 `write!` 用法和 `println!` 很相似。
        match self.content {
            NodeEnum::Document => write!(f, "Document"),
            NodeEnum::Doctype(ref name, ref public, ref system) => {
                write!(f, "<!DOCTYPE {} \"{}\" \"{}\">", name, public, system)
            }
            NodeEnum::Text(ref text) => write!(f, "{}", text),
            NodeEnum::Comment(ref comment) => write!(f, "<!-- {} -->", comment),
            NodeEnum::Element(ref tag, ref map) => {
                write!(f, "<{}", tag);
                for (name, value) in map {
                    write!(f, " {}='{}'", name, value);
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
    fn escape_default(s: &str) -> String {
        s.chars().flat_map(|c| c.escape_default()).collect()
    }

    let handle = handle.borrow();
    // FIXME: don't allocate
    let mut content = match handle.node {
        SysDocument => {
            NodeEnum::Document
        }
        SysDoctype(ref name, ref public, ref system) => {
            NodeEnum::Doctype(name.to_string(), public.to_string(), system.to_string())
        }
        SysText(ref text) => {
            NodeEnum::Text(text.to_string())
        }
        SysComment(ref text) => {
            NodeEnum::Comment(text.to_string())
        }
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
        fn walk(node: Rc<Node>, cond_vec: &Vec<Cond>, res_vec: &mut Vec<Rc<Node>>) {
            if reverse_test(node.clone(), cond_vec, cond_vec.len() - 1) {
                res_vec.push(node.clone());
            }
            for child in node.children.borrow().iter() {
                walk(child.clone(), cond_vec, res_vec);
            }
        }
        fn reverse_test(node: Rc<Node>, cond_vec: &Vec<Cond>, cond_pos: usize) -> bool {
            // 比较当前节点和当前条件的关系
            let cond = &cond_vec[cond_pos];
            let ret = cond.test(node.clone());
            match (ret, cond_pos, node.parent.borrow().deref()) {
                // 当前成功且已经完成所有cond，返回成功
                (true, 0, _) => true,
                // 当前成功，但还有cond需要测试，却已经到达顶层
                (true, _, &None) => false,
                // 当前成功，还有cond需要测试，未到达顶层，继续向上层测试
                (true, _, &Some(ref parent)) => {
                    reverse_test(parent.upgrade().unwrap(), cond_vec, cond_pos - 1)
                }
                (_, _, _) => false,
            }
        }
        let mut res_vec = Vec::new();
        walk(root, &self.vec, &mut res_vec);
        res_vec
    }
}

fn load<R: Read>(input: &mut R) -> Box<Fn(&str) -> Vec<Rc<Node>>> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(input)
        .unwrap();
    let root = parse(dom.document);
    create_context(root)
}

fn create_context(root: Rc<Node>) -> Box<Fn(&str) -> Vec<Rc<Node>>> {
    Box::new(move |selector| {
        let selector = Selector::new(selector);
        selector.select(root.clone())
    })
}

fn main() {
    // let stdin = io::stdin();
    let mut input = HTML.as_bytes();
    let S = load(&mut input);
    for node in S("div") {
        println!("{}", node);
    }
    // let dom = parse_document(RcDom::default(), Default::default())
    //     .from_utf8()
    //     .read_from(&mut input)
    //     .unwrap();
    // let root = parse(dom.document);
    // println!("{:?}", root);
    // if !dom.errors.is_empty() {
    //     println!("\nParse errors:");
    //     for err in dom.errors.into_iter() {
    //         println!("    {}", err);
    //     }
    // }
}
