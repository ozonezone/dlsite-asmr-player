import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import { VitePWA } from "vite-plugin-pwa";
import generouted from "@generouted/react-router/plugin";

export default defineConfig({
  plugins: [
    react(),
    generouted(),
    tsconfigPaths(),
    VitePWA({
      registerType: "autoUpdate",
      srcDir: "src",
      filename: "service-worker.js",
      strategies: "injectManifest",
      injectRegister: false,
      injectManifest: {
        injectionPoint: null,
      },
      manifest: {
        icons: [
          {
            src: "android-chrome-192x192.png",
            sizes: "192x192",
            type: "image/png",
          },
        ],
      },
    }),
  ],
});
