const generateEchartsSpec = ( dataValues ) => {
	const seriesData = [];

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

	const eChartsSpec = {
		xAxis: {
			type: 'category',
			data: seriesData.map( ( d ) => d[ 0 ] )
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

module.exports = {
	generateEchartsSpec
};
