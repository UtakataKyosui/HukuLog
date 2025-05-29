import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: process.env.INTERNAL_API_URL + '/api/:path*',
      },
    ];
  },
  experimental: {
    serverComponentsExternalPackages: ['@simplewebauthn/browser'],
  },
};

export default nextConfig;
