const express = require( 'express' )
const server = express()

// Listen to get requests over the standard sluf
server.get( '/', ( req, rest ) => res.send( 'hello world!' ) )

// Listen on port specified
server.listen( 3000, () => console.log( 'Listening on 3000' ) )
