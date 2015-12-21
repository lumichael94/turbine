use std::io::BufRead;
use util::helper;

// Initialization Function
pub fn init(){
    println!("Welcome to Turbine.");
}

// Command Line Interface.
pub fn command_loop(){
    println!("=>> Starting Command REPL");
    let mut go: bool = true;
    while go {
        go = read_command();
    }
}

// Reads and executes a command
// Output: Boolean (Success or Failure when reading command)
pub fn read_command() -> bool{
    let response: String = helper::read_in();
    let split = response.split(" ");
    let raw_vec = split.collect::<Vec<&str>>();
    let mut flags = helper::vec_to_string(&raw_vec);
    let command: String = flags.remove(0);
    let _ = match &command[..]{
        "quit"|"exit"   => return false,
        _               => println!("=>> Did not recognize command, please try again."),
    };
    return true;
}
