import { defineConfig, type Plugin, type ViteDevServer } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path';
import fs from 'fs';

const gumballsPackagePath = path.resolve(__dirname, '..', 'gumballs');

function return404onMissingAssetPlugin(): Plugin {
  const publicDir = path.resolve(process.cwd(), 'public');

  return {
    name: 'return-404-on-missing-asset-plugin',

    configureServer(server: ViteDevServer) {
      server.middlewares.use((req, res, next) => {
        const url = req.url || '';

        if (url.includes('assets') && !url.startsWith('/@')) {
          const cleanUrl = url.split('?')[0];
          const filePath = path.join(publicDir, cleanUrl);

          if (!fs.existsSync(filePath)) {
            res.statusCode = 404;
            res.setHeader('Content-Type', 'text/plain');
            res.end(`404 Not Found: ${url}`);
            return;
          }
        }
        
        next();
      })
    }
  }
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), return404onMissingAssetPlugin()],
  server: {
    fs: {
      allow: [
        '.',
        gumballsPackagePath
      ]
    }
  }
})
