/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  poweredByHeader: false,
  async redirects() {
    return [
      {
        source: '/blog/:path*',
        destination: 'https://blog.xtexx.ml/:path*',
        permanent: true,
      },
    ]
  },
  async headers() {//application/ld+json; profile="https://www.w3.org/ns/activitystreams"
    const nodeinfo = {
      key: 'Content-Type',
      value: 'application/json; profile="http://nodeinfo.diaspora.software/ns/schema/2.1#"',
    }
    return [
      {
        source: '/.well-known/nodeinfo',
        headers: [nodeinfo],
      },
      {
        source: '/nodeinfo.json',
        headers: [nodeinfo],
      },
      {
        source: '/.well-known/host-meta',
        headers: [
          {
            key: 'Content-Type',
            value: 'application/xrd+xml',
          },
        ],
      },
      {
        source: '/ap',
        headers: [
          {
            key: 'Content-Type',
            value: 'application/ld+json; profile="https://www.w3.org/ns/activitystreams"; charset=utf-8',
          },
        ],
      },
    ]
  },
  async rewrites() {
    return [
      {
        source: '/.well-known/host-meta.json',
        destination: '/api/host-meta.json',
      },
      {
        source: '/.well-known/webfinger',
        destination: '/api/webfinger',
      },
    ]
  },
}

module.exports = nextConfig
