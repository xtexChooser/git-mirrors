import { listObjects, readObject } from '../registry.js'
import ip from 'ip'
import { ROARecord } from './types.js'
import { parseGoRTRJson, GoRTRJson } from './gortr.js'
import axios from 'axios'

export async function collectROA(external = false): Promise<ROARecord[]> {
    let roa = await collectLocalROA('ROUTE')
    roa = roa.concat(await collectLocalROA('ROUTE6'))
    if (external) roa = roa.concat(await collectRemoteROA())
    return roa
}

export async function collectLocalROA(schema: string): Promise<ROARecord[]> {
    const roas: ROARecord[] = []
    await Promise.all(
        (
            await listObjects(schema)
        ).map(async (key) => {
            const obj = await readObject(schema, key)
            const origins = obj['origin'] as number[]
            const subnet = obj[schema.toLowerCase()] as string
            origins.forEach((asn) => {
                roas.push({
                    asn,
                    prefix: subnet,
                    maxLength:
                        obj['max_len'] ||
                        ip.cidrSubnet(subnet).subnetMaskLength,
                    source: `xvnet ${schema}`,
                })
            })
        })
    )
    return roas
}

export async function collectRemoteROA(): Promise<ROARecord[]> {
    return (
        await collectRemoteGoRTRJson(
            'https://rpki.cloudflare.com/rpki.json',
            'IANA'
        )
    ).concat(
        await collectRemoteGoRTRJson(
            'https://dn42.burble.com/roa/dn42_roa_46.json',
            'DN42'
        )
    )
}

export const USER_AGENT = 'xvnet-Registry-Toolkit/1'

export async function collectRemoteGoRTRJson(
    url: string,
    source: string
): Promise<ROARecord[]> {
    const res = await axios.get(url, {
        responseType: 'json',
        headers: { 'User-Agent': USER_AGENT, 'Accept-Encoding': '' }, // Brotli broken when getting DN42 ROA
    })
    return parseGoRTRJson(res.data as GoRTRJson, source)
}
