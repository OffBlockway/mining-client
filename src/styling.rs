/*
 *
 * Styling:
 *     - This file contains the functionality for styling the client 
 *
 */

// Use statements
//
// Standard string
#[allow(unused_imports)]
use std::string::String;

// The header message
pub const HEADER: &str = "\nWelcome to the Off Blockway Party Chain Mining Client! 

Type 'help' or '-h' for a help message on how to use the mining client or 'quit' or '-q' to quit the mining client.\n";

// The registration message
#[allow(dead_code)]
pub const REGISTRATION: &str = "\nCongratulations! You have successfully registered with one of the full nodes on the network!\n";
    
// The help message
pub const HELP: &str = "\nThis is the mining client for Off Blockway, we know that mining can be confusing and tiresome, so we're here to make it as fun as possible!
Below are the commands at your disposal to mine for Off Blockway:

\t quit: 'quit' or '-q' will quit the Party Chain CLI
\t help: 'help or '-h' will display the help message 
\t construct: 'construct' or '-c' will construct a block out of the unverified transactions

We appreciate all your dedication to Off Blockway!\n";

// The closing message
pub const BYE: &str = "\nBye! Thank you for mining Off Blockway.\n";
