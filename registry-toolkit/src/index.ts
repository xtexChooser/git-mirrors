#!/usr/bin/env node

import { createCommand } from 'commander'
import { formatAllSchema, formatObject, formatSchema } from './format.js'
import { printROA } from './roa/gen.js'
import { lintAll, lintObject, lintSchema } from './lint.js'

const program = createCommand('xvnet-registry')

program.description('Toolkit to work with the xvnet registry')

program
    .command('lint [schema] [key]')
    .description('check objects')
    .action((schema, key) =>
        schema
            ? key
                ? lintObject(schema, key)
                : lintSchema(schema)
            : lintAll()
    )

program
    .command('format [schema] [key]')
    .description('format objects')
    .action((schema, key) =>
        schema
            ? key
                ? formatObject(schema, key)
                : formatSchema(schema)
            : formatAllSchema()
    )

program
    .command('roa <type> <inetFamily>')
    .option('-e, --external', 'Include ROA from external', false)
    .description('print ROA')
    .action((type, inetFamily, opts) =>
        printROA(type, opts.external, inetFamily)
    )

program.parse(process.argv)
