import * as echarts from 'echarts';
import * as fs from 'fs';
import { optimize } from 'svgo';
import { ChartData, WikiLineChart } from './chart.js';
import { createLineChart } from './charts/LineChart.js';

const renderChart = async (
	sourceFile: string,
	chartDefFile: string,
	outputFile: string
): Promise<void> => {
	try {
		// eslint-disable-next-line security/detect-non-literal-fs-filename
		const jsonData = fs.readFileSync( sourceFile, 'utf8' );
		const sourceData = JSON.parse( jsonData ) as ChartData;

		// eslint-disable-next-line security/detect-non-literal-fs-filename
		const chartDefinitionJson = fs.readFileSync( chartDefFile, 'utf8' );
		const chartDef = JSON.parse( chartDefinitionJson ) as WikiLineChart;

		const eChartsSpec = createLineChart( chartDef, sourceData );
		const { width, height } = chartDef;

		const chart = echarts.init( null, 'vintage', {
			renderer: 'svg',
			ssr: true,
			width,
			height
		} );

		chart.setOption( eChartsSpec );

		const rawSvg = chart.renderToSVGString();
		// Use SVGO to prefix all IDs in the SVG
		const processedSvg = optimize( rawSvg, {
			plugins: [
				{ name: 'prefixIds', params: { prefix: chartDef.idPrefix } }
			]
		} ).data;

		// - means stdout
		// eslint-disable-next-line security/detect-non-literal-fs-filename
		fs.writeFileSync( outputFile === '-' ? process.stdout.fd : outputFile, processedSvg );

		chart.dispose();
	} catch ( error ) {
		console.error( 'Error rendering echarts spec to SVG:', error );
	}
};

export { renderChart };
