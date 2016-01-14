// use std::net::{TcpStream, TcpListener};
// use std::thread;
// use std::time::Duration;
// use std::io::{Read, Write};
// use data::{tenv, log};
// use std::sync::{Arc, RwLock};
// use std::collections::HashMap;

// Starts the listening TCP server
// Input    local_stat      Status of the main thread
// Input    tenv_stat       Status of the connected threads
// Input    curr_logs       Collection of the current uncommitted logs
// pub fn listen(address: String, local_stat: Arc<RwLock<(String, String)>>,
// tenv_stat: Arc<RwLock<HashMap<String, tenv::Tenv>>>, curr_logs: Arc<RwLock<HashMap<String, log::Log>>>){
// 	let add: &str  = &address;
// 	let listener = TcpListener::bind(add).unwrap();
// 	println!("\n=>> Listening started on {}", address);
//     for stream in listener.incoming() {
// 		//Cloning to move into threads
// 		let m_stat = local_stat.clone();
// 		let n_stat = tenv_stat.clone();
// 		let c_logs = curr_logs.clone();
//     	match stream {
//     		Err(e) => { println!("=>> Error on listening: {}", e) }
//     		Ok(stream) => {
//     			thread::spawn(move || {
// 					let _ = stream.set_read_timeout(Some(Duration::new(5,0)));
//     				handle(stream, m_stat, n_stat, c_logs);
//     			});
//     		}
//     	}
// 	}
// }

//Connect to IP address.
// Input    local_stat      Status of the main thread
// Input    tenv_stat       Status of the connected threads
// Input    curr_logs       Collection of the current uncommitted logs
// pub fn connect(address: &str, local_stat: Arc<RwLock<(String, String)>>,
// tenv_stat: Arc<RwLock<HashMap<String, tenv::Tenv>>>, curr_logs: Arc<RwLock<HashMap<String, log::Log>>>){
// 	let stream_attempt = TcpStream::connect(address);
// 	match stream_attempt {
// 		Ok(stream) => {
// 			thread::spawn(move||{
// 				handle(stream, local_stat, tenv_stat, curr_logs);
// 			});
// 		},
// 		Err(_) => {
// 			println!("=>> Error connecting to peer: {:?}", address);
// 		}
// 	}
// }

// Main handler for node connections
// Input    stream          TcpStream with connected node.
// Input    local_stat      Status of the main thread
// Input    tenv_stat       Status of the connected threads
// Input    curr_logs       Collection of the current uncommitted logs
// fn handle(mut stream: TcpStream, local_stat: Arc<RwLock<(String, String)>>,
// tenv_stat: Arc<RwLock<HashMap<String, tenv::Tenv>>>, curr_logs: Arc<RwLock<HashMap<String, log::Log>>>) {
// 	thread::sleep(Duration::from_millis(100));
	// println!("Connected. Passed to handler");
    // Main Statuses
    // let listening: String  = "LISTENING".to_string();
    // let proposing: String  = "PROPOSING".to_string();
    // let committing: String = "COMMITTING".to_string();
    // Handshake
    // let hs_mstat = local_stat.clone();
    // let hs_nstat = tenv_stat.clone();
    // Main handler loop
	// loop {
        // Cloning arcs
        // let main_arc = local_stat.clone();
        // let logs_arc = curr_logs.clone();
        // let marc = main_arc.clone();
    // }
	// Finish and exit
// 	println!("=>> Finished reading from stream.");
// 	drop(stream);
// }

// Pings connected node.
// Input    stream          TcpStream with connected node.
// pub fn ping(stream: &mut TcpStream)-> bool{
// 	let mut inc = [0;2];
// 	let _ = stream.write(&[0, 0]);
// 	let b: bool = match stream.read(&mut inc){
// 		Err(_) => false,
// 		Ok(_) => {
// 			if inc[0] == 1 {
// 				return true;
// 			} else {
// 				return false;
// 			}
// 		},
// 	};
// 	return b;
// }
