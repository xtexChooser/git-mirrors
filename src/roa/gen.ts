import logger from '../logger.js'
import ip from 'ip'
import { collectROA } from './collector.js'
import { ROARecord } from './types.js'
import { printROAToGoRTRJson } from './gortr.js'

export async function printROA(
    type: 'json' | 'bird2' | 'bird1' | 'grtr' | 'obgpd',
    external: boolean,
    inetFamily: 'ipv4' | 'ipv6' | 'all'
) {
    let roas = await collectROA(external)
    switch (inetFamily) {
        case 'ipv4':
            roas = roas.filter((roa) => ip.isV4Format(ip.cidr(roa.prefix)))
            break
        case 'ipv6':
            roas = roas.filter((roa) => !ip.isV4Format(ip.cidr(roa.prefix)))
            break
        case 'all':
            break
        default:
            logger.error({ inetFamily }, 'unknown inet family type')
    }
    switch (type) {
        case 'json':
            await printROAToJson(roas)
            break
        case 'bird2':
            await printROAToBIRD2(roas)
            break
        case 'bird1':
            await printROAToBIRD1(roas)
            break
        case 'grtr':
            await printROAToGoRTRJson(roas)
            break
        case 'obgpd':
            await printROAToOBGPD(roas)
            break
        default:
            logger.error({ type }, 'unknown ROA type')
    }
}

export async function printROAToJson(roas: ROARecord[]) {
    console.log(JSON.stringify(roas))
}

export async function printROAToBIRD2(roas: ROARecord[]) {
    console.log('# xvnet ROA Generator for BIRD2')
    console.log(`# Updated on ${new Date().toISOString()}`)
    for (const roa of roas) {
        console.log(
            `route ${roa.prefix} max ${roa.maxLength} as ${roa.asn}; # ${roa.source}`
        )
    }
}

export async function printROAToBIRD1(roas: ROARecord[]) {
    console.log('# xvnet ROA Generator for BIRD1')
    console.log(`# Updated on ${new Date().toISOString()}`)
    for (const roa of roas) {
        console.log(
            `roa ${roa.prefix} max ${roa.maxLength} as ${roa.asn}; # ${roa.source}`
        )
    }
}

export async function printROAToOBGPD(roas: ROARecord[]) {
    console.log('# xvnet ROA Generator for OpenBGPD')
    console.log(`# Updated on ${new Date().toISOString()}`)
    console.log('roa-set {')
    for (const roa of roas) {
        console.log(
            `  ${roa.prefix} maxlen ${roa.maxLength} source-as ${roa.asn} # ${roa.source}`
        )
    }
    console.log('}')
}
