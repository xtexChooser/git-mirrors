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

	test( 'renders a graph with unique id', async () => {
		const output = './result.json';
		const dataPath = `${ __dirname }/../../cli/chart-data.json`;
		const copyPath = `${ __dirname }/../../cli/chart-definition-copy.json`;
		const definition = JSON.parse( fs.readFileSync( `${ __dirname }/../../cli/chart-definition.json`, { encoding: 'utf-8' } ) );
		definition.idPrefix = 'unique-';
		fs.writeFileSync( copyPath, JSON.stringify( definition ) );
		await renderChart(
			dataPath,
			`${ __dirname }/../../cli/chart-definition-copy.json`,
			output
		);
		const result = fs.readFileSync( output ).toString();
		fs.unlinkSync( output );
		fs.unlinkSync( copyPath );
		expect( result ).not.toContain( 'id="zr2-c2"' );
	} );
} );
