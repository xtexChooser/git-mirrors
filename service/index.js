const express = require( 'express' );
const bodyParser = require( 'body-parser' );

const chartRoute = require( './routes/chart' );

const app = express();
const port = 3000;

app.use( bodyParser.json() );

app.use( '/v1', chartRoute );

app.listen( port, () => {
	console.log( `chart rendering service running on port ${ port }` );
} );
