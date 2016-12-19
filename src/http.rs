use std;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Write};

use hyper::client::RequestBuilder;
use hyper::Client as HyperClient;
use hyper::client::response::Response;
use hyper::header::{Cookie, CookiePair};

use regex::Regex;

pub type Error = &'static str;

pub struct Client {
    client: HyperClient,
    cookies: RefCell<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct HttpResult {
    res: Response,
}

impl HttpResult {
    fn new(res: Response) -> Self {
        HttpResult { res: res }
    }
    pub fn get_header(&self, name: &str) -> Option<Vec<String>> {
        self.res
            .headers
            .get_raw(name)
            .map(|arr| {
                arr.iter()
                    .map(|vec| std::str::from_utf8(vec).unwrap().to_string())
                    .collect::<Vec<_>>()
            })
    }
    pub fn get_body(&mut self) -> Result<String, Error> {
        let mut ret = String::new();
        match self.res.read_to_string(&mut ret) {
            Ok(_) => Ok(ret),
            Err(_) => Err("Error"),
        }
    }
    pub fn get_raw(&mut self) -> Result<Vec<u8>, Error> {
        let mut ret: Vec<u8> = Vec::new();
        match self.res.read_to_end(&mut ret) {
            Ok(_) => Ok(ret),
            Err(_) => Err("Error"),
        }
    }
    pub fn get_read(self) -> Response {
        self.res
    }
}



impl Client {
    pub fn new() -> Client {
        Client {
            client: HyperClient::new(),
            cookies: RefCell::new(HashMap::new()),
        }
    }
    fn handle_set_cookie(&self, res: &Response) {
        let set_cookies = res.headers.get_raw("Set-Cookie");
        if set_cookies.is_none() {
            return;
        }
        let set_cookies = set_cookies.unwrap();
        for vec in set_cookies.iter() {
            let set_cookie = std::str::from_utf8(vec).unwrap();
            let re = Regex::new(r"^\w+=[^;]+").unwrap();
            // let () = re.find(set_cookie);
            let kv = re.find(set_cookie);
            if kv.is_none() {
                continue;
            }
            let (s, e) = kv.unwrap();
            let kv: Vec<_> = set_cookie[s..e].split("=").collect();
            if kv.len() != 2 {
                continue;
            }
            self.cookies.borrow_mut().insert(kv[0].to_string(), kv[1].to_string());
        }
    }
    fn request(&self, builder: RequestBuilder) -> Result<HttpResult, Error> {
        let mut cookies = Vec::new();
        for (k, v) in self.cookies.borrow().iter() {
            cookies.push(CookiePair::new(k.clone(), v.clone()));
        }
        let res = match cookies.len() {
            0 => builder.send(),
            _ => builder.header(Cookie(cookies)).send(),
        };
        if res.is_err() {
            return Err("Error");
        }
        let res = res.unwrap();
        self.handle_set_cookie(&res);
        return Ok(HttpResult::new(res));
    }
    pub fn get(&self, url: &str) -> Result<HttpResult, Error> {
        let builder = self.client.get(url);
        self.request(builder)
    }
    pub fn add_cookie(&mut self, key: &str, value: &str) {
        self.cookies.borrow_mut().insert(key.to_string(), value.to_string());
    }
    pub fn set_cookie_string(&mut self, cookie_string: &str) {
        for cookie in cookie_string.split(";") {
            if 0 == cookie.len() {
                continue;
            }
            let kv = cookie.trim().split("=").collect::<Vec<&str>>();
            self.cookies.borrow_mut().insert(kv[0].to_string(), kv[1].to_string());
        }
    }
}

pub trait ReadTrait {
    fn to_string(&mut self) -> Result<String, Error>;
    fn to_binary(&mut self) -> Result<Vec<u8>, Error>;
}

impl ReadTrait for Response {
    fn to_string(&mut self) -> Result<String, Error> {
        let mut ret = String::new();
        match self.read_to_string(&mut ret) {
            Ok(_) => Ok(ret),
            Err(_) => Err("Error"),
        }
    }
    fn to_binary(&mut self) -> Result<Vec<u8>, Error> {
        let mut ret: Vec<u8> = Vec::new();
        match self.read_to_end(&mut ret) {
            Ok(_) => Ok(ret),
            Err(_) => Err("Error"),
        }
    }
}

pub trait PipeTrait {
    fn print(&self) {
        println!("{:?}", "hello");
    }
    fn dest<W>(&mut self, dest: &mut W)
        where Self: Read,
              W: Write
    {
        let mut buffer = [0u8; 4096];
        loop {
            let size: usize = self.read(&mut buffer).unwrap();
            if size <= 0 {
                break;
            }
            dest.write(&buffer[..size]).unwrap();
        }
        return;
    }
}

impl PipeTrait for Response {}
