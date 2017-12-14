// Listening logic 
const express = require( 'express' )
const server = express()

// Listen to get requests over the standard slug
server.get( '/', ( req, rest ) => res.send( 'hello world!' ) )

// Requesting logic
request = require( 'request-json')
var trivia = request.createClient( 'https://opentdb.com/' )
trivia.get( 'https://opentdb.com/api.php?amount=50' ).pipe( fs.createWriteStream( 'trivia.json' ) )

// Listen on port specified
server.listen( 3000, () => console.log( 'Listening on 3000' ) )

const fs = require( 'fs' );
var string = 'ok why is this not working';
var path = './trivia.json'

fs.writeFile( path, string, (err) => {
    if (err) throw err;
    console.log( 'File saved!' );
})
