import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:5150/api/:path*',
      },
    ];
  },
  experimental: {
    serverComponentsExternalPackages: ['@simplewebauthn/browser'],
  },
};

export default nextConfig;
