const is_prod = process.env.NODE_ENV === "production";

const output = is_prod ? "export" : undefined;

/** @type {import('next').NextConfig} */
const nextConfig = {
  output,
};

module.exports = nextConfig;
