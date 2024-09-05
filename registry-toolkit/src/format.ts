import logger from './logger.js'
import {
    deserializeObject,
    listObjects,
    readObjectContent,
    serializeObject,
    writeObject,
} from './registry.js'

export async function formatAllSchema() {
    for (const schema of await listObjects('SCHEMA')) {
        await formatSchema(schema)
    }
}

export async function formatSchema(schema: string) {
    logger.trace({ schema }, 'Format schema')

    for (const key of await listObjects(schema)) {
        await formatObject(schema, key)
    }
}

export async function formatObject(schema: string, key: string) {
    logger.trace({ schema, key }, 'Format object')

    try {
        const content = await readObjectContent(schema, key)
        const formatted = serializeObject(deserializeObject(content))
        if (formatted != content) {
            logger.info({ schema, key }, 'Formatted')
            writeObject(schema, key, deserializeObject(content))
        }
    } catch (e) {
        logger.error({ schema, key, e })
    }
}
