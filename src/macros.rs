#[macro_export]
macro_rules! entity {
    (struct $ENTITY:ident{
        $($FIELD:ident:$TYPE:ty),*
    })=>{
        use orm::Entity;
        #[derive(Debug, Clone)]
        struct $ENTITY {
            id: Option<u64>,
            $(pub $FIELD: $TYPE),*
        }

        impl $ENTITY {
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
            fn set_id(&mut self, id: u64) {
                self.id = Some(id);
            }
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
