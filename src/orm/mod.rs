use mysql;
use mysql::Value;
use mysql::Pool;
use mysql::error::Error;
use mysql::QueryResult;
use std::cell::RefCell;

pub mod cond;
pub mod macros;

pub trait Entity {
    fn get_table() -> String;
    fn get_fields() -> String;
    fn get_prepare() -> String;
    fn set_id(&mut self, id: u64);
    fn get_id_cond(&self) -> String;
    fn get_params(&self) -> Vec<(String, Value)>;
    fn get_params_id(&self) -> Vec<(String, Value)>;
    fn from_row(mut row: mysql::conn::Row) -> Self;
}

pub struct DB {
    pool: Pool,
}

impl DB {
    pub fn insert<E: Entity + Clone>(&self, entity: &E) -> Result<E, Error> {
        let sql = format!("INSERT INTO {} SET {}", E::get_table(), E::get_prepare());
        println!("{}", sql);
        let res = self.pool.prep_exec(sql, entity.get_params());
        match res {
            Ok(res) => {
                let mut ret = (*entity).clone();
                ret.set_id(res.last_insert_id());
                Ok(ret)
            }
            Err(err) => Err(err),
        }
    }
    pub fn update<E: Entity>(&self, entity: &E) -> Result<u64, Error> {
        let sql = format!("UPDATE {} SET {} WHERE {}",
                          E::get_table(),
                          E::get_prepare(),
                          entity.get_id_cond());
        println!("{}", sql);
        let res = self.pool.prep_exec(sql, entity.get_params());
        match res {
            Ok(res) => Ok(res.affected_rows()),
            Err(err) => Err(err),
        }
    }
    pub fn get<E: Entity>(&self, id: u64) -> Result<Option<E>, Error> {
        let sql = format!("SELECT {} FROM {} WHERE `id` = {}",
                          E::get_fields(),
                          E::get_table(),
                          id);
        println!("{}", sql);
        let res = self.pool.first_exec(sql, ());
        match res {
            Ok(option) => Ok(option.map(|row| E::from_row(row))),
            Err(err) => Err(err),
        }
    }
    pub fn delete<E: Entity>(&self, entity: E) -> Result<u64, Error> {
        let sql = format!("DELETE FROM {} WHERE {}",
                          E::get_table(),
                          entity.get_id_cond());
        println!("{}", sql);
        let res = self.pool.prep_exec(sql, ());
        match res {
            Ok(res) => Ok(res.affected_rows()),
            Err(err) => Err(err),
        }
    }
}

pub fn open(user: &str, pwd: &str, host: &str, port: u16, db: &str) -> Result<DB, Error> {
    let conn_str = format!("mysql://{}:{}@{}:{}/{}", user, pwd, host, port, db);
    match mysql::Pool::new(conn_str.as_ref()) {
        Ok(pool) => Ok(DB { pool: pool }),
        Err(err) => Err(err),
    }
}
