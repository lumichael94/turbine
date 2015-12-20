extern crate postgres;

use self::postgres::Connection;

pub struct Log {
    pub hash:       String,     //  Log Hash
    pub state:      String,     //  Hash of the state
    pub account:    String,     //  Origin account address
    pub nonce:      i64,        //  Nonce of log
    pub max_fuel:   i64,        //  Maximum fuel to use
    pub code:       String,     //  Code of Log
    pub signature:  Vec<u8>,    //  Modify with Electrum style signatures
}

// Retreives an log.
// Input    hash    Hash of log to retrieve.
// Input    conn    Database connection.
// Output   log     Retrieved log struct.
pub fn get_log (hash : &str, conn: &Connection) -> Log{
    let maybe_stmt = conn.prepare("SELECT * FROM log WHERE hash = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let i: String = hash.to_string();
    let rows = stmt.query(&[&i]).unwrap();
    let row = rows.get(0);
    Log {
        hash        :   row.get(0),
        state       :   row.get(1),
        account     :   row.get(2),
        nonce       :   row.get(3),
        max_fuel    :   row.get(4),
        code        :   row.get(5),
        signature   :   row.get(6),
    }
}

// Saves log struct
// Input    l               Log struct to save.
// Input    conn            Database connection.
pub fn save_log (save_l : &Log, conn: &Connection){
    let l = save_l.clone();
    conn.execute("INSERT INTO log \
                 (hash, state, account, nonce, maximum_fuel, code, signature) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
                  &[&l.hash, &l.state, &l.account, &l.nonce,
                    &l.max_fuel, &l.code, &l.signature]).unwrap();
}

// Drops specified log
// Input    hash    Hash of log to retrieve.
// Input    conn        Database connection.
pub fn remove_log (hash : &str, conn: &Connection){
    conn.execute("DELETE FROM log WHERE hash = $1", &[&(hash.to_string())]).unwrap();
}

// Creates an log table.
// Input    conn    Database connection.
pub fn create_log_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS log (
                  hash      text,
                  state     text,
                  account   text,
                  nonce     bigint,
                  max_fuel  bigint,
                  code      text,
                  signature bytea
                  )", &[]).unwrap();
}

// Drops an account table.
// Input    conn    Database connection.
pub fn drop_log_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS log", &[]).unwrap();
}
