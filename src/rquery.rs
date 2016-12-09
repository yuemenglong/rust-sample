// // let $ = rquery::load("<html></html>");
// // $("div#test").attr("asdf")->&str

// extern crate regex;

// use std::io::Read;
// use std::iter::Iterator;
// use std::collections::HashMap;
// use std::default::Default;
// use std::rc::Rc;
// use std::cell::RefCell;
// use std::ops::Deref;
// use regex::Regex;

// use tendril::SliceExt;
// use tendril::TendrilSink;

// use html5ever::parse_document;
// use html5ever::Attribute;
// // use html5ever::QualName;

// use html5ever::rcdom::RcDom;
// // use html5ever::rcdom::Handle;
// // use html5ever::rcdom::SysNode;
// // use html5ever::rcdom::{SysText, SysElement};
// use html5ever::rcdom::{Document, Doctype, Text as SysText, Comment, Element as SysElement, Handle,
//                        Node as SysNode};

// use std::rc::Weak;
// #[derive(Debug)]
// struct Node<'a> {
//     parent: Weak<Node<'a>>,
//     cur: Rc<NodeEnum<'a>>,
//     children: Vec<Rc<Node<'a>>>,
// }

// #[derive(Debug)]
// enum NodeEnum<'a> {
//     Element(&'a str, HashMap<&'a str, &'a str>),
//     Text(&'a str),
// }

// fn parse(dom: &RcDom) {
//     let node = dom.document.deref().borrow();
//     match node.node {
//         SysElement(ref name, _, ref attrs) => {
//             let attr_map = HashMap::new();
//             let element = NodeEnum::Element(name.as_ref(), attr_map);
//         },
//         SysText(ref text) => {},
//         _ => {}
//     }
// }

// fn parse_recursive() {}

// pub fn load<R: Read>(input: &mut R) -> Box<Fn(&str) -> Vec<SysNodeWrap>> {
//     let dom = parse_document(RcDom::default(), Default::default())
//         .from_bytes(Default::default())
//         .read_from(input)
//         .unwrap();
//     context_creator(dom.document.deref().clone())
// }

// fn context_creator(rc: Rc<RefCell<SysNode>>) -> Box<Fn(&str) -> Vec<SysNodeWrap>> {
//     Box::new(move |selector: &str| -> Vec<SysNodeWrap> {
//         let re = Regex::new(r"[.#\[\]\w]+").unwrap();
//         // Get Cond From Selector String
//         let cond_vec = re.find_iter(selector)
//             .map(|(start, end)| Cond::new(&selector[start..end]))
//             .collect();
//         // Traverse All SysNode To Get Matched, Save To res_vec
//         let mut res_vec = Vec::new();
//         walk(rc.clone(), &cond_vec, &mut res_vec);
//         for rc in &res_vec {
//             rc.borrow().debug();
//         }
//         res_vec.iter().map(|rc| SysNodeWrap { rc: rc.clone() }).collect()
//     })
// }

// fn walk(rc: Rc<RefCell<SysNode>>, cond_vec: &Vec<Cond>, res_vec: &mut Vec<Rc<RefCell<SysNode>>>) {
//     if test(rc.clone(), cond_vec, cond_vec.len() - 1) {
//         res_vec.push(rc.clone());
//     }
//     let node = rc.borrow();
//     for child in node.children.iter() {
//         walk(child.deref().clone(), cond_vec, res_vec);
//     }
// }

// #[derive(Debug)]
// pub struct SysNodeWrap {
//     rc: Rc<RefCell<SysNode>>,
// }
// impl SysNodeWrap {
//     fn new(rc: Rc<RefCell<SysNode>>) -> SysNodeWrap {
//         SysNodeWrap { rc: rc.clone() }
//     }
//     fn attr<'a>(&'a self, name: &'a str) -> Option<String> {
//         let node = self.rc.borrow();
//         let attr_map = node.attr_map().unwrap();
//         match attr_map.get(name) {
//             Some(ref value) => Some(value.to_string()),
//             _ => None,
//         }
//     }
// }

// #[derive(Debug)]
// enum CondItem<'a> {
//     Tag(&'a str),
//     Class(&'a str),
//     Id(&'a str),
//     HasAttr(&'a str),
//     Attr { name: &'a str, value: &'a str },
// }

