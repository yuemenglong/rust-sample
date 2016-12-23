#[macro_use]
extern crate mysql;

use mysql::value::from_row;

#[macro_use]
pub mod orm;

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

        impl $ENTITY{
            fn get_table()->String{
                stringify!($ENTITY).to_string()
            }
            fn set_id(&mut self, id:u64){
                self.id = Some(id);
            }
            fn get_fields()->String{
                let mut vec = Vec::new();
                vec.push("id".to_string());
                $(vec.push(stringify!($FIELD).to_string());)*
                vec.join(", ")
            }
            fn get_id_cond(&self)->String{
                format!("id = {}", self.id.unwrap())
            }
            fn get_prepare()->String{
                let mut vec = Vec::new();
                $(vec.push(format!("{} = :{}", stringify!($FIELD), stringify!($FIELD)));)*
                vec.join(", ")
            }
            fn get_params(&self)->Vec<(String, mysql::Value)>{
                let mut vec = Vec::new();
                $(vec.push((stringify!($FIELD).to_string(), mysql::Value::from(&self.$FIELD)));)*
                vec
            }
            fn get_params_id(&self)->Vec<(String, mysql::Value)>{
                vec![("id".to_string(), mysql::Value::from(self.id))]
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
    name:String,
});
fn main() {
    let pool = mysql::Pool::new("mysql://root:root@localhost:3306/test").unwrap();
    let mut stmt = pool.prepare("select id, age, name from person").unwrap();
    let res = stmt.execute(()).unwrap();
    // println!("{:?}", res.count());
    res.map(|row|{
        let mut row = row.unwrap();
        let p = Person::from_row(row);
        println!("{:?}", p);
    }).collect::<Vec<_>>();
    // println!("{:?}", res);
    // orm::macros::test();
    let p = Person {
        id: None,
        age: 10,
        name: "asdf".to_string(),
    };
    println!("insert into {} set {}", Person::get_table(), Person::get_prepare());
}
