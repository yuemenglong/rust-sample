use std::string::String;

use std::rc::Rc;
use std::collections::HashMap;
use regex::Regex;

use rquery::node::Node;
use rquery::node::NodeEnum;

#[derive(Debug)]
enum CondItem {
    Tag(String),
    Class(String),
    Id(String),
    Has(String),
    Attr(String, String),
    NextLayer,
}

impl CondItem {
    fn new(str: &str) -> CondItem {
        match &str[0..1] {
            ">" => CondItem::NextLayer,
            "." => CondItem::Class(str[1..].to_string()),
            "#" => CondItem::Id(str[1..].to_string()),
            "[" => {
                let re = Regex::new("=").unwrap();
                match re.find(str) {
                    Some((s, e)) => {
                        CondItem::Attr(str[1..s].to_string(), str[e..str.len() - 1].to_string())
                    }
                    None => CondItem::Has(str[1..str.len() - 1].to_string()),
                }
            }
            _ => CondItem::Tag(str.to_string()),
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
    fn attr_exists(attrs: &HashMap<String, String>, name: &str) -> bool {
        match attrs.get(name) {
            Some(_) => true,
            None => false,
        }
    }
    fn test(&self, node: Rc<Node>) -> bool {
        if let NodeEnum::Element(ref tag, ref attrs) = node.content {
            match self {
                &CondItem::Tag(ref name) => name == tag,
                &CondItem::Class(ref class) => Self::class_eq(attrs, class),
                &CondItem::Id(ref id) => Self::attr_eq(attrs, "id", id),
                &CondItem::Attr(ref name, ref value) => Self::attr_eq(attrs, name, value),
                &CondItem::Has(ref name) => Self::attr_exists(attrs, name),
                &CondItem::NextLayer => false,
            }
        } else {
            false
        }
        // tag: &str, attrs: &HashMap<&str, &str>
    }
}

#[derive(Debug)]
pub struct Cond {
    vec: Vec<CondItem>,
}

impl Cond {
    fn new() -> Self {
        Cond { vec: Vec::new() }
    }
    fn push(&mut self, str: &str) {
        self.vec.push(CondItem::new(str));
    }
    pub fn test(&self, node: Rc<Node>) -> bool {
        self.vec.iter().all(move |ref item| item.test(node.clone()))
    }
    pub fn is_next_layer(&self) -> bool {
        if self.vec.len() != 1 {
            return false;
        }
        match self.vec[0] {
            CondItem::NextLayer => true,
            _ => false,
        }
    }
    pub fn parse(str: &str) -> Vec<Cond> {
        let re = Regex::new(r"((#|\.)?[^#.\[ >]+)|(\[.+\])| |>").unwrap();
        let mut vec = Vec::new();
        let mut cond = Cond::new();
        for (s, e) in re.find_iter(str) {
            let item = &str[s..e];
            match (item, cond.vec.len()) {
                (" ", 0) => {
                    continue;
                }
                (" ", _) => {
                    vec.push(cond);
                    cond = Cond::new();
                    continue;
                }
                (">", 0) => {
                    cond.push(item);
                    vec.push(cond);
                    cond = Cond::new();
                    continue;
                }
                (">", _) => {
                    vec.push(cond);
                    cond = Cond::new();
                    cond.push(item);
                    vec.push(cond);
                    cond = Cond::new();
                    continue;
                }
                (_, _) => {
                    cond.push(item);
                    continue;
                }
            }
        }
        if cond.vec.len() > 0 {
            vec.push(cond);
        }
        vec
    }
}