// impl<'a> CondItem<'a> {
//     fn new(str: &'a str) -> CondItem<'a> {
//         match &str[0..1] {
//             "." => CondItem::Class(&str[1..]),
//             "#" => CondItem::Id(&str[1..]),
//             _ => CondItem::Tag(&str),
//         }
//     }
//     fn has_class(attrs: &HashMap<&str, &str>, class: &str) -> bool {
//         if let Some(&class_string) = attrs.get("class") {
//             return class_string.split(" ").any(|str| str == class);
//         } else {
//             return false;
//         }
//     }
//     fn has_attr(attrs: &HashMap<&str, &str>, name: &str, value: &str) -> bool {
//         if let Some(&attr_value) = attrs.get(name) {
//             return attr_value == value;
//         } else {
//             return false;
//         }
//     }
//     fn test(&self, tag: &str, attrs: &HashMap<&str, &str>) -> bool {
//         match self {
//             &CondItem::Tag(name) => name == tag,
//             &CondItem::Class(class) => Self::has_class(attrs, class),
//             &CondItem::Id(id) => Self::has_attr(attrs, "id", id),
//             &CondItem::Attr { name, value } => Self::has_attr(attrs, name, value),
//             _ => false,
//         }
//     }
// }

// #[derive(Debug)]
// struct Cond<'a> {
//     vec: Vec<CondItem<'a>>,
// }

// impl<'a> Cond<'a> {
//     fn new(str: &'a str) -> Cond<'a> {
//         let re = Regex::new(r"([.#]?[^.#\[\]]+)|(\[\w+\])").unwrap();
//         let vec = re.find_iter(str)
//             .map(|(start, end)| CondItem::new(&str[start..end]))
//             .collect::<Vec<CondItem>>();
//         Cond { vec: vec }
//     }
//     fn test(&self, tag: &str, attrs: &HashMap<&str, &str>) -> bool {
//         self.vec.iter().all(|ref item| item.test(&tag, &attrs))
//     }
// }

// trait SysNodeKit<'a> {
//     fn debug(&self);
//     fn attr_map(&'a self) -> Option<HashMap<&'a str, &'a str>>;
// }

// fn escape_default(s: &str) -> String {
//     s.chars().flat_map(|c| c.escape_default()).collect()
// }

// impl<'a> SysNodeKit<'a> for SysNode {
//     fn debug(&self) {
//         match self.node {
//             Document => println!("Document"),
//             Doctype(ref name, ref public, ref system) => {
//                 println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system)
//             }
//             SysText(ref text) => println!("SysText: {}", escape_default(text)),
//             Comment(ref text) => println!("<!-- {} -->", escape_default(text)),
//             SysElement(ref name, _, ref attrs) => {
//                 print!("<{}", name.local);
//                 for attr in attrs.iter() {
//                     print!(" {}=\"{}\"", attr.name.local, attr.value);
//                 }
//                 println!(">");
//             }
//         }
//     }
//     fn attr_map(&'a self) -> Option<HashMap<&'a str, &'a str>> {
//         if let SysElement(ref name, _, ref attrs) = self.node {
//             let mut attr_map: HashMap<&str, &str> = HashMap::new();
//             for &Attribute { ref name, ref value } in attrs {
//                 attr_map.insert(name.local.as_ref(), value.as_ref());
//             }
//             Some(attr_map)
//         } else {
//             None
//         }
//     }
// }

// fn test(rc: Rc<RefCell<SysNode>>, cond_vec: &Vec<Cond>, cond_pos: usize) -> bool {
//     let node = rc.borrow();
//     let cond = &cond_vec[cond_pos];
//     if let SysElement(ref name, _, ref attrs) = node.node {
//         let name = name.local.as_ref();
//         let attr_map = node.attr_map().unwrap();
//         let ret = cond.test(name, &attr_map);
//         // println!("{:?}", ret);
//         match (ret, cond_pos, &node.parent) {
//             // 当前成功且已经完成所有cond，返回成功
//             (true, 0, _) => true,
//             // 当前成功，但还有cond需要测试，却已经到达顶层
//             (true, _, &None) => false,
//             // 当前成功，还有cond需要测试，未到达顶层，继续向上层测试
//             (true, _, &Some(ref parent)) => {
//                 test(parent.upgrade().unwrap().clone(), cond_vec, cond_pos - 1)
//             }
//             (_, _, _) => false,
//         }
//     } else {
//         return false;
//     }
// }


// pub fn selector(selector: &str, dom: &RcDom) {
//     let re = Regex::new(r"[.#\[\]\w]+").unwrap();
//     let cond_vec = re.find_iter(selector)
//         .map(|(start, end)| Cond::new(&selector[start..end]))
//         .collect();
//     println!("{:?}", selector);
//     println!("{:?}", cond_vec);
//     let mut res_vec = Vec::new();
//     let rc = dom.document.deref().clone();
//     walk(rc, &cond_vec, &mut res_vec);
//     for node in res_vec {
//         node.borrow().debug();
//     }
//     // println!("{:?}", res_vec);
// }
