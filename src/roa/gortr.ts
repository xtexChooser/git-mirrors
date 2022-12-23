import { ROARecord } from './types'

export type GoRTRJson = {
    metadata: {
        counts: number
        generated: number
        valid: number
    }
    roas: {
        prefix: string
        maxLength: number
        asn: string
        ta?: string
    }[]
}

export async function printROAToGoRTRJson(roas: ROARecord[]) {
    const timestamp = Math.floor(Date.now() / 1000)
    console.log(
        JSON.stringify({
            metadata: {
                counts: roas.length,
                generated: timestamp,
                valid: timestamp + 60 * 15,
            },
            roas: roas.map((roa) => {
                return {
                    prefix: roa.prefix,
                    maxLength: roa.maxLength,
                    asn: `AS${roa.asn}`,
                    ta: roa.source,
                }
            }),
        } as GoRTRJson)
    )
}

export function parseGoRTRJson(roa: GoRTRJson, source: string): ROARecord[] {
    const timestamp = Math.floor(Date.now() / 1000)
    if (roa.metadata.valid < timestamp) throw 'GoRTR ROA is out-dated'
    return roa.roas.map((roa) => {
        if (!roa.asn.startsWith('AS')) throw `invalid ASN ${roa.asn}`
        return {
            asn: parseInt(roa.asn.substring(2)),
            prefix: roa.prefix,
            maxLength: roa.maxLength,
            source: roa.ta ? `${source} - ${roa.ta}` : source,
        }
    })
}
