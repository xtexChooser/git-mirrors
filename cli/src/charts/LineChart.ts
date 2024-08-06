import { EChartsOption, LegendComponentOption } from 'echarts';
import { ChartData, ChartValues, Field, WikiLineChart } from '../chart.js';

interface SeriesItem {
  name: string;
  type: 'line';
  data: number[];
  showSymbol: boolean;
	itemStyle?: {
		color?: string;
	};
	lineStyle?: {
		color?: string;
	}
}

const hexColorRegex = /^#([a-fA-F0-9]{3}|[a-fA-F0-9]{6}|[a-fA-F0-9]{8})$/;

const isValidColor = ( color: string ): boolean => hexColorRegex.test( color );

const newSeries = ( field: Field, showSymbol: boolean, color?: string ): SeriesItem => {
	const seriesName = field.title || field.name;

	const series: SeriesItem = {
		name: seriesName,
		type: 'line',
		data: [],
		showSymbol
	};

	if ( color && isValidColor( color ) ) {
		series.itemStyle = series.itemStyle || {};
		series.itemStyle.color = color;

		series.lineStyle = series.lineStyle || {};
		series.lineStyle.color = color;
	}

	return series;
};

const getTitle = ( chartDefinition: WikiLineChart ): string | undefined => {
	const { yAxis } = chartDefinition;
	return yAxis && yAxis.title;
};

// @todo do we allow legend position as an option in the chart definition?
const getLegendPosition = (): string => 'right';

const getLegend = (): LegendComponentOption => {
	const legendPosition = getLegendPosition();
	const legend: LegendComponentOption = {
		type: 'plain',
		orient: 'horizontal',
		right: legendPosition === 'right' ? 5 : undefined,
		top: legendPosition === 'top' ? 5 :
			legendPosition === 'right' ? 'center' : undefined,
		left: legendPosition === 'left' ? 5 : undefined,
		bottom: legendPosition === 'bottom' ? 5 : undefined,
		padding: [ 5, 5, 5, 5 ]
	};

	if ( legendPosition === 'right' || legendPosition === 'left' ) {
		legend.orient = 'vertical';
	}

	return legend;
};

const validateChartDefinition = ( chartDefinition: WikiLineChart ): WikiLineChart => {
	if ( typeof chartDefinition.showSymbols !== 'boolean' ) {
		chartDefinition.showSymbols = false;
	}

	return chartDefinition;
};

export const createLineChart = (
	chartDefinitionInput: WikiLineChart,
	chartData: ChartData
): EChartsOption => {
	const chartDefinition = validateChartDefinition( chartDefinitionInput );

	const seriesList: SeriesItem[] = [];

	const { xAxis, legend, colors = [], showSymbols = false } = chartDefinition;

	chartData.schema.fields.forEach( ( field, idx ) => {
		// skip header row
		if ( idx > 0 ) {
			// handle if colors are not provided in the chart definition
			// or there are fewer colors than the number of series
			const color = colors.length && colors[ idx - 1 ] || undefined;
			seriesList.push( newSeries( field, showSymbols, color ) );
		}
	} );

	const xAxisValues: string[] = [];

	chartData.data.forEach( ( values: ChartValues ) => {
		const [ xAxisValue, ...dataItems ] = values;

		const dateValue = new Date( xAxisValue );
		if ( xAxis.type === 'date' ) {
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

	const xAxisLabelRotate = xAxis.angle !== null ? xAxis.angle : 0;

	const chartSpec: EChartsOption = {
		animation: false,
		xAxis: {
			type: 'category',
			axisLabel: {
				rotate: xAxisLabelRotate
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

	if ( legend ) {
		chartSpec.legend = getLegend();
	}

	return chartSpec;
};
