
extern crate html5ever;
extern crate tendril;
extern crate hyper;
extern crate regex;

pub mod http;
pub mod rquery;
use http::Client;

fn main() {
    let mut client = Client::new();
    let mut res = client.get("http://www.baidu.com").unwrap();
    let s = rquery::load(&mut res.get_read());
    let res = s("#lg");
    let r = res.attr("class");
    println!("{:?}", r);
}
