// Serde used for serialization
#[macro_use]
extern crate serde_derive;

// Crate inclusion
//
// Blockchain
extern crate off_blockway;
// Time stamping
extern crate chrono;
// Used for serialization
extern crate serde;
extern crate serde_json;

// Use statements
//
// Standard library's functionality for file paths
#[allow(unused_imports)]
use std::path::Path;
// Prelude
use std::io::prelude::*;
// Commands
use std::process::Command;
// Standard library 
use std::*;
// Used for writing to files
#[allow(unused_imports)]
use std::fs::{ OpenOptions, File };
// Passport and miner 
use json::{ Miner, Passport };

// Modules
//
// Styling
mod styling;
// Json
mod json;

/*
 *
 * Main:
 *    - Contains the functionality for executing the mining client 
 *
 */

// Main function 
fn main()
{

    // Registers the miner with the client 
    register_miner();

    // TODO: receive cargo
    
    // Starts the input stream 

    // TODO: have input stream use trivia database and ship package on success 
    
    input_stream();
    
    
}

// Registers the mining node with the full client
pub fn register_miner()
{
    
    // Creates a new passport
    #[allow(unused_must_use)]
    let passport = Passport::new();
    // Creates the file path to the JSON passport
    let path = "./json/passport.json";
    // Serializes the json using the given path
    #[allow(unused_must_use)]
    passport.write_to( &path );
    
}

// Input function
pub fn input_stream()
{

    // Get the input reader
    let input = io::stdin();
    print!( "{}\n", styling::HEADER );
    // Start the node server
    Command::new( "forever" ).args( &[ "start", "./js/server.js" ] ).output().expect( "Could not start the server" );
    // While the input stream has not reached EOF
    for line in input.lock().lines()
    {
        
        // Input form this line
        let input = line.unwrap();
        // If the user requests the help message
        if input.clone() == "-h" || input.clone() == "help"
        {
            
            println!( "{}", styling::HELP );
            
        }
        // If the user requests to quit
        else if input.clone() == "-q" || input.clone() == "quit"
        {
            
            println!( "{}", styling::BYE );
            Command::new( "forever" ).args( &["stop", "js/server.js" ] ).output().expect(
                "Could not stop process" );
            break;
            
        }
        // If the user queries for a trivia question
        else if input.clone() == "-c" || input.clone() == "construct"
        {

            // TODO: query trivia databse 
            
        }
        // Otherwise the user entered an invalid command 
        else
        {

            println!( "{}", styling::INVALID );
            
        }
        
    }   
    
}
