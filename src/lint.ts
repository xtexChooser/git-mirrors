import logger from './logger.js';
import { listObjects, loadSchema, readObject } from './registry.js';
import ip from 'ip'

export async function lintAll() {
    await lintAllSchema()
    await lintIPAddrs('ROUTE')
    await lintIPAddrs('ROUTE6')
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

        const jtd = await loadSchema(schema)
        if (!jtd(obj)) {
            logger.error({ schema, key, obj, error: jtd.errors })
        }

        const schemaSelfRefKey = schema.replace('-', '_').toLowerCase()
        if (obj[schemaSelfRefKey] != key) {
            logger.error({ schema, key, value: obj[schemaSelfRefKey]?.toString() }, 'Object does not have a self-reference')
        }
    } catch (e) {
        logger.error({ schema, key, e })
        throw e
    }
}

export async function lintIPAddrs(schema: string) {
    const nets = await listObjects(schema)
    for (const net of nets) {
        const netNet = ip.cidrSubnet(net)
        for (const other of nets) {
            const otherNet = ip.cidrSubnet(other)
            if (net != other) {
                if (netNet.contains(otherNet.firstAddress) || netNet.contains(otherNet.lastAddress)) {
                    logger.error({ schema, net, other }, 'IP subnets conflict')
                }
            }
        }
    }
}
