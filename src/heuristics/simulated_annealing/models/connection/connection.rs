use rusqlite::{Error, Row};

use super::super::db::FromRow;

#[derive(Debug, Clone)]
pub struct Connection {
    pub id_city_1: i32,
    pub id_city_2: i32,
    pub distance: f64,
}

impl FromRow for Connection {
    const QUERY: &'static str = "SELECT id_city_1, id_city_2, distance FROM connections";

    fn from_row(row: &Row) -> Result<Connection, Error> {
        Ok(Connection {
            id_city_1: row.get("id_city_1")?,
            id_city_2: row.get("id_city_2")?,
            distance: row.get("distance")?,
        })
    }
}
