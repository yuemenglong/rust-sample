use std::string::String;

use std::rc::Rc;
use std::fmt;

use rquery::selector::Selector;
use rquery::node::Node;
use rquery::node::NodeEnum;

impl fmt::Debug for SelectResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = Ok(());
        for (i, node) in self.res.iter().enumerate() {
            ret = match i {
                0 => write!(f, "\n{}", Self::node_html(node.clone())),
                _ => write!(f, "{}", Self::node_html(node.clone())),
            }
        }
        ret
    }
}

pub struct SelectResult {
    res: Vec<Rc<Node>>,
}

impl SelectResult {
    fn new(res: Vec<Rc<Node>>) -> SelectResult {
        SelectResult { res: res }
    }
    fn from_node(node: Rc<Node>) -> SelectResult {
        SelectResult { res: vec![node] }
    }
    fn node_element(node: Rc<Node>) -> String {
        let mut res = String::new();
        match node.content {
            NodeEnum::Element(ref tag, ref attrs) => {
                res.push_str(format!("<{}", tag).as_ref());
                for (name, value) in attrs {
                    res.push_str(format!(" {}='{}'", name, value).as_ref());
                }
                res.push_str("/>");
            }
            _ => {}
        }
        res
    }
    fn node_html(node: Rc<Node>) -> String {
        fn recursive_html(node: Rc<Node>, res: &mut String) {
            match node.content {
                NodeEnum::Element(ref tag, ref attrs) => {
                    res.push_str(format!("<{}", tag).as_ref());
                    for (name, value) in attrs {
                        res.push_str(format!(" {}='{}'", name, value).as_ref());
                    }
                    res.push_str(">");
                    for child in node.children.borrow().iter() {
                        recursive_html(child.clone(), res);
                    }
                    res.push_str(format!("</{}>", tag).as_ref());
                }
                NodeEnum::Text(ref text) => {
                    res.push_str(text);
                }
                _ => {}
            }
        }
        let mut res = String::new();
        recursive_html(node, &mut res);
        res
    }
    fn node_text(node: Rc<Node>) -> String {
        fn recursive_text(node: Rc<Node>, res: &mut String) {
            match node.content {
                NodeEnum::Text(ref text) => res.push_str(text),
                _ => {}
            }
            for child in node.children.borrow().iter() {
                recursive_text(child.clone(), res);
            }
        }
        let mut res = String::new();
        recursive_text(node, &mut res);
        res
    }
    fn check(&self) {
        if self.res.len() > 1 {
            panic!("{}", "There Are More Than One Children");
        }
    }
    pub fn children(&self, selector: &str) -> SelectResult {
        let mut res = Vec::new();
        let selector = Selector::new(selector);
        for node in self.res.iter() {
            let vec = selector.select(node.clone());
            if vec.len() > 0 {
                res.extend(vec);
            }
        }
        SelectResult::new(res)
    }
    pub fn get(&self, idx: usize) -> SelectResult {
        SelectResult { res: vec![self.res[idx].clone()] }
    }
    pub fn attr(&self, name: &str) -> Option<&String> {
        self.check();
        if self.res.len() == 0 {
            return None;
        }
        match self.res[0].content {
            NodeEnum::Element(_, ref attrs) => attrs.get(name),
            _ => None,
        }
    }
    pub fn tag(&self) -> Option<&String> {
        self.check();
        if self.res.len() == 0 {
            return None;
        }
        match self.res[0].content {
            NodeEnum::Element(ref tag, _) => Some(tag),
            _ => None,
        }
    }
    pub fn text(&self) -> Option<String> {
        self.check();
        if self.res.len() == 0 {
            return None;
        }
        Some(Self::node_text(self.res[0].clone()))
    }
    pub fn html(&self) -> Option<String> {
        self.check();
        if self.res.len() == 0 {
            return None;
        }
        Some(Self::node_html(self.res[0].clone()))
    }
    pub fn element(&self) -> Option<String> {
        self.check();
        if self.res.len() == 0 {
            return None;
        }
        Some(Self::node_element(self.res[0].clone()))
    }
}

impl IntoIterator for SelectResult {
    type Item = SelectResult;
    type IntoIter = ::std::vec::IntoIter<SelectResult>;

    fn into_iter(self) -> Self::IntoIter {
        self.res
            .iter()
            .map(|node| SelectResult::from_node(node.clone()))
            .collect::<Vec<SelectResult>>()
            .into_iter()
    }
}

pub fn create_context(root: Rc<Node>) -> Box<Fn(&str) -> SelectResult> {
    Box::new(move |selector| {
        let selector = Selector::new(selector);
        let vec = selector.select(root.clone());
        SelectResult::new(vec)
    })
}
