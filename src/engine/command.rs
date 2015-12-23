use std::{thread, process};
// use std::io::Write;
use std::sync::Arc;
use data::{account, state, database, log, profile, tenv};
use network::server;
use util::helper;
use std::sync::RwLock;
use std::collections::HashMap;
use std::time::Duration;

//====================================================================
// COMMAND FUNCTIONS
// Contains functions called by the CLI.
//====================================================================

// Drops all database tables
pub fn drop_all(){
        let conn = database::connect_db();
        account::drop_account_table(&conn);
        state::drop_state_table(&conn);
        log::drop_log_table(&conn);
        profile::drop_profile_table(&conn);
        database::close_db(conn);
}

// Execute db command with flags
// Input    flags   (flags for db command)
pub fn database_flags(flags: Vec<String>){
    let flag = &flags[0];
    match &flag[..]{
        "-r" => {
            let conn = database::connect_db();
            let target = &flags[1];
            match &target[..]{
                "all" => {
                    println!("=>> Are you sure you want to drop everything? (y/n)");
                    let yn: bool = helper::read_yn();
                    if yn {
                        drop_all();
                        process::exit(1);
                    }
                },
                "account"   => account::drop_account_table(&conn),
                "profile"   => {
                    profile::deactivate(&conn);
                    profile::drop_profile_table(&conn);
                },
                "log"       => log::drop_log_table(&conn),
                "state"     => state::drop_state_table(&conn),
                _           => println!("=>> Unrecognized flag target for [db -drop]"),
            };
            database::close_db(conn);
        },
        _ => println!("=>> Unrecognized flags for command [db]"),
    }
}

// Main Entry Function
// Connects to network and starts consensus loop
pub fn turbo(){
    println!("\n\n=>> Performing network check...");
    let conn = database::connect_db();
    let p = profile::get_active(&conn).unwrap();

    let trusted: Vec<String> = helper::string_to_vec(&p.trusted);
    let local_ip: String = "192.168.1.1".to_string();
    println!("=>> Starting local server...");

    // Initializing Arcs
    // Local Status. String<Status>
    let local_stat: Arc<RwLock<(String, String)>> = Arc::new(RwLock::new((String::new(), String::new())));
    // Connected Nodes and their current status. HashMap<Address, (State, Nonce)>
    let thread_stat: Arc<RwLock<HashMap<String, tenv::Tenv>>> = Arc::new(RwLock::new(HashMap::new()));
    // Current Logs. HashMap<Hash, Log>
    let curr_logs: Arc<RwLock<HashMap<String, log::Log>>> = Arc::new(RwLock::new(HashMap::new()));

    database::close_db(conn);

    // Cloning to move into server
    let m_stat = local_stat.clone();
    let t_stat = thread_stat.clone();
    let c_logs = curr_logs.clone();
    //Starting Server
    let _ = thread::spawn(move ||
        server::listen(local_ip, m_stat, t_stat, c_logs)
    );
    //Connecting to trusted accounts for active profile.
    println!("\n=>> There are {:?} trusted accounts on this profile.", trusted.len());
    thread::sleep(Duration::from_millis(500)); // Allow server to bind
    // Connecting to peers
    for ip in trusted{
        server::connect(&ip, local_stat.clone(), thread_stat.clone(), curr_logs.clone());
    }
    //TODO: Start consensus loop
    println!("=>> Starting Consensus Protocol");

}
