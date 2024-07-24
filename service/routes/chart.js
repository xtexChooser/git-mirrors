const express = require( 'express' );
const echarts = require( 'echarts' );

const spec = require( '../services/spec' );

const router = express.Router();

router.post( '/render', async ( req, res ) => {
	try {
		const chartSpec = req.body;
		const { data, width, height } = chartSpec;
		console.log( data );

		const eChartsSpec = spec.generateEchartsSpec( data );

		const chart = echarts.init( null, 'vintage', {
			renderer: 'svg',
			ssr: true,
			width: width,
			height: height
		} );

		chart.setOption( eChartsSpec );

		const svg = chart.renderToSVGString();

		chart.dispose();

		res.setHeader( 'Content-Type', 'image/svg+xml' );
		res.send( svg );
	} catch ( error ) {
		console.error( 'Error rendering echarts spec to SVG:', error );
		res.status( 500 ).json( {
			error:
        error.message || 'An error occurred while rendering the echarts spec.'
		} );
	}
} );

module.exports = router;
