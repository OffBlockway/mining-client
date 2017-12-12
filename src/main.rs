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
// Used for timestamps
use self::chrono::Utc;
// Use off blockway functionality
use self::off_blockway::*;
// Used for serialization
use self::serde_json::Error;
// Used for writing to files
#[allow(unused_imports)]
use std::fs::{ OpenOptions, File };
// Strings
use std::string::String;

// Modules
//
// Styling
mod styling;

/*
 *
 * Passport:
 *     - Struct and impl for passport and associated functions  
 *
 */

// Passport struct
//
// Derive statement used for JSON serialization 
#[derive( Serialize, Deserialize )]
pub struct Passport
{

    // Unique ID of the mining node
    uid: String,
    // Timestamp on running the client
    timestamp: String,
    // Url of the current server
    url: String
    
}

// Impl for the passport
impl Passport
{

    // Constructor for a new passport
    pub fn new() -> Self
    {

        // The current ip address 
        let ip = Command::new( "ipconfig" ).args( &[ "getifaddr", "en0" ] ).output()
            .expect( "Could not find IP address" );
        // The Command as a string
        let mut ip_string = String::from_utf8( ip.stdout ).unwrap();
        // The string is returned with a new line character so we remove this
        let length = ip_string.len();
        // Truncate by the length - 2 ( "\n" )
        ip_string.truncate( length - 2 );
        // The current time
        let current_time = Utc::now().to_string();
        // The url of the current user
        //
        // First make a copy of the ip string so there is no move error
        let mut ip_copy = ip_string.clone();
        // Pushes the string ":3000" to complete the url 
        ip_copy.push_str( ":3000" );
        let url_string = ip_copy.clone();
        // The unique id is generated by hashing the user's ip address
        let unique_hash = self::hash_util::create_leaf_hash( &ip_copy.clone() );
        // Creates a new passport and sets the fields based on the calculated value 
        let pass = Passport
        {

            uid: unique_hash, 
            timestamp: current_time,
            url: url_string

        };
        pass
        
    }

    // Serializes the passport as JSON
    pub fn write_to( &self, file_name: &str ) -> Result< (), Error >
    {

        // Open the filepath with write specification
        #[allow(unused_must_use)]
        let mut file = OpenOptions::new(  ).write( true ).create( true ).open( file_name ).unwrap();
        // Serializes the information
        let data = serde_json::to_string( &self )?;
        // Write the JSON to the filepath 
        file.write_all( data.as_ref() );
        // Return result 
        Ok( () )
        
    }
    
}

/*
 *
 * Miner:
 *    - Contains the functionality and associated structs and impls for miners  
 *
 */

// Miner struct
//
// Derive statement used for JSON serialization
#[derive( Serialize, Deserialize )]
pub struct Miner
{

    // The unique ID of the miner, which will be the same as in passport
    uid: String,
    // The cargo that the miner will ship, which is a Merkle Tree of Transactions 
    transactions: merkle::Merkle,
    // The block containing the transactions
    package: block::Block
    
}

// Miner impl
impl Miner
{

    // New constructor 
    pub fn new() -> Self
    {

        // The current ip address
        let ip = Command::new( "ipconfig" ).args( &[ "getifaddr", "en0" ] ).output()
            .expect( "Could not find IP address" );
        // The Command as a string
        let mut ip_string = String::from_utf8( ip.stdout ).unwrap();
        // The string is returned with a new line character so we remove this
        let length = ip_string.len();
        // Truncate by the length - 2 ( "\n" )
        ip_string.truncate( length - 2 );
        // Adds the :3000 string to the ip representing the url
        ip_string.push_str( ":3000" );
        // The hashed url, representing the unique id
        let unique_hash = hash_util::create_leaf_hash( &ip_string.clone() );
        // The current time 
        let current_time = Utc::now().to_string();
        // An empty merkle tree
        let merkle = merkle::Merkle::empty();
        // Creates a new block
        let mut block = block::Block::new( 0, hash_util::empty_hash() );
        // Sets the timestamp
        block.timestamp = current_time;
        // Create and return the miner
        let miner = Miner
        {

            // Sets the fields of the miner to the values calculated above
            uid: unique_hash,
            transactions: merkle,
            package: block
            
        };
        miner
        
    }

    // Serializes the miner as JSON
    pub fn write_to( &self, file_name: &str ) -> Result< (), Error >
    {
        
        // Open the filepath with write specification
        #[allow(unused_must_use)]
        let mut file = OpenOptions::new(  ).write( true ).create( true ).open( file_name ).unwrap();
        // Serializes the information
        let data = serde_json::to_string( &self )?;
        // Write the JSON to the filepath
        file.write_all( data.as_ref() );
        // Return result
        Ok( () )
            
    }

    // Creates the Merkle Tree given the transaction log
    pub fn construct_merkle( &self, file_name: &str )
    {

        // Gets the node from the transaction log 
        let nodes: Vec<transaction::Transaction> = off_blockway::merkle::Merkle::read_and_construct( file_name).unwrap().nodes();
        // Inserts the nodes into the transaction tree
        for i in 0 .. nodes.len()
        {
            
            self.transactions.insert( *nodes.get(i).unwrap() );

        }
        // Resets the values of the block after Merkle Construction 
        self.reset_block_merkle();

    } 

    // Resets the block's Merkle after the nodes are added 
    pub fn reset_block_merkle( &self )
    {
    
        self.package.merkle_root = *self.transactions.root_hash();

    } 
    
} 

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
