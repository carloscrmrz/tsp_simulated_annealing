use postgres::{Client, Error, NoTls, Row};

pub const DATABASE_URL: &str = "postgresql://carloscabrera@localhost/carloscabrera";

pub fn connect() -> Result<Client, Error> {
    Client::connect(DATABASE_URL, NoTls)
}


pub trait FromRow {
    const QUERY: &'static str;
    fn from_row(row: &Row) -> Self;
}

pub fn load_all<T: FromRow>(client: &mut Client) -> Result<Vec<T>, Error> {
    let rows = client.query(T::QUERY, &[])?;
    Ok(rows.iter().map(T::from_row).collect())
}

