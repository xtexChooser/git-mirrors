import logger from './logger.js';
import { listObjects, loadSchema, readObject } from './registry.js';

export async function lintAllSchema() {
    for (const schema of await listObjects('SCHEMA')) {
        await lintSchema(schema)
    }
}

export async function lintSchema(schema: string) {
    logger.trace({ schema }, 'Check schema')

    const jtd = await loadSchema(schema)

    for (const key of await listObjects(schema)) {
        logger.trace({ schema, key }, 'Check object')

        const obj = await readObject(schema, key)

        if (!jtd(obj)) {
            logger.error({ schema, key, obj, error: jtd.errors })
        }
    }
}

export async function lintObject(schema: string, key: string) {
    logger.trace({ schema, key }, 'Check object')

    try {
        const jtd = await loadSchema(schema)
        const obj = await readObject(schema, key)
        if (!jtd(obj)) {
            logger.error({ schema, key, obj, error: jtd.errors })
        }
    } catch (e: any) {
        logger.error({ schema, key, e })
    }
}
