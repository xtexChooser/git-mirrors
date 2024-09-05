import pino from 'pino'
import pretty from 'pino-pretty'

const stream = pretty({
    ignore: 'pid,time,hostname',
})

const logger = pino(stream)

export default logger
