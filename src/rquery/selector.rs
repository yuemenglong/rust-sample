use std::rc::Rc;
use std::ops::Deref;
use regex::Regex;

use rquery::cond::Cond;
use rquery::node::Node;

#[derive(Debug)]
pub struct Selector<'a> {
    vec: Vec<Cond<'a>>,
}

impl<'a> Selector<'a> {
    pub fn new(str: &'a str) -> Selector<'a> {
        let re = Regex::new(r"[.#\[\]\w]+").unwrap();
        let vec = re.find_iter(str)
            .map(|(start, end)| Cond::new(&str[start..end]))
            .collect();
        Selector { vec: vec }
    }
    pub fn select(&self, root: Rc<Node>) -> Vec<Rc<Node>> {
        fn walk(root: Rc<Node>,
                node: Rc<Node>,
                cond_vec: &Vec<Cond>,
                res_vec: &mut Vec<Rc<Node>>) {
            if reverse_test(root.clone(), node.clone(), cond_vec, cond_vec.len() - 1) {
                res_vec.push(node.clone());
            }
            for child in node.children.borrow().iter() {
                walk(root.clone(), child.clone(), cond_vec, res_vec);
            }
        }
        fn reverse_test(root: Rc<Node>,
                        node: Rc<Node>,
                        cond_vec: &Vec<Cond>,
                        cond_pos: usize)
                        -> bool {
            // Compare Current Nod And Current Cond
            let cond = &cond_vec[cond_pos];
            let current_test = cond.test(node.clone());
            let parent = node.parent.borrow();
            let is_back_cond = cond_pos == cond_vec.len() - 1;
            let is_front_cond = cond_pos == 0;
            let is_root = node.deref() as *const _ == root.deref() as *const _;
            match (current_test, is_back_cond, is_front_cond, is_root) {
                // Current Is Fail And Not Match Back Cond, Fail
                (false, true, _, _) => false,
                // Current Is Fail But Has Match Back Cond, But Reach Root, Fail
                (false, false, _, true) => false,
                // Current Is Fail But Has Match Back Cond, And Not Reach Root, Recursive For Current Cond
                (false, false, _, false) => {
                    let parent = parent.as_ref().unwrap().upgrade().unwrap();
                    reverse_test(root, parent, cond_vec, cond_pos)
                }
                // Current Is Succ And Finish All Conds, Succ
                (true, _, true, _) => true,
                // Current Is Succ And Not Finish All Conds, But Reach Root, Fail
                (true, _, false, true) => false, 
                // Current Is Succ And Not Finish All Conds, And Not Reach Root, Recursive For Next Cond
                (true, _, false, false) => {
                    let parent = parent.as_ref().unwrap().upgrade().unwrap();
                    reverse_test(root, parent, cond_vec, cond_pos - 1)
                }
            }
        }
        let mut res_vec = Vec::new();
        walk(root.clone(), root.clone(), &self.vec, &mut res_vec);
        res_vec
    }
}