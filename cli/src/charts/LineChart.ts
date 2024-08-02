import { EChartsOption, LegendComponentOption } from 'echarts';
import { ChartData, ChartValues, Field, WikiLineChart } from '../chart.js';

interface SeriesItem {
  name: string;
  type: 'line';
  data: number[];
}

const newSeries = ( field: Field ): SeriesItem => {
	const seriesName = field.title || field.name;

	const series : SeriesItem = {
		name: seriesName,
		type: 'line',
		data: []
	};

	return series;
};

const getTitle = ( chartDefinition: WikiLineChart ): string | undefined => {
	const { yAxis } = chartDefinition;
	return yAxis && yAxis.title;
};

// @todo do we allow legend position as an option in the chart definition?
const getLegendPosition = () : string => 'right';

const getLegend = () : LegendComponentOption => {
	const legendPosition = getLegendPosition();
	const legend : LegendComponentOption = {
		type: 'plain',
		orient: 'horizontal',
		right: legendPosition === 'right' ? 5 : undefined,
		top: legendPosition === 'top' ? 5 : legendPosition === 'right' ? 'center' : undefined,
		left: legendPosition === 'left' ? 5 : undefined,
		bottom: legendPosition === 'bottom' ? 5 : undefined,
		padding: [ 5, 5, 5, 5 ]
	};

	if ( legendPosition === 'right' || legendPosition === 'left' ) {
		legend.orient = 'vertical';
	}

	return legend;
};

export const createLineChart = (
	chartDefinition: WikiLineChart,
	chartData: ChartData
): EChartsOption => {
	const seriesList: SeriesItem[] = [];

	chartData.schema.fields.forEach( ( field, idx ) => {
		if ( idx > 0 ) {
			seriesList.push( newSeries( field ) );
		}
	} );

	const xAxisValues: string[] = [];

	chartData.data.forEach( ( values: ChartValues ) => {
		const [ xAxisValue, ...dataItems ] = values;

		const dateValue = new Date( xAxisValue );
		if ( chartDefinition.xAxis.type === 'date' ) {
			const dateFormatted = dateValue.toISOString().split( 'T' )[ 0 ];
			xAxisValues.push( dateFormatted );
		} else {
			xAxisValues.push( xAxisValue );
		}

		dataItems.forEach( ( dataItem, dataItemIdx ) => {
			seriesList[ dataItemIdx ].data.push( dataItem );
		} );
	} );

	const legendPosition = getLegendPosition();

	const chartSpec: EChartsOption = {
		xAxis: {
			type: 'category',
			axisLabel: {
				rotate: 90
			},
			nameRotate: 0,
			nameLocation: 'middle',
			nameGap: 50,
			data: xAxisValues
		},
		yAxis: {
			type: 'value',
			axisLabel: {
				rotate: 0
			},
			nameRotate: 90,
			nameLocation: 'middle',
			nameGap: 40,
			name: getTitle( chartDefinition )
		},
		grid: {
			containLabel: true,
			right: legendPosition === 'right' ? 100 : 20,
			top: legendPosition === 'top' ? 100 : 20,
			left: legendPosition === 'left' ? 100 : 50,
			bottom: legendPosition === 'bottom' ? 100 : 20
		},
		series: seriesList
	};

	if ( chartDefinition.legend ) {
		chartSpec.legend = getLegend();
	}

	return chartSpec;
};
