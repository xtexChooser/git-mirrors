import { Command } from 'commander';
import { renderChart } from './renderChart.js';

const program = new Command();

program.name( 'chart-render' ).description( 'Render charts with Apache Echarts' );

program
	.command( 'line' )
	.description( 'Render a line chart' )
	.argument( '<source>', 'Source file path' )
	.argument( '<definition>', 'Chart definition json' )
	.argument( '<output>', 'Output file path' )
	.option( '-w, --width <number>', 'Chart width' )
	.option( '-h, --height <number>', 'Chart height' )
	.action(
		async (
			sourceFile: string,
			chartDefFile: string,
			outputFile: string,
			options: { width?: string; height?: string }
		) => {
			await renderChart(
				sourceFile,
				chartDefFile,
				outputFile,
				options.width,
				options.height
			);
		}
	);

export { program };
