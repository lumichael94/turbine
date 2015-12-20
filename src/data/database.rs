extern crate postgres;

use self::postgres::{Connection, SslMode};

// Connect to database.
// Output   Connection      Database connection.
pub fn connect_db() -> Connection{
    let conn = Connection::connect("postgresql://postgres:api@localhost", &SslMode::None).unwrap();
    return conn;
}

// Close database connection.
// Input    Connection      Database connection to be consumed.
pub fn close_db(conn: Connection){
    let _ = Connection::finish(conn);
}
