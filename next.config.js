/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: true,
    swcMinify: true,
    poweredByHeader: false,
    async redirects() {
        return [
            {
                source: "/blog/:path*",
                destination: "https://blog.xtexx.eu.org/:path*",
                permanent: true,
            },
        ];
    },
    async headers() {
        const nodeinfo = {
            key: "Content-Type",
            value: 'application/json; profile="http://nodeinfo.diaspora.software/ns/schema/2.1#"',
        };
        const cors = [
            {
                key: "Access-Control-Allow-Origin",
                value: "*",
            },
            {
                key: "Access-Control-Expose-Headers",
                value: "*",
            },
            {
                key: "Access-Control-Max-Age",
                value: "86400",
            },
            {
                key: "Access-Control-Allow-Methods",
                value: "*",
            },
            {
                key: "Access-Control-Allow-Headers",
                value: "*",
            },
        ];
        return [
            {
                source: "/.well-known/nodeinfo",
                headers: [nodeinfo],
            },
            {
                source: "/nodeinfo.json",
                headers: [nodeinfo],
            },
            {
                source: "/.well-known/host-meta",
                headers: [
                    {
                        key: "Content-Type",
                        value: "application/xrd+xml",
                    },
                ],
            },
            {
                source: "/.well-known/matrix/:path*",
                headers: [
                    {
                        key: "Content-Type",
                        value: "application/json; charset=utf-8",
                    },
                ],
            },
            {
                source: "/ap/:path*",
                headers: [
                    {
                        key: "Content-Type",
                        value: 'application/ld+json; profile="https://www.w3.org/ns/activitystreams"; charset=utf-8',
                    },
                ],
            },
        ];
    },
    async rewrites() {
        return [
            {
                source: "/.well-known/host-meta.json",
                destination: "/api/host-meta.json",
            },
            {
                source: "/.well-known/webfinger",
                destination: "/api/webfinger",
            },
            {
                source: "/ap/api/:path*",
                destination: "/api/ap/:path*",
            },
        ];
    },
};

module.exports = nextConfig;
