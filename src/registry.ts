import Ajv, { JTDSchemaType, ValidateFunction } from "ajv/dist/jtd.js";
import { readdir, readFile } from "fs/promises";
import json5 from "json5";

const ajv = new Ajv()
export { ajv }

export async function readObject(type: string, key: string): Promise<object> {
    return json5.parse(await readFile(`${type.toLowerCase()}/${key.replace('/', '_')}`, 'utf-8'))
}

export async function listObjects(type: string): Promise<string[]> {
    return (await readdir(type.toLowerCase())).map((key) => key.replace('_', '/'))
}

export type Schema = {
    jtd: JTDSchemaType<object>,
}

export async function loadSchema(schema: string): Promise<ValidateFunction<object>> {
    return ajv.compile((await readObject('schema', schema) as Schema).jtd)
}
