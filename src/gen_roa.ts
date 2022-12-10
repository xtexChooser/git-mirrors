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

export async function printROA(type: 'json' | 'bird2' | 'bird1') {
    switch (type) {
        case 'json': await printROAToJson(); break;
        case 'bird2': await printROAToBIRD2(); break;
        case 'bird1': await printROAToBIRD1(); break;
        default: logger.error({ type }, 'unknown ROA type')
    }
}

export async function printROAToJson() {
    console.log(JSON.stringify(await collectROA()))
}

export async function printROAToBIRD2() {
    console.log('# XTEX-VNET ROA Generator for BIRD2')
    console.log(`# Updated on ${new Date().toISOString()}`)
    for (const roa of await collectROA()) {
        console.log(`route ${roa.prefix} max ${roa.maxLength} as ${roa.asn};`)
    }
}

export async function printROAToBIRD1() {
    console.log('# XTEX-VNET ROA Generator for BIRD1')
    console.log(`# Updated on ${new Date().toISOString()}`)
    for (const roa of await collectROA()) {
        console.log(`roa ${roa.prefix} max ${roa.maxLength} as ${roa.asn};`)
    }
}
