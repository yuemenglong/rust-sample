use std::collections::HashMap;
use std::io::{Read, Write};

use hyper::Client as HyperClient;
use hyper::client::response::Response;
use hyper::header::{Cookie, CookiePair};

pub type Error = &'static str;

pub struct Client {
    client: HyperClient,
    cookies: HashMap<String, String>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: HyperClient::new(),
            cookies: HashMap::new(),
        }
    }
    pub fn get(&self, url: &str) -> Result<Response, Error> {
        let mut cookies = Vec::<CookiePair>::new();
        for (k, v) in self.cookies.iter() {
            cookies.push(CookiePair::new(k.clone(), v.clone()));
        }
        let mut builder = self.client.get(url);
        if 0 != cookies.len() {
            builder = builder.header(Cookie(cookies));
        }
        builder.send().or(Err("Error"))
    }
    pub fn add_cookie(&mut self, key: &str, value: &str) {
        self.cookies.insert(key.to_string(), value.to_string());
    }
    pub fn set_cookie_string(&mut self, cookie_string: &str) {
        for cookie in cookie_string.split(";") {
            if 0 == cookie.len() {
                continue;
            }
            let kv = cookie.trim().split("=").collect::<Vec<&str>>();
            self.cookies.insert(kv[0].to_string(), kv[1].to_string());
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
