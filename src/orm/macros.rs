#[macro_use]
use mysql;
use orm::Entity;

#[macro_export]
macro_rules! entity {
    (struct $ENTITY:ident{
        $($FIELD:ident:$TYPE:ty,)*
    })=>{
        #[derive(Debug, Clone)]
        struct $ENTITY {
            id: u64,
            $(pub $FIELD: $TYPE,)*
        }

        impl $ENTITY {
            fn get_field_names() -> Vec<&'static str> {
                vec![$(stringify!($FIELD)),*]
            }
            fn get_param(&self)->Vec<(String, mysql::Value)>{
                let vec = Vec::new();
                vec
            }
        }

        impl Entity for $ENTITY{
            fn set_id(&mut self, id: u64) {
                self.id = id;
            }
            fn get_prepare_fields()->String{
                let mut vec = Vec::new();
                $(vec.push(format!("{} = :{}", $FIELD, $FIELD));)*
                vec.join(" AND ")
            }
            fn get_params(&self)->Vec<(String, mysql::Value)>{
                let vec = Vec::new();
                vec
                // params!{
                    // $(stringify!($FIELD)=>self.$FIELD),*
                // }
            }
            fn get_insert_sql(&self)->String{
                format!("INSERT INTO {} SET{}", stringify!($ENTITY), Self::get_prepare_fields())
            }
        }
    }
}

// entity!{struct Person{
//         name:String,
//         age:i32,
// }}

pub fn test() {
    let pool = mysql::Pool::new("mysql://root:root@localhost:3306/test").unwrap();

}
