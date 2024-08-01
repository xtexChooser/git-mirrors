'use strict';

const { renderChart } = require( '../../cli/src/renderChart.ts' );
const fs = require( 'fs' );

describe( 'renderChart', () => {
	test( 'renders a graph', async () => {
		const output = './result.json';
		await renderChart(
			`${ __dirname }/../../cli/chart-data.json`,
			`${ __dirname }/../../cli/chart-definition.json`,
			output
		);
		const result = fs.readFileSync( output ).toString();
		fs.unlinkSync( output );
		expect( result ).toMatchSnapshot();
	} );
} );
