extern crate postgres;

use self::postgres::Connection;

pub struct State {
    pub hash            :   String,
    pub nonce           :   i64,
    pub p_state         :   String,    // Hash of parent state
    pub a_hash          :   String,    // Hash of accounts
    pub l_hash          :   String,    // Hash of logs
    pub fuel_exp        :   i64,       // Fuel Expenditure
}

// Saves account state.
// Input    save_s          State struct to save.
// Input    conn            Database connection.
pub fn save_state(save_s: &State, conn: &Connection){
    let s = save_s.clone();

    conn.execute("INSERT INTO state \
                  (hash, nonce, parent_state, accounts_hash, logs_hash, fuel_expenditure) \
                  VALUES ($1, $2, $3, $4, $5, $6)",
                  &[&s.hash, &s.nonce, &s.p_state, &s.p_state, &s.l_hash, &s.fuel_exp]).unwrap();
}

// Retreives a state.
// Input    hash        Hash of state to retrieve.
// Input    conn        Database connection.
// Output   state       Retrieved state struct.
pub fn get_state(hash: &str, conn: &Connection) -> State{
    let maybe_stmt = conn.prepare("SELECT * FROM state WHERE hash = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let h: String = hash.to_string();
    let rows = stmt.query(&[&h]).unwrap();
    let row = rows.get(0);
    State {
        nonce:      row.get(0),
        hash:       row.get(1),
        p_state:    row.get(2),
        a_hash:     row.get(3),
        l_hash:     row.get(4),
        fuel_exp:   row.get(5),
    }
}

// Drops specified state.
// Input    hash        Hash of state to drop.
// Input    conn        Database connection.
pub fn drop_state(hash: &str, conn: &Connection){
    conn.execute("DELETE FROM state \
                  WHERE hash = $1",
                  &[&(hash.to_string())]).unwrap();
}

// Creates a state table.
// Input    conn    Database connection.
pub fn create_state_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS state (
                    nonce               BIGINT,
                    hash                text,
                    parent_state        text,
                    accounts_hash       text,
                    logs_hash           text,
                    fuel_expenditure    BIGINT
                  )", &[]).unwrap();
}

// Drop a state table.
// Input    conn    Database connection.
pub fn drop_state_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS state", &[]).unwrap();
}
