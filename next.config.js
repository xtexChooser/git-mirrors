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
  async headers() {
    return [
      {
        source: '/.well-known/nodeinfo',
        headers: [
          {
            key: 'Content-Type',
            value: 'application/json; profile="http://nodeinfo.diaspora.software/ns/schema/2.1#"',
          },
        ],
      },
      {
        source: '/nodeinfo.json',
        headers: [
          {
            key: 'Content-Type',
            value: 'application/json; profile="http://nodeinfo.diaspora.software/ns/schema/2.1#"',
          },
        ],
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
