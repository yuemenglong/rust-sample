use std::string::String;

use std::rc::Rc;
use std::collections::HashMap;
use regex::Regex;

use rquery::node::Node;
use rquery::node::NodeEnum;

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
    fn class_eq(attrs: &HashMap<String, String>, class: &str) -> bool {
        if let Some(ref class_string) = attrs.get("class") {
            return class_string.split(" ").any(|str| str == class);
        } else {
            return false;
        }
    }
    fn attr_eq(attrs: &HashMap<String, String>, name: &str, value: &str) -> bool {
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
                &CondItem::Class(class) => Self::class_eq(attrs, class),
                &CondItem::Id(id) => Self::attr_eq(attrs, "id", id),
                &CondItem::Attr { name, value } => Self::attr_eq(attrs, name, value),
                // _ => false,
            }
        } else {
            false
        }
        // tag: &str, attrs: &HashMap<&str, &str>
    }
}

#[derive(Debug)]
pub struct Cond<'a> {
    vec: Vec<CondItem<'a>>,
}

impl<'a> Cond<'a> {
    pub fn new(str: &'a str) -> Cond<'a> {
        let re = Regex::new(r"([.#]?[^.#\[\]]+)|(\[\w+\])").unwrap();
        let vec = re.find_iter(str)
            .map(|(start, end)| CondItem::new(&str[start..end]))
            .collect::<Vec<CondItem>>();
        Cond { vec: vec }
    }
    pub fn test(&self, node: Rc<Node>) -> bool {
        self.vec.iter().all(move |ref item| item.test(node.clone()))
    }
}
