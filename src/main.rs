mod http;
extern crate hyper;

use http::Client;
use http::PipeTrait;
use http::ReadTrait;

extern crate select;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

fn main() {
    let client = Client::new();
    let res = client.get("http://www.baidu.com").unwrap().to_string().unwrap();
    // let document = Document::from(include_str!("../index.html"));
    // let s = "<html></html>".to_string();
    let document = Document::from(res.as_ref());
    let div = document.find(Attr("id", "head")).find(Name("div")).first().unwrap();
    let ret = div.attr("class");
    println!("{:?}", ret);
}