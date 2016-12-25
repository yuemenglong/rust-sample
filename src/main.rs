#[macro_use]
extern crate orm;

use orm::*;

entity!{struct Person{
    age:i32,
    name:String,
    updatetime:DateTime,
}}

entity!{struct A{
    bid: u64,
    value: i32,
}}

entity!{struct B{
    value: i32,
}}

macro_rules! anno {
    (struct $ENTITY:ident{
        $($(#[$META:meta]),*
        $FIELD:ident:$TYPE:ty,)*
    })=>({
        $($(println!("{:?}", stringify!($META));)*)*
    });

}

fn main() {
    anno!{struct Test{
        #[cfg(target_os = "windows")],
        name:String,
    }}
    // let db = orm::open("root", "root", "localhost", 3306, "test").unwrap();
    // let res = db.pool.prep_exec("select a.id as a_id, a.bid as a_bid, a.value as a_value, b.id as b_id, b.value as b_value from a join b on a.bid = b.id", ()).unwrap();
    // let mut nameMap = HashMap::new();
    // nameMap.insert("id".to_string(), "a_id".to_string());
    // nameMap.insert("bid".to_string(), "a_bid".to_string());
    // nameMap.insert("value".to_string(), "a_value".to_string());
    // for row in res{
    //     let row = row.unwrap();
    //     let a = A::from_row_ex(row, &nameMap);
    //     println!("{:?}", a);
    // }
    // println!("{:?}", res);
    // let cb = |row: Result<Row, Error>| {
    //     let row = row.unwrap();
    //     println!("{:?}", row);
    // };
    // res.map(cb).collect::<Vec<_>>();
    // println!("{:?}", res.column_indexes());
    // let width = res.columns_ref().len();
    // for row in res{
    //     let row = row.unwrap();
    //     for i in 0..width{
    //         println!("{:?}", row[i]);
    //     }
    //     // println!("{:?}", row);
    // }
}

fn test() {
    println!("{:?}", datetime!(2001, 2, 3));
    println!("{:?}", datetime!(2001, 2, 3, 4, 5, 6));
    let db = orm::open("root", "root", "localhost", 3306, "test").unwrap();
    let res = db.drop_table::<Person>();
    println!("{:?}", res);
    let res = db.create_table::<Person>();
    println!("{:?}", res);
    let p = Person {
        id: None,
        age: 20,
        name: "hello".to_string(),
        updatetime: datetime!(2001, 2, 3, 4, 5, 6, 789),
    };
    let mut p = db.insert(&p).unwrap();
    println!("{:?}", p);
    p.age = 22;
    let res = db.update(&p).unwrap();
    println!("{:?}", res);
    let p: Person = db.get(p.id.unwrap()).unwrap().unwrap();
    println!("{:?}", p);
    let res = db.select::<Person>(cond!{id=p.get_id().unwrap()}).execute().unwrap();
    println!("{:?}", res);
    // let res = db.delete(p).unwrap();
    // println!("{:?}", res);
}
