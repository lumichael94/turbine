extern crate rand;
extern crate crypto;
extern crate postgres;
extern crate chrono;
extern crate bincode;

use self::postgres::Connection;


pub struct Profile{
    pub name        : String,       // Name of profile
    pub password    : String,       // Password of profile
    pub account     : String,       // Account address
    pub active      : bool,         // Determines if it is the account in use
    pub secret_key  : Vec<u8>,      // Secret Key of account
    pub trusted     : String,       //List of trusted accounts
}

pub fn drop_profile(name: String, conn: &Connection){
    conn.execute("DELETE FROM profile \
                  WHERE name = $1",
                  &[&name])
                  .unwrap();
}

pub fn save_profile(save_p: &Profile, conn: &Connection){
    let p = save_p.clone();
    //TODO: Change away from hardcoded constants.
    let trusted: String = "127.0.0.1:8888".to_string();

    let exist: bool = profile_exist(&p.name, conn);
    if exist {
        conn.execute("UPDATE profile \
                        SET password = $2, account = $3, active = $4, secret_key = $5, trusted = $6 \
                        WHERE name = $1",
                      &[&p.name, &p.password, &p.account, &p.active, &p.secret_key, &trusted]).unwrap();
    } else {
        conn.execute("INSERT INTO profile \
                      (name, password, account, active, secret_key, trusted) \
                      VALUES ($1, $2, $3, $4, $5, $6)",
                       &[&p.name, &p.password, &p.account, &p.active, &p.secret_key, &trusted]).unwrap();
    }
}

// Creates an profile table.
// Input    conn    Database connection.
pub fn create_profile_table(conn: &Connection){
    conn.execute("CREATE TABLE IF NOT EXISTS profile (
                    name            text primary key,
                    password        text,
                    account         bool,
                    active          text,
                    secret_key      bytea,
                    trusted         text
                  )", &[]).unwrap();
}

// Drops an profile table.
// Input    conn    Database connection.
pub fn drop_profile_table(conn: &Connection){
    conn.execute("DROP TABLE IF EXISTS profile", &[]).unwrap();
}

// Checks if an profile exists
// Input    name        Name of profile to retrieve
// Input    conn        Database connection.
// Output   Boolean     Profile exists?
pub fn profile_exist(name: &str, conn: &Connection) -> bool{
    let maybe_stmt = conn.prepare("SELECT * FROM profile WHERE name = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[&name]);
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

// Returns the number of profile accounts
// Input    conn        Database connection.
// Output   i32         Number of saved profiles.
pub fn num_profile(conn: &Connection) -> i32{
    let maybe_stmt = conn.prepare("SELECT * FROM profile");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[]).unwrap();
    return rows.len() as i32;
}

// Retreives a profile.
// Input    name        Name of profile to retrieve
// Input    conn        Database connection.
// Output   profile     Retrieved profile struct.
pub fn get_profile(name: &str, conn: &Connection) -> Profile{
    let maybe_stmt = conn.prepare("SELECT * FROM profile WHERE name = $1");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let n: String = name.to_string();
    let rows = stmt.query(&[&n]).unwrap();
    let row = rows.get(0);
    Profile {
        name        : row.get(0),
        password    : row.get(1),
        account     : row.get(2),
        active      : row.get(3),
        secret_key  : row.get(4),
        trusted     : row.get(5),
    }
}

// Retrieves the active profile profile
// Input    conn        Database connection.
// Output   Result      Retrieves active profile or error message.
pub fn get_active(conn: &Connection) -> Result<Profile, &str> {
    let maybe_stmt = conn.prepare("SELECT * FROM profile WHERE active = true");
    let stmt = match maybe_stmt{
        Ok(stmt) => stmt,
        Err(err) => panic!("Error preparing statement: {:?}", err)
    };
    let rows = stmt.query(&[]);
    match rows {
        Err(_) => Err("Error retrieving active profile."),
        Ok(r) => {
            if r.len() != 0 {
                let row = r.get(0);
                let p = Profile {
                    name        : row.get(0),
                    password    : row.get(1),
                    account     : row.get(2),
                    active      : row.get(3),
                    secret_key  : row.get(4),
                    trusted     : row.get(5),
                };
                Ok(p)
            } else {
                Err("No active profiles.")
            }
        },
    }
}
// Switches active profile.
// Input    n           Name of profile to activate.
// Input    conn        Database connection.
// Output   boolean     Successfully activated?
pub fn switch_active(n: &str, conn: &Connection) -> bool{
    let possible_active = get_active(conn);
    match possible_active {
        Err(_) => activate(n, conn),
        Ok(mut p) => {
            p.active = false;
            save_profile(&p, conn);
            activate(n, conn)
        },
    }
}

// Activate profile of a given name
// Input    name        Name of profile to activate.
// Input    conn        Database connection.
// Output   boolean     Successfully activated?
pub fn activate(name: &str, conn: &Connection) -> bool{
    println!("\n=>> Activating profile...");
    let exist = profile_exist(name, conn);
    if !exist{
        println!("=>> Profile does not exist.");
        return false;
    }
    //Check if there is a profile activated
    match get_active(conn){
        Err(_) => {
            let mut p = get_profile(name, conn);
            p.active = true;
            save_profile(&p, conn);
            println!("=>> Profile activated.");
            return true;
        },
        Ok(p) => {
            if p.name != name{
                println!("Profile {:?} is currently active. Deactivating and activating {:?}", p.name, name);
                return switch_active(name, conn);
            } else {
                println!("Profile {:?} is already active.", p.name);
                return true;
            }

        },
    }
}

// Deactive active profile
// Input    conn        Database connection.
pub fn deactivate(conn: &Connection){
    let mut p = get_active(conn).unwrap();
    p.active = false;
    save_profile(&p, conn);
}
