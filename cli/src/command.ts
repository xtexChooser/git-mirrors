import { Command } from 'commander';
import { renderChart } from './renderChart.js';

const program = new Command();

program.name( 'chart-render' ).description( 'Render charts with Apache Echarts' );

program
	.command( 'line' )
	.description( 'Render a line chart' )
	.argument( '<source>', 'Source file path' )
	.argument( '<output>', 'Output file path' )
	.option( '--width <width>', 'Width of the chart', '800' )
	.option( '--height <height>', 'Height of the chart', '600' )
	.action(
		async (
			sourceFile: string,
			outputFile: string,
			options: { width: string; height: string }
		) => {
			await renderChart(
				sourceFile,
				outputFile,
				parseInt( options.width, 10 ),
				parseInt( options.height, 10 )
			);
		}
	);

export { program };
