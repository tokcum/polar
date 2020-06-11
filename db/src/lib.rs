use model::ObjectType;

use rusqlite::{Connection, NO_PARAMS};
use serde_json;

pub fn push(data: &ObjectType) {
    let mut conn = Connection::open("polar.db").unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS objecttypes ( \
                      id INTEGER PRIMARY KEY, \
                      data TEXT NOT NULL UNIQUE \
                      )", NO_PARAMS).unwrap();

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
