import { EChartsOption } from 'echarts';

export interface DataValues {
  values: Record<string, string|number>[];
}

export const generateEchartsSpec = ( dataValues: DataValues ) => {
	const seriesData: ( string | number )[][] = [];

	dataValues.values.forEach( ( d ) => {
		const v = Object.values( d );

		const date = new Date( v[ 0 ] );

		if ( !isNaN( date.getTime() ) ) {
			const dateFormatted = date.toISOString().split( 'T' )[ 0 ];
			seriesData.push( [ dateFormatted, v[ 1 ] ] );
		} else {
			seriesData.push( v );
		}
	} );

	const eChartsSpec: EChartsOption = {
		xAxis: {
			type: 'category',
			data: seriesData.map( ( d ) => d[ 0 ] as string )
		},
		yAxis: {
			type: 'value'
		},
		series: [
			{
				type: 'line',
				data: seriesData.map( ( d ) => d[ 1 ] )
			}
		]
	};

	return eChartsSpec;
};
