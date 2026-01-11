import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  reactStrictMode: false,
  transpilePackages: ['react-map-gl', '@deck.gl/react', '@deck.gl/layers', '@deck.gl/core'],
  webpack: (config) => {
    config.resolve.alias['mapbox-gl'] = 'maplibre-gl';
    return config;
  },
};

export default nextConfig;
