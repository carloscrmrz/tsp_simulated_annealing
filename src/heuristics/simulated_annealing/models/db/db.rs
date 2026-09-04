use rusqlite::{Connection, Error, Row};

pub const DATABASE_PATH: &str = "tsp.db";

pub fn connect(db_path: Option<String>) -> Result<Connection, Error> {
    let path = db_path.as_deref().unwrap_or(DATABASE_PATH);
    Connection::open(path.strip_prefix("sqlite://").unwrap_or(path))
}

pub trait FromRow: Sized {
    const QUERY: &'static str;
    fn from_row(row: &Row) -> Result<Self, Error>;
}

pub fn load_all<T: FromRow>(connection: &Connection) -> Result<Vec<T>, Error> {
    let mut statement = connection.prepare(T::QUERY)?;
    let mut rows = statement.query([])?;

    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        items.push(T::from_row(row)?);
    }
    Ok(items)
}
