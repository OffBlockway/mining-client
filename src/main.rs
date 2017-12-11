// Crate inclusion
//
// Blockchain
extern crate off_blockway;

// Use statements
//
// Standard library's functionality for file paths
use std::path::Path;
// Prelude
use std::io::prelude::*;
// Commands
use std::process::Command;
// Standard library 
use std::*;

// Modules
//
// Styling
mod styling;

/*
 *
 * Main:
 *    - Contains the functionality for executing the mining client 
 *
 */

// Main function 
fn main()
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
            Command::new( "forever" ).args( &["stop", "js/server.js" ] ).output().expect( "Could not stop process" );
            break;
            
        }
            
    }
    
}
