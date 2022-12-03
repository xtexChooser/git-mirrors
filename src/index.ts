#!/usr/bin/env node

import { createCommand } from "commander"
import { lintAllSchema, lintSchema } from "./lint.js"

const program = createCommand('xvnet-registry')

program.description('Toolkit to work with the XTEX-VNET registry')

program.command('lint [schema]')
    .description('check registry objects')
    .action((schema) => schema ? lintSchema(schema) : lintAllSchema())

program.parse(process.argv)
