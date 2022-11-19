import type { NextApiRequest, NextApiResponse } from 'next'
import site_lrs from '../../data/site_lrs.json'

type Data = {
    links: any[],
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    res.status(200).json({
        links: site_lrs,
    })
}
