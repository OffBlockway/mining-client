// The client
//var client = requestJson.createClient( 'https://boiling-cove-42309.herokuapp.com/' )

//client.post( 'client')

// Requesting logic
const request = require('request');
const fs = require( 'fs' )
let stream = fs.createWriteStream( './json/trivia.json' )
request( 'https://opentdb.com/api.php?amount=50', { json: true }, (err, res, body) =>
         {
             
             if( err ) {  console.log( 'sucks' ); }
             else
             { stream.write( JSON.stringify( body ) ) }
             
         } );

// Listening logic 
const express = require( 'express' )
const server = express()

// Listen to get requests over the standard slug
server.get( '/', ( req, res ) => res.send( 'hello world!' ) )

// Listen on 3000 
//server.listen( 3000, () => console.log( 'Listening on 3000' ) )
