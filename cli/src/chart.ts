export interface WikiLineChart {
    version: number,
    type: 'line',
    width: number,
    height: number,
    xAxis: {
        title: string,
        angle: number,
        type: 'date'
    },
    yAxis: {
        title: string,
    },
    legend: string,
    interpolate: 'basis',
    showSymbols: boolean,
    colors: string[],
    source: string
}

export interface Field {
    name: string,
    type: 'string',
    title: string
}

export type ChartValues = [string, ...number[]];

export interface ChartData {
    license: string,
    description: string,
    schema: {
        fields: Field[]
    },
    data: ChartValues[]
}
