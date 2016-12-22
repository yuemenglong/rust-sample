use mysql;
use mysql::Pool;
use mysql::error::Error;
use mysql::QueryResult;
use std::cell::RefCell;

pub trait Entity {
    // add code here
    fn get_insert_sql(&self) -> String;
    fn get_insert_params(&self)->Vec<(String, mysql::Value)>;
}

struct DB {
    pool: Pool,
}

impl DB {
    fn insert<E: Entity>(&self, entity: &E)->Result<QueryResult, Error> {
        let sql = entity.get_insert_sql();
        let stmt = self.pool.prepare(sql);
        if stmt.is_err() {
            return Err(stmt.unwrap_err());
        }
        let param = entity.get_insert_params();
        let stmt = stmt.unwrap();
        stmt.execute(param)
    }
}

fn open(user: &str, pwd: &str, host: &str, port: u16, db: &str) -> Result<DB, Error> {
    let conn_str = format!("mysql://{}:{}@{}:{}/{}", user, pwd, host, port, db);
    match mysql::Pool::new(conn_str.as_ref()) {
        Ok(pool) => Ok(DB { pool: pool }),
        Err(err) => Err(err),
    }
}
