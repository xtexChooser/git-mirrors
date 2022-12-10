import logger from '../logger.js'
import { collectROA } from './collector.js'
import { ROARecord } from './types.js';

export async function printROA(type: 'json' | 'bird2' | 'bird1' | 'grtr' | 'obgpd', external: boolean) {
    const roas = await collectROA(external)
    switch (type) {
        case 'json': await printROAToJson(roas); break;
        case 'bird2': await printROAToBIRD2(roas); break;
        case 'bird1': await printROAToBIRD1(roas); break;
        case 'grtr': await printROAToGoRTRJson(roas); break;
        case 'obgpd': await printROAToOBGPD(roas); break;
        default: logger.error({ type }, 'unknown ROA type')
    }
}

export async function printROAToJson(roas: ROARecord[]) {
    console.log(JSON.stringify(roas))
}

export async function printROAToBIRD2(roas: ROARecord[]) {
    console.log('# XTEX-VNET ROA Generator for BIRD2')
    console.log(`# Updated on ${new Date().toISOString()}`)
    for (const roa of roas) {
        console.log(`route ${roa.prefix} max ${roa.maxLength} as ${roa.asn};`)
    }
}

export async function printROAToBIRD1(roas: ROARecord[]) {
    console.log('# XTEX-VNET ROA Generator for BIRD1')
    console.log(`# Updated on ${new Date().toISOString()}`)
    for (const roa of roas) {
        console.log(`roa ${roa.prefix} max ${roa.maxLength} as ${roa.asn};`)
    }
}

type GoRTRJson = {
    metadata: {
        counts: number,
        generated: number,
        valid: number,
    }
    roas: {
        prefix: string,
        maxLength: number,
        asn: string,
    }[]
}

export async function printROAToGoRTRJson(roas: ROARecord[]) {
    const timestamp = Math.floor(Date.now() / 1000)
    console.log(JSON.stringify({
        metadata: {
            counts: roas.length,
            generated: timestamp,
            valid: timestamp + (60 * 15)
        },
        roas: roas.map(roa => {
            return {
                prefix: roa.prefix,
                maxLength: roa.maxLength,
                asn: `AS${roa.asn}`
            }
        })
    } as GoRTRJson))
}

export async function printROAToOBGPD(roas: ROARecord[]) {
    console.log('# XTEX-VNET ROA Generator for OpenBGPD')
    console.log(`# Updated on ${new Date().toISOString()}`)
    console.log('roa-set {')
    for (const roa of roas) {
        console.log(`  ${roa.prefix} maxlen ${roa.maxLength} source-as ${roa.asn}`)
    }
    console.log('}')
}
