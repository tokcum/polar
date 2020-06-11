use model::ObjectType;

use rusqlite::{Connection, NO_PARAMS};
use serde_json;

fn open() -> Connection {
    let conn = Connection::open("polar.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS objecttypes ( \
                      id INTEGER PRIMARY KEY, \
                      data TEXT NOT NULL UNIQUE \
                      )", NO_PARAMS).unwrap();
    conn
}

pub fn select() -> ObjectType {
    let conn = open();
    let ot: ObjectType = conn.query_row("SELECT data FROM objecttypes LIMIT 1", NO_PARAMS, |row| row.get(0)).unwrap();
    ot
}

pub fn insert(data: &ObjectType) {
    let mut conn = open();
    let tx = conn.transaction().unwrap();
    let d = serde_json::to_string(&data).unwrap();
    tx.execute("INSERT INTO objecttypes (data) VALUES (?1)", &[&d]).unwrap();
    tx.commit().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
