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
let nodeLog = './log/node.log'

// Listening logic 
const express = require( 'express' )
const server = express()

// Requesting Logic
const requestJson = require( 'request-json' )
//var fullNode = ""; 
// The client
var client = requestJson.createClient( 'https://boiling-cove-42309.herokuapp.com/' )

request.post( 'https://boiling-cove-42309.herokuapp.com/miner', '132.162.123.150:3000',  function( err, res, body ) {

    if( err ) { console.log( "error!" ) }
    if( res.statusCode == "450" )
    {

        fs.writeFile( './log/node.log', "450",  (err) =>{
            if (err) throw err;
        } )
        
    }
    else {
        
        fs.writeFile( './log/url.log', 'http://' + JSON.parse( res.body ).URL, (err) =>{
            if (err) throw err;
        } )
        //fullNode = 'http://' + JSON.parse( res.body ).URL;

    }
    console.log( res.body );
    
     
} )

// The full node
const fullNode = fs.readFileSync( './log/url.log', 'utf8', function( err, data ) {
    if( err ) throw err;
})

const passport = fs.readFileSync( './json/passport.json', 'utf8', function( err, data ) {
    if( err ) throw err;
})

request.get( fullNode + '/', function ( err, res, body) {
    if ( err ) { console.log(err)}
    console.log(body)
})
console.log(passport)
console.log( fullNode + '/register/')
// Post the passport to the full node 
request.post( fullNode + '/register/', passport, function( err, res, body) {

    if(err) { console.log(err)}
    else if( res.statusCode == "450"){
        console.log("450 status code")
    }
    else{
        console.log("nice")
    }
})

// Listen to get requests over the standard slug
server.get( '/', ( req, res ) => res.send( 'hello world!' ) )

/*
// Listen on 3000
server.listen( 3000, ( err ) => {

    if(err) {console.log(err)}
    else {console.log( 'Listening on 3000' )}

})
*/
