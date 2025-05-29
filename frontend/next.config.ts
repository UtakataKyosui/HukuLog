import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: `${process.env.INTERNAL_API_URL ? process.env.INTERNAL_API_URL : "http://localhost:5151"}/api/:path*`,
      },
    ];
  },
  serverExternalPackages: ['@simplewebauthn/browser'],
  output: 'standalone',
};

export default nextConfig;
