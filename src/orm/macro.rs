#[macro_use]
use mysql;
use orm::Entity;

pub macro_rules! entity {
    (struct $ENTITY:ident{
        $($FIELD:ident:$TYPE:ty),*
    })=>{
        #[derive(Debug)]
        struct $ENTITY {
            $(pub $FIELD: $TYPE),*
        }

        impl $ENTITY {
            // This is purely an example—not a good one.
            fn get_field_names() -> Vec<&'static str> {
                vec![$(stringify!($FIELD)),*]
            }
            fn get_param(&self)->Vec<(String, mysql::Value)>{
                params!{
                    $(stringify!($FIELD)=>self.$FIELD),*
                }
            }
        }

        impl Entity for $ENTITY{
            fn get_insert_sql(&self)->String{
                let mut kv = String::new();
                $(kv.push_str(&format!(" {} = :{}", stringify!($FIELD), stringify!($FIELD)));),*
                format!("INSERT INTO {} SET{}", stringify!($ENTITY), kv)
            }
            fn get_insert_params(&self)->Vec<(String, mysql::Value)>{
                params!{
                    $(stringify!($FIELD)=>self.$FIELD),*
                }
            }
        }
    }
}
