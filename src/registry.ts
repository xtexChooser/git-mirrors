import Ajv, { JTDSchemaType, ValidateFunction } from "ajv/dist/jtd.js";
import { readdir, readFile, writeFile } from "fs/promises";
import json5 from "json5";
import sortKeys from "sort-keys";
import logger from "./logger.js";

const ajv = new Ajv()
export { ajv }

export async function listObjects(type: string): Promise<string[]> {
    return (await readdir(type.toLowerCase())).map((key) => key.replace('_', '/').substring(0, key.length - 6))
}

export function getObjectPath(type: string, key: string): string {
    return `${type.toLowerCase()}/${key.replace('/', '_')}.json5`;
}

export function serializeObject(obj: object): string {
    return json5.stringify(sortKeys(obj, { deep: true }), { space: 2, quote: '"' }) + '\n'
}

export function deserializeObject(obj: string): object {
    return json5.parse(obj)
}

export async function readObjectContent(type: string, key: string): Promise<string> {
    return await readFile(getObjectPath(type, key), 'utf-8')
}

export async function readObject(type: string, key: string): Promise<object> {
    try {
        return deserializeObject(await readObjectContent(type, key))
    } catch (e) {
        logger.error({ type, key, e }, "Failed to read object")
        throw e
    }
}

export async function writeObject(type: string, key: string, obj: object) {
    await writeFile(getObjectPath(type, key), serializeObject(obj), 'utf-8')
}

export type Schema = {
    jtd: JTDSchemaType<object>,
}

export async function loadSchema(schema: string): Promise<ValidateFunction<object>> {
    try {
        return ajv.compile((await readObject('schema', schema) as Schema).jtd)
    } catch (e) {
        logger.error({ schema, e }, "Failed to load schema")
        throw e
    }
}
