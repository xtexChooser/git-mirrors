/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  async redirects() {
    return [
      {
        source: '/blog/:path',
        destination: 'https://blog.xtexx.ml/:path',
        permanent: true,
      },
    ]
  },
}

module.exports = nextConfig
