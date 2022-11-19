import type { NextApiRequest, NextApiResponse } from 'next'
import site_lrs from '../../data/site_lrs.json'
import webfingerData from '../../data/webfinger.json'
import { collect as collectEcho } from './whoami'

type Data = {
    subject: string,
    aliases: string[] | undefined,
    links: any[],
}

export function lookupData(username: string): {
    aliases: string[],
    links: any[]
} | undefined {
    let data = (webfingerData as any[string])[username]
    if (data != undefined && data.aliasTo != undefined)
        return lookupData(data.aliasTo)
    return data
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    let uri: string = req.query['resource'] as string
    if (uri == undefined) {
        res.status(400).end('"resource" query param is not provided')
        return
    }
    if (!uri.startsWith('acct:')) {
        res.status(404).end('Only acct urls are allowed')
        return
    }
    if (uri.indexOf('@') == -1) {
        res.status(404).end('Username is not provided')
        return
    }
    let username = uri.substring(5, uri.indexOf('@')).toLowerCase()
    if (username.startsWith('//'))
        username = username.substring(2)
    let aliases: string[] = []
    let links: any[] = []
    switch (username) {
        case 'this': {
            links = site_lrs
            break
        }
        case 'echo': {
            links = [
                {
                    rel: 'contents',
                    href: JSON.stringify(collectEcho(req)),
                }
            ]
            break
        }
        default: {
            let result = lookupData(username)
            if (result == undefined) {
                res.status(404).end(`User "${username}" not found`)
                return
            } else {
                aliases = result.aliases
                links = result.links
            }
        }
    }
    res.status(200).json({
        subject: uri,
        aliases: aliases.length == 0 ? undefined : aliases,
        links,
    })
}
