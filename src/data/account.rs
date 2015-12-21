use postgres::Connection;
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable};


#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone)]
pub struct Account{
    pub address     : String,
    pub fuel        : i64,
    pub log_n       : i64,
    pub state_n     : i64,
    pub pc          : i64,
    pub public_key  : Vec<u8>,
    pub code        : Vec<u8>,
    pub memory      : String,
}

// Saves account struct.
// Input    acc             Account struct to save.
// Input    conn            Database connection.
pub fn save_account(acc: &Account, conn: &Connection){
    let a = acc.clone();
    if account_exist(&a.address, &conn){
        conn.execute("UPDATE account \
            SET fuel = $2, log_nonce = $3, \
            state_nonce = $4, pc = $5, public_key = $6, code = $7, memory = $8 \
            WHERE address = $1",
                      &[&a.address, &a.fuel, &a.log_n, &a.state_n,
                        &a.pc, &a.public_key, &a.code, &a.memory]).unwrap();
    } else {
    conn.execute("INSERT INTO account \
                  (address, fuel, log_nonce, state_nonce, pc, public_key, code, memory) \
                  VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                  &[&a.address, &a.fuel, &a.log_n, &a.state_n,
                    &a.pc, &a.public_key, &a.code, &a.memory]).unwrap();
    }
}

// Drops specified account.
// Input    address     Address of account to drop
// Input    conn        Database connection.
pub fn drop_account(address: &str, conn: &Connection){
    conn.execute("DELETE FROM account \
                  WHERE address = $1",
                  &[&address])
                  .unwrap();
}

// Creates an account table.
// Input    conn    Database connection.
pub fn create_account_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS account (
                    address         text primary key,
                    fuel            bigint,
                    log_nonce       bigint,
                    state_nonce     bigint,
                    pc              bigint
                    public_key      bytea,
                    code            bytea,
                    memory          text,
                  )", &[]).unwrap();
}

// Drops an account table.
// Input    conn    Database connection.
pub fn drop_account_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS account", &[]).unwrap();
}

// Retreives an account.
// Input    add         Address of account to retrieve.
// Input    conn        Database connection.
// Output   account     Retrieved account struct.
pub fn get_account(add: &str, conn: &Connection) -> Account{
    let maybe_stmt = conn.prepare("SELECT * FROM account WHERE address = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let a: String = add.to_string();
    let rows = stmt.query(&[&a]).unwrap();
    let row = rows.get(0);
    Account {
        address     : row.get(0),
        fuel        : row.get(1),
        log_n       : row.get(2),
        state_n     : row.get(3),
        public_key  : row.get(4),
        code        : row.get(5),
        memory      : row.get(6),
        pc          : row.get(7),
    }
}

// Checks if an account exists
// Input    add         Address of account to retrieve
// Input    conn        Database connection.
// Output   Boolean     Account exists?
pub fn account_exist(add: &str, conn: &Connection) -> bool{
    let maybe_stmt = conn.prepare("SELECT * FROM account WHERE address = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[&add]);
    match rows {
        Err(_) => false,
        Ok(r) => {
            if r.len() != 0 {
                true
            } else {
                false
            }
        },
    }
}

// Converts account struct to byte vector.
// Input    acc         Account struct to convert.
// Output   Vec<u8>     Converted byte vector.
pub fn acc_to_vec(a: &Account)-> Vec<u8>{
    encode(a, SizeLimit::Infinite).unwrap()
}

// Converts byte vector to account struct.
// Input    raw_acc     Byte vector to convert.
// Output   account     Converted account.
pub fn vec_to_acc(raw_acc: &Vec<u8>) -> Account{
    decode(&raw_acc[..]).unwrap()
}
