// Requesting logic
const request = require('request');
const fs = require( 'fs' )
let stream = fs.createWriteStream( './json/trivia.json' )
request( 'https://opentdb.com/api.php?amount=50', { json: true }, (err, res, body) =>
         {
             
             if( err ) {  console.log( 'sucks' ); }
             else
             { fs.writeFile( './json/trivia.json', JSON.stringify( body ), (err) =>{
                 if (err) throw err;
                 console.log("The file has been saved");
             } ); }
             
         } );
stream.end()

// Path definitions
let errorLog = './log/error.log'
let requestLog = './log/request.log'

// Listening logic 
const express = require( 'express' )
const server = express()

// Requesting Logic
const requestJson = require( 'request-json' )

// The client
var client = requestJson.createClient( 'https://boiling-cove-42309.herokuapp.com/' )

// The full node

// Post registration to client

// Post the passport to the full node 

// Listen to get requests over the standard slug
server.get( '/', ( req, res ) => res.send( 'hello world!' ) )

// Listen on 3000
server.listen( 3000, () => console.log( 'Listening on 3000' ) )
