#!/usr/bin/env node

import { createCommand } from "commander"
import { lintAllSchema, lintObject, lintSchema } from "./lint.js"

const program = createCommand('xvnet-registry')

program.description('Toolkit to work with the XTEX-VNET registry')

program.command('lint [schema] [key]')
    .description('check registry objects')
    .action((schema, key) => schema ? (key ? lintObject(schema, key) : lintSchema(schema)) : lintAllSchema())

program.parse(process.argv)
