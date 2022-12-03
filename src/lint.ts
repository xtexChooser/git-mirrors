import { readdir } from "fs/promises";
import logger from "./logger.js";
import { ajv, listObjects, loadSchema, readObject } from "./registry.js";

export async function lintAllSchema() {
    for (const schema of await readdir('./schema')) {
        await lintSchema(schema)
    }
}

export async function lintSchema(schema: string) {
    logger.trace({ schema }, "Check schema")

    const jtd = await loadSchema(schema)

    for (const key of await listObjects(schema)) {
        logger.trace({ schema, key }, "Check object")

        const obj = await readObject(schema, key)

        if (!jtd(obj)) {
            logger.error({ schema, key, obj, error: jtd.errors })
        }
    }
}
