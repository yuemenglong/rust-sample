extern crate html5ever;
extern crate tendril;
extern crate hyper;
extern crate regex;

pub mod http;
pub mod rquery;
// use http::Client;
use std::fs::File;
// use std::io::Write;
use std::str::Chars;

fn main() {
    // let mut client = Client::new();
    // let mut res = client.get("http://www.baidu.com").unwrap();
    // File::create("index.html").unwrap().write(res.get_raw().unwrap().as_ref());
    // println!("{:?}", res.get_body());
    // let s = rquery::load(&mut res.get_read());
    let s= rquery::load(&mut File::open("index.html").unwrap());
    let node = s("[id=wrapper]");
    println!("{:?}", node);

}


