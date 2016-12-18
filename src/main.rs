
extern crate html5ever;
extern crate tendril;
extern crate hyper;
extern crate regex;

pub mod http;
pub mod rquery;

use http::Client;

struct Test<'a> {
    a: &'a str,
    b: &'a str,
}

impl<'a> Test<'a> {
    fn get_a(&self) -> &str {
        self.a
    }
    fn set_b(&'a mut self, b: &'a str) {
        self.b = b;
    }
}

fn main() {
    let mut client = Client::new();
    let mut res = client.get("http://www.baidu.com").unwrap();
    println!("{:?}", res.get_body());
    println!("{:?}", res.get_body());
}
