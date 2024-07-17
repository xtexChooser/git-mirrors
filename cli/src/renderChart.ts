import * as echarts from 'echarts';
import fs from 'fs';
import { DataValues, generateEchartsSpec } from './generateEchartsSpec.js';

const renderChart = async (
	sourceFile: string,
	outputFile: string,
	width: number,
	height: number
): Promise<void> => {
	try {
		console.log( 'source', sourceFile );
		console.log( 'output', outputFile );

		// eslint-disable-next-line security/detect-non-literal-fs-filename
		const jsonData = fs.readFileSync( sourceFile, 'utf8' );
		const sourceData = JSON.parse( jsonData ) as DataValues;

		const eChartsSpec = generateEchartsSpec( sourceData );

		const chart = echarts.init( null, 'vintage', {
			renderer: 'svg',
			ssr: true,
			width: width,
			height: height
		} );

		chart.setOption( eChartsSpec );

		const svg = chart.renderToSVGString();

		// eslint-disable-next-line security/detect-non-literal-fs-filename
		fs.writeFileSync( outputFile, svg );

		chart.dispose();
	} catch ( error ) {
		console.error( 'Error rendering echarts spec to SVG:', error );
	}
};

export { renderChart };
