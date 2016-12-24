#[macro_use]
extern crate orm;

use orm::*;

entity!{struct Person{
    age:i32,
    name:String,
}}
entity!{struct Person2{
    age:i32,
    name:String,
}}

fn main() {
    let db = orm::open("root", "root", "localhost", 3306, "test").unwrap();
    let p = Person {
        id: None,
        age: 20,
        name: "asdf".to_string(),
    };
    let mut p = db.insert(&p).unwrap();
    p.age = 30;
    db.update(&p);
}
