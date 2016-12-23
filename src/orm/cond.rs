use mysql::Value;

#[derive(Debug)]
pub enum Cond {
    Eq(String, Value),
}

impl Cond {
    pub fn to_prepare(&self) -> String {
        match self {
            &Cond::Eq(ref field, _) => format!("{} = :{}", field, field).to_string(),
        }

    }
    pub fn to_param(self) -> (String, Value) {
        match self {
            Cond::Eq(field, value) => (field, value),
        }
    }
}
