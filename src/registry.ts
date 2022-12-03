import Ajv, { JTDSchemaType, ValidateFunction } from "ajv/dist/jtd.js";
import { readdir, readFile, writeFile } from "fs/promises";
import json5 from "json5";
import sortKeys from "sort-keys";

const ajv = new Ajv()
export { ajv }

export async function listObjects(type: string): Promise<string[]> {
    return (await readdir(type.toLowerCase())).map((key) => key.replace('_', '/'))
}

export function getObjectPath(type: string, key: string): string {
    return `${type.toLowerCase()}/${key.replace('/', '_')}`;
}

export function serializeObject(obj: object): string {
    return json5.stringify(sortKeys(obj, { deep: true }), { space: 2 })
}

export function deserializeObject(obj: string): object {
    return json5.parse(obj)
}

export async function readObjectContent(type: string, key: string): Promise<string> {
    return await readFile(getObjectPath(type, key), 'utf-8')
}

export async function readObject(type: string, key: string): Promise<object> {
    return deserializeObject(await readObjectContent(type, key))
}

export async function writeObject(type: string, key: string, obj: object) {
    await writeFile(getObjectPath(type, key), serializeObject(obj), 'utf-8')
}

export type Schema = {
    jtd: JTDSchemaType<object>,
}

export async function loadSchema(schema: string): Promise<ValidateFunction<object>> {
    return ajv.compile((await readObject('schema', schema) as Schema).jtd)
}
