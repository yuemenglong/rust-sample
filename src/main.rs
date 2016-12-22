#[macro_use]
extern crate mysql;

pub mod orm;

macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

macro_rules! say {
    ($($e:expr),*) => {{
        $(println!("{:?}", $e);)*
    }}
}

macro_rules! block {
    ($s:stmt) => {{$s;}};
}

trait Entity {
    // add code here
}

macro_rules! entity {
    (struct $ENTITY:ident{
        $($FIELD:ident:$TYPE:ty),*
    })=>{
        #[derive(Debug)]
        struct $ENTITY {
            pub id: Option<i64>,
            $(pub $FIELD: $TYPE),*
        }

        impl $ENTITY {
            // This is purely an example—not a good one.
            fn get_field_names() -> Vec<&'static str> {
                vec![$(stringify!($FIELD)),*]
            }
            fn get_param(&self)->Vec<(String, mysql::Value)>{
                params!{
                    "id"=>self.id,
                    $(stringify!($FIELD)=>self.$FIELD),*
                }
            }
            fn insert(&self){
                let mut kv = String::new();
                $(kv.push_str(&format!(" {} = :{}", stringify!($FIELD), stringify!($FIELD)));),*
                let sql = format!("INSERT INTO {} SET{}", stringify!($ENTITY), kv);
                println!("{:?}", sql);
            }
        }
    }
}
// let mut kv = String::new();
//                 $({kv.push(format!(" {} = :{},", stringify!($FIELD), stringify!($FIELD)))}),*
//                 println!("{:?}", kv);
entity!{
struct Person {
    age: i32
}}
fn main() {
    // let pool = mysql::Pool::new("mysql://root:root@localhost:3306/test").unwrap();
    // say!(1, 2, 3, "hi", "wuwuw");
    let x = 1;
    // block!{let y = x}

    let p = Person { id: None, age: 1 };
    println!("{:?}", p.get_param());
    p.insert();
}
