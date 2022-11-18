/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
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
    ]
  },
}

module.exports = nextConfig
