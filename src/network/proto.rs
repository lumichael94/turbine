extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate chrono;

use std::net::TcpStream;
// use rusqlite::Connection;
use std::io::{Read, Write};
use std::sync::{Arc, RwLock};

//====================================================================
// GENERAL PROTOCOL FUNCTIONS
//====================================================================


//====================================================================
// PROPOSING FUNCTIONS
// Contains functions called during phase: "proposing"
//====================================================================

// Sending possible state hash
// Input    stream          TcpStream with connected node.
// Input    s_hash          State hash before requested state.
pub fn send_poss_state_hash(stream: &mut TcpStream, s_hash: String){
    let raw_hash = s_hash.as_bytes();
    let size = raw_hash.len();
    let _ = stream.write(&[13, size as u8]);
    let _ = stream.write(raw_hash);
}

// Requesting possible state hash
// Input    stream          TcpStream with connected node.
// Output   String          Connected nodes proposal state hash
pub fn request_poss_shash(stream: &mut TcpStream)-> String{
    // Requesting Possible State Hash
    let _ = stream.write(&[13, 0]);
    let mut incoming = [0;2];
    let _ = stream.read(&mut incoming).unwrap();
    String::from_utf8(read_stream(stream, incoming[1])).unwrap()
}

// Reads data from TcpStream
// Input    stream      TcpStream with connected node.
// Input    length      Length of message received.
// Output   Vec<u8>     Message received.
pub fn read_stream(stream: &mut TcpStream, length: u8) -> Vec<u8>{
	let mut data_buf = vec![0; length as usize];
	let _ = stream.read(&mut data_buf[..]);
	return data_buf;
}

// Retrieve status from main thread.
// Input    local_stat  Status of the main thread
// Output   Tuple       Current status of the main thread and current state.
pub fn get_local_stat(local_stat: Arc<RwLock<(String, String)>>)->(String, String){
    let marc = local_stat.clone();
    let reader = marc.read().unwrap();
    let status: String = reader.0.clone();
    let state: String = reader.1.clone();
    drop(reader);
    return (status, state);
}
