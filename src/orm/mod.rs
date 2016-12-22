use mysql;
use mysql::Pool;
use mysql::error::Error;
use mysql::QueryResult;
use std::cell::RefCell;

pub trait Entity {
    // add code here
    fn set_id(&mut self, id: u64);
    fn get_insert_sql(&self) -> String;
    fn get_insert_params(&self) -> Vec<(String, mysql::Value)>;
}

pub struct DB {
    pool: Pool,
}

impl DB {
    pub fn insert<E: Entity + Clone>(&self, entity: &E) -> Result<E, Error> {
        let stmt = self.pool.prepare(entity.get_insert_sql());
        if stmt.is_err() {
            return Err(stmt.unwrap_err());
        }
        let mut stmt = stmt.unwrap();
        let res = stmt.execute(entity.get_insert_params());
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        let res = res.unwrap();
        println!("{:?}", res);
        let mut ret = (*entity).clone();
        ret.set_id(res.last_insert_id());
        Ok(ret)
    }
}

pub fn open(user: &str, pwd: &str, host: &str, port: u16, db: &str) -> Result<DB, Error> {
    let conn_str = format!("mysql://{}:{}@{}:{}/{}", user, pwd, host, port, db);
    match mysql::Pool::new(conn_str.as_ref()) {
        Ok(pool) => Ok(DB { pool: pool }),
        Err(err) => Err(err),
    }
}
