'use strict';

const { renderChart } = require( '../../cli/src/renderChart.ts' );
const fs = require( 'fs' );

describe( 'renderChart', () => {
	test( 'renders a graph', async () => {
		const output = './result.json';
		await renderChart(
			`${ __dirname }/../../cli/data/1993_Canadian_federal_election/data.json`,
			`${ __dirname }/../../cli/data/1993_Canadian_federal_election/chart.json`,
			output
		);
		const result = fs.readFileSync( output ).toString();
		fs.unlinkSync( output );
		expect( result ).toMatchSnapshot();
	} );

	test( 'renders a graph with unique id', async () => {
		const output = './result.json';
		const dataPath = `${ __dirname }/../../cli/data/1993_Canadian_federal_election/data.json`;
		const copyPath = `${ __dirname }/../../cli/data/chart-definition-copy.json`;
		const definition = JSON.parse(
			fs.readFileSync(
				`${ __dirname }/../../cli/data/1993_Canadian_federal_election/chart.json`,
				{ encoding: 'utf-8' }
			)
		);
		definition.idPrefix = 'unique-';
		fs.writeFileSync( copyPath, JSON.stringify( definition ) );
		await renderChart(
			dataPath,
			`${ __dirname }/../../cli/data/chart-definition-copy.json`,
			output
		);
		const result = fs.readFileSync( output ).toString();
		fs.unlinkSync( output );
		fs.unlinkSync( copyPath );
		expect( result ).not.toContain( 'id="zr2-c2"' );
	} );

	test( 'render a line chart with no showSymbols', async () => {
		const output = './result.json';
		const chartDataPath = `${ __dirname }/../../cli/data/1993_Canadian_federal_election/data.json`;
		const chartDefinitionPath = `${ __dirname }/../../cli/data/1993_Canadian_federal_election/chart.json`;

		const definition = JSON.parse(
			fs.readFileSync( chartDefinitionPath, { encoding: 'utf-8' } )
		);

		delete definition.showSymbols;

		const tmpChartDefinitionPath = `${ __dirname }/../../cli/data/chart-definition-no-symbols.json`;
		fs.writeFileSync( tmpChartDefinitionPath, JSON.stringify( definition ) );

		await renderChart( chartDataPath, tmpChartDefinitionPath, output );

		const result = fs.readFileSync( output ).toString();

		fs.unlinkSync( output );
		fs.unlinkSync( tmpChartDefinitionPath );

		expect( result ).toMatchSnapshot();
	} );

	test( 'render a line chart with invalid showSymbols', async () => {
		const output = './result.json';
		const chartDataPath = `${ __dirname }/../../cli/data/1993_Canadian_federal_election/data.json`;
		const chartDefinitionPath = `${ __dirname }/../../cli/data/1993_Canadian_federal_election/chart.json`;

		const definition = JSON.parse(
			fs.readFileSync( chartDefinitionPath, { encoding: 'utf-8' } )
		);

		definition.showSymbols = 'kittens';

		const tmpChartDefinitionPath = `${ __dirname }/../../cli/data/chart-definition-no-symbols.json`;
		fs.writeFileSync( tmpChartDefinitionPath, JSON.stringify( definition ) );

		await renderChart( chartDataPath, tmpChartDefinitionPath, output );

		const result = fs.readFileSync( output ).toString();

		fs.unlinkSync( output );
		fs.unlinkSync( tmpChartDefinitionPath );

		expect( result ).toMatchSnapshot();
	} );
} );
