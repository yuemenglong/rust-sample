#[macro_use]
extern crate mysql;

use mysql::value::from_row;
use mysql::Value;

pub mod orm;

use orm::cond::Cond;
use orm::Entity;

macro_rules! cond {
    ($FIELD:ident=$E:expr) => {{
        Cond::Eq(stringify!($FIELD).to_string(), Value::from($E))
    }}
}

macro_rules! row_take {
    ($FIELD:ident, Option<$TYPE:ty>, $ROW:ident) => {{
        let ret = $ROW.take(stringify!($FIELD));
        ret
    }};
    ($FIELD:ident, $TYPE:ty, $ROW:ident) => {{
        let ret = $ROW.take(stringify!($FIELD));
        ret.unwrap()
    }};
}

macro_rules! entity {
    (struct $ENTITY:ident{
        $($FIELD:ident:$TYPE:ty,)*
    })=>{
        #[derive(Debug, Clone)]
        struct $ENTITY{
            id: Option<u64>,
            $($FIELD:$TYPE,)*
        }

        impl Entity for $ENTITY{
            fn get_table()->String{
                stringify!($ENTITY).to_string()
            }
            fn set_id(&mut self, id:u64){
                self.id = Some(id);
            }
            fn get_id_cond(&self)->String{
                format!("`id` = {}", self.id.unwrap())
            }
            fn get_fields()->String{
                let mut vec = Vec::new();
                vec.push("`id`".to_string());
                $(vec.push(format!("`{}`", stringify!($FIELD)));)*
                vec.join(", ")
            }
            fn get_prepare()->String{
                let mut vec = Vec::new();
                $(vec.push(format!("`{}` = :{}", stringify!($FIELD), stringify!($FIELD)));)*
                vec.join(", ")
            }
            fn get_params(&self)->Vec<(String, Value)>{
                let mut vec = Vec::new();
                $(vec.push((stringify!($FIELD).to_string(), Value::from(&self.$FIELD)));)*
                vec
            }
            fn get_params_id(&self)->Vec<(String, Value)>{
                vec![("id".to_string(), Value::from(self.id))]
            }
            fn from_row(mut row: mysql::conn::Row)->$ENTITY{
                $ENTITY{
                    id: row_take!(id, Option<u64>, row),
                    $($FIELD: row_take!($FIELD, $TYPE, row),)*
                }
            }
        }
    }
}

struct B {
    age: i32,
    name: String,
}
entity!(struct Person{
    age:i32,
    name:Option<String>,
});
fn main() {
    let db = orm::open("root", "root", "localhost", 3306, "test").unwrap();
    let p = Person {
        id: None,
        age: 20,
        name: Some("bill".to_string()),
    };
    let mut p = db.insert(&p).unwrap();
    println!("{:?}", p);
    p.age = 21;
    let ret = db.update(&p);
    println!("{:?}", ret);
    let p: Person = db.get(p.id.unwrap()).unwrap().unwrap();
    println!("{:?}", p);
    let ret = db.delete(p);
    println!("{:?}", ret);
    // let cond = cond!{id=1};
    // println!("{:?}", cond.to_param());
    // let pool = mysql::Pool::new("mysql://root:root@localhost:3306/test").unwrap();
    // let mut stmt = pool.prepare("select id, age, name from person").unwrap();
    // let res = stmt.execute(()).unwrap();
    // // println!("{:?}", res.count());
    // res.map(|row| {
    //         let mut row = row.unwrap();
    //         let p = Person::from_row(row);
    //         println!("{:?}", p);
    //     })
    //     .collect::<Vec<_>>();
    // // println!("{:?}", res);
    // // orm::macros::test();
    // let p = Person {
    //     id: None,
    //     age: 10,
    //     name: "asdf".to_string(),
    // };
    // println!("insert into {} set {}", Person::get_table(), Person::get_prepare());
}
