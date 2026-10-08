import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react-swc'
import tailwindcss from '@tailwindcss/vite'
import istanbul from 'vite-plugin-istanbul'


// https://vite.dev/config/
export default defineConfig({
    server:{
        port: 3000,
        strictPort: true,
    },
    plugins: [react(),
        tailwindcss(),
        // Line coverage of the browser flows (scripts/e2e-web.sh with COVERAGE=1): the sources are
        // instrumented only when VITE_COVERAGE=true, so the normal dev server and build are untouched.
        ...(process.env.VITE_COVERAGE === 'true'
            ? [istanbul({include: 'src/**', exclude: ['node_modules', 'tests'], extension: ['.js', '.jsx'], requireEnv: false})]
            : [])],
})
