import type { NextApiRequest, NextApiResponse } from 'next'

type Data = {
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data>
) {
    console.log(req)
    console.log(JSON.stringify(req))
    res.status(200).end()
}
