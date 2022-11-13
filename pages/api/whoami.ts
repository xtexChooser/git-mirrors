import type { NextApiRequest, NextApiResponse } from 'next'
import type { IncomingHttpHeaders } from 'node:http'

type Data = {
  httpVersion: string,
  headers: IncomingHttpHeaders,
  address: string,
  port: number,
  ipv6: boolean,
  method: string,
  userAgent: string | undefined,
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  res.status(200).json({
    httpVersion: req.httpVersion,
    headers: req.headers,
    address: req.socket.remoteAddress!,
    port: req.socket.remotePort!,
    ipv6: req.socket.remoteFamily == 'IPv6',
    method: req.method!,
    userAgent: req.headers["user-agent"],
  })
}
