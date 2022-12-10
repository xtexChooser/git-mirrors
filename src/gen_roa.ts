import logger from './logger.js'
import { listObjects, readObject } from './registry.js';
import ip from 'ip'

export type ROARecord = {
    asn: number,
    prefix: string,
    maxLength: number,
    inetFamily: 'ipv4' | 'ipv6',
}

export async function collectROA(): Promise<ROARecord[]> {
    return (await collectROAv4()).concat(await collectROAv6())
}

export async function collectROAv4(): Promise<ROARecord[]> {
    return Promise.all((await listObjects('ROUTE')).map(async key => {
        const obj = await readObject('ROUTE', key)
        return {
            asn: obj['origin'] as number,
            prefix: obj['route'] as string,
            maxLength: ip.cidrSubnet(obj['route'] as string).subnetMaskLength,
            inetFamily: 'ipv4'
        }
    }))
}

export async function collectROAv6(): Promise<ROARecord[]> {
    return Promise.all((await listObjects('ROUTE6')).map(async key => {
        const obj = await readObject('ROUTE6', key)
        return {
            asn: obj['origin'] as number,
            prefix: obj['route6'] as string,
            maxLength: ip.cidrSubnet(obj['route6'] as string).subnetMaskLength,
            inetFamily: 'ipv6'
        }
    }))
}

export async function printROA(type: 'json' | 'bird2' | 'bird1' | 'grtr' | 'obgpd') {
    const roas = await collectROA()
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
