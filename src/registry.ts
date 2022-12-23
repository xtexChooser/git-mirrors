import Ajv, { JTDSchemaType, ValidateFunction } from 'ajv/dist/jtd.js'
import { readdir, readFile, writeFile } from 'fs/promises'
import yaml from 'yaml'
import sortKeys from 'sort-keys'
import logger from './logger.js'

const ajv = new Ajv()
export { ajv }

export async function listObjects(type: string): Promise<string[]> {
    return (await readdir(type.toLowerCase())).map((key) =>
        key.replace('_', '/').substring(0, key.length - 5)
    )
}

export function getObjectPath(type: string, key: string): string {
    return `${type.toLowerCase()}/${key.replace('/', '_')}.yaml`
}

export function serializeObject(obj: object): string {
    return yaml.stringify(sortKeys(obj, { deep: true }))
}

export function deserializeObject(obj: string): object {
    return yaml.parse(obj)
}

export async function readObjectContent(
    type: string,
    key: string
): Promise<string> {
    return await readFile(getObjectPath(type, key), 'utf-8')
}

export async function readObject(type: string, key: string): Promise<object> {
    try {
        return deserializeObject(await readObjectContent(type, key))
    } catch (e) {
        logger.error({ type, key, e }, 'Failed to read object')
        throw e
    }
}

export async function writeObject(type: string, key: string, obj: object) {
    await writeFile(getObjectPath(type, key), serializeObject(obj), 'utf-8')
}

export type Schema = {
    jtd: JTDSchemaType<object>
    ref?: ObjectReferenceRecord[]
}

export type ObjectReferenceRecord = {
    path: string
    schema: string
}

export async function loadSchema(
    schema: Schema
): Promise<ValidateFunction<object>> {
    try {
        return ajv.compile(schema.jtd)
    } catch (e) {
        logger.error({ schema, e }, 'Failed to parse schema JTD')
        throw e
    }
}
