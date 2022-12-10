import { listObjects, readObject } from '../registry.js';
import ip from 'ip'
import { ROARecord } from './types.js';

export async function collectROA(external = false): Promise<ROARecord[]> {
    let roa = await collectLocalROA('ROUTE')
    roa = roa.concat(await collectLocalROA('ROUTE6'))
    if (external)
        roa = roa.concat(await collectRemoteROA())
    return roa
}

export async function collectLocalROA(schema: string): Promise<ROARecord[]> {
    return Promise.all((await listObjects(schema)).map(async key => {
        const obj = await readObject(schema, key)
        const subnet = obj[schema.toLowerCase()] as string
        return {
            asn: obj['origin'] as number,
            prefix: subnet,
            maxLength: ip.cidrSubnet(subnet).subnetMaskLength,
        }
    }))
}

export async function collectRemoteROA(): Promise<ROARecord[]> {
    return []
}
