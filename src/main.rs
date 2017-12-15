// Serde macro use used for deriving serialization
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
// Used for random numbers
extern crate rand; 

// Use statements
//
// Standard library's functionality for file paths
#[allow(unused_imports)]
use std::path::Path;
// Prelude
#[allow(unused_imports)]
use std::io::prelude::*;
// Commands
#[allow(unused_imports)]
use std::process::Command;
// Standard library
#[allow(unused_imports)]
use std::*;
// Used for writing to files
#[allow(unused_imports)]
use std::fs::{ OpenOptions, File };
// Passport and miner
#[allow(unused_imports)]
use json::{ Miner, Passport, Archive, Question };

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

    /*  
    let mut miner = Miner::new();
    miner.construct_merkle( &"./json/transactions.json" );
    miner.write_to( &"dummy.json" );
     */     
    
}

// Registers the mining node with the full client
pub fn register_miner()
{

    // Checks to see if URL was received from the client
    let log = Path::new( "./log/node.log" );
    // If the url error log exists 
    if log.exists()
    {

        let mut file = File::open( "./log/node.log" ).unwrap();
        let mut buffer = String::new();
        file.read_to_string( &mut buffer );
        if buffer == "450"
        {

            println!( "Unable to register with full node!" );
            
        }
        
    }
    // Creates a new passport
    let passport = Passport::new();
    // Creates the file path to the JSON passport
    let path = "./json/passport.json";
    // Serializes the json using the given path
    #[allow(unused_variables)]
    let temp = passport.write_to( &path );
    
}

// Input function
pub fn input_stream()
{

    // Get the input reader
    let stream = io::stdin();
    // Header message 
    print!( "{}\n", styling::HEADER );
    // Start the node server
    Command::new( "forever" ).args( &[ "start", "./js/server.js" ] ).output().expect( "Could not start the server" );
    // Trivia archive
    let archive = Archive::read_and_construct( "./json/trivia.json" ).unwrap();
    // While the input stream has not reached EOF
    loop
    {

        // Current line input 
        let mut input = String::new();
        #[allow(unused_variables)]
        let temp = stream.read_line( &mut input );
        let length = input.clone().len();
        // Removes new line character 
        input.truncate( length - 1 );
        // If the user requests the help message
        if input.clone() == "-h" || input.clone() == "help"
        {
            
            // Print the help message 
            println!( "{}", styling::HELP );
            
        }
        // If the user requests to quit
        else if input.clone() == "-q" || input.clone() == "quit"
        {
            
            // Print the quit message 
            println!( "{}", styling::BYE );
            // Stop the server 
            Command::new( "forever" ).args( &[ "stop", "js/server.js" ] ).output().expect( "Could not stop process" );
            // Break from the input loop 
            break;
            
        }
        // If the user queries for a trivia question
        else if input.clone() == "-c" || input.clone() == "construct"
        {

            // Run trivia protocol
            trivia_protocol( archive.clone() );
            
        }
        // Otherwise the user entered an invalid command 
        else 
        {
            
            // Print the invalid command message 
            println!( "{}", styling::INVALID );
            
        }
        
    }   
    
}

pub fn trivia_protocol( archive: Archive )
{

    // IO stream 
    let stream = io::stdin();
    let archive = archive; 
    loop
    {

        // Pulls a random question 
        let question = archive.clone().random();
        // Options vector
        let mut options = question.incorrect();
        // Pushes correct option
        options.push( question.correct() );
        // Buffer
        println!( "----------------------------------------------------------------" );
        // Prints the category
        println!( "\nCategory: {}\n", question.category() );
        // Buffer
        println!( "----------------------------------------------------------------" );
        // Prints the type
        println!( "\nType: {}\n", question.kind() );
        // Buffer
        println!( "----------------------------------------------------------------" );
        // Prints the difficulty
        println!( "\nDifficulty: {}\n", question.difficulty() );
        // Buffer
        println!( "----------------------------------------------------------------" );
        // Prints the question
        println!( "\nQuestion: {}\n", question.question() );
        // Amount of options in the current question
        let cap = options.len();
        // Prints the options
        for index in 0 .. cap
        {
            
            // Buffer
            println!( "----------------------------------------------------------------" );
            // Option
            println!( "\nOption {}: {}\n", index + 1, options.get( index ).unwrap() );
            
        }
        // Buffer
        println!( "----------------------------------------------------------------\n" );
        let mut input = String::new();
        #[allow(unused_variables)]
        let temp = stream.read_line( &mut input );
        let length = input.clone().len();
        // Removes new line character 
        input.truncate( length - 1 );
        // If the user's input is the same as the correct answer 
        if input.clone() == question.correct().to_string()
        {
            
            // Print the correct answer message 
            println!( "{}", styling::CORRECT );
            // Continue to the next iteration of the loop 
            return;
            
        }
        // Otherwise, it was an incorrect answer 
        else
        {
            
            // Print the incorrect answer message 
            println!( "{}", styling::INCORRECT );
            continue;
            
        }
        
    }

}
