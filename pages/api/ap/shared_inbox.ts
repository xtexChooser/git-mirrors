import type { NextApiRequest, NextApiResponse } from 'next'

type Data = {
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    console.log(req.body)
    console.log(JSON.stringify(req.body))
    res.status(200).end()
}
