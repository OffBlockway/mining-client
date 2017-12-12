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

// The failed registration message
#[allow(dead_code)]
pub const FAILED: &str = "\nYour registration with a full node failed! That means that there are no full nodes on the network currently, come back another time to mine Off Blockway!\n";
    
// The help message
pub const HELP: &str = "\nThis is the mining client for Off Blockway, we know that mining can be confusing and tiresome, so we're here to make it as fun as possible!

How does mining Off Blockway work?

We take care of most of the work for you after starting up the CLI. By now, you should have received a message confirming your registration with one of the full nodes. If you instead were given a failed registration message then quit the client and come back another time! Otherwise, behind the scenes, you are receiving a log of transactions from your full node that need verification. We handle all the communication between systems, all you need to do is tell us when you're ready to start sending transactions. How do you do this? For Off Blockway, verified transactons will be sent to the full node after the miner answers one of our trivia questions correctly. Yes, you read that correctly -- trivia! Running the construct command described below will start to compile the data on which transactions were verified, after you answer a trivia question correctly, the data will be sent to the full node! 

Below are the commands at your disposal to mine for Off Blockway:

\t quit: 'quit' or '-q' will quit the Party Chain CLI
\t help: 'help or '-h' will display the help message 
\t construct: 'construct' or '-c' will construct a block out of the unverified transactions and prompt you to answer trivia questions. 

We appreciate all your dedication to Off Blockway!\n";

// The closing message
pub const BYE: &str = "\nBye! Thank you for mining Off Blockway.\n";
