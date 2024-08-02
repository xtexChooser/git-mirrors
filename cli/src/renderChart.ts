import * as echarts from 'echarts';
import * as fs from 'fs';
// eslint-disable-next-line n/no-missing-import
import { ChartData, WikiLineChart } from './chart';
// eslint-disable-next-line n/no-missing-import
import { createLineChart } from './charts/LineChart';

const renderChart = async (
	sourceFile: string,
	chartDefFile: string,
	outputFile: string,
	widthOption?: string,
	heightOption?: string
): Promise<void> => {
	try {
		// eslint-disable-next-line security/detect-non-literal-fs-filename
		const jsonData = fs.readFileSync( sourceFile, 'utf8' );
		const sourceData = JSON.parse( jsonData ) as ChartData;

		// eslint-disable-next-line security/detect-non-literal-fs-filename
		const chartDefinitionJson = fs.readFileSync( chartDefFile, 'utf8' );
		const chartDef = JSON.parse( chartDefinitionJson ) as WikiLineChart;

		const eChartsSpec = createLineChart( chartDef, sourceData );
		const width = widthOption ? parseInt( widthOption ) : chartDef.width;
		const height = heightOption ? parseInt( heightOption ) : chartDef.height;

		const chart = echarts.init( null, 'vintage', {
			renderer: 'svg',
			ssr: true,
			width,
			height
		} );

		chart.setOption( eChartsSpec );

		const svg = chart.renderToSVGString();

		// - means stdout
		// eslint-disable-next-line security/detect-non-literal-fs-filename
		fs.writeFileSync( outputFile === '-' ? process.stdout.fd : outputFile, svg );

		chart.dispose();
	} catch ( error ) {
		console.error( 'Error rendering echarts spec to SVG:', error );
	}
};

export { renderChart };
