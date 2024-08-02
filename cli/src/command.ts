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
	.action(
		async (
			sourceFile: string,
			chartDefFile: string,
			outputFile: string
		) => {
			await renderChart(
				sourceFile,
				chartDefFile,
				outputFile
			);
		}
	);

export { program };
