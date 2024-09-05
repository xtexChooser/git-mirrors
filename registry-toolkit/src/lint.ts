import logger from './logger.js'
import { listObjects, loadSchema, readObject } from './registry.js'
import { Schema } from './registry.js'
import jsonpath from 'jsonpath'

export async function lintAll() {
    await lintAllSchema()
}

export async function lintAllSchema() {
    for (const schema of await listObjects('SCHEMA')) {
        await lintSchema(schema)
    }
}

export async function lintSchema(schema: string) {
    logger.trace({ schema }, 'Check schema')

    for (const key of await listObjects(schema)) {
        await lintObject(schema, key)
    }
}

export async function lintObject(schema: string, key: string) {
    logger.trace({ schema, key }, 'Check object')

    try {
        const obj = await readObject(schema, key)

        const schemaObj = (await readObject('SCHEMA', schema)) as Schema
        const jtd = await loadSchema(schemaObj)
        if (!jtd(obj)) {
            logger.error({ schema, key, obj, error: jtd.errors })
        }

        const schemaSelfRefKey = schema.replace('-', '_').toLowerCase()
        if (obj[schemaSelfRefKey] != key) {
            logger.error(
                { schema, key, value: obj[schemaSelfRefKey]?.toString() },
                'Object does not have a self-reference'
            )
        }

        for (const reference of schemaObj.ref ?? []) {
            const refs = jsonpath.query(obj, reference.path)
            if (refs.length == 0) continue
            for (const ref of refs) {
                if (!(typeof ref == 'number' || typeof ref == 'string'))
                    logger.error(
                        { schema, key, ref, reference, type: typeof ref },
                        'Unexpected ref type'
                    )
                try {
                    await readObject(reference.schema, ref.toString())
                } catch (e) {
                    logger.error(
                        { schema, key, ref, reference, e },
                        'Failed to resolve object reference'
                    )
                }
            }
        }
    } catch (e) {
        logger.error({ schema, key, e })
        throw e
    }
}
