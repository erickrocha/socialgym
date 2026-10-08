import { defineConfig, devices } from '@playwright/test';

// C-010 task 13: the web client in a browser against the infra/test stack, through its gateway (REST on
// 8080). One command: scripts/e2e-web.sh (starts and stops the stack). The Vite dev server is started
// here, pointed at that gateway, on localhost:5173: that origin is on the services' default CORS list.
export default defineConfig({
    testDir: './tests/e2e',
    reporter: [['list'], ['json', { outputFile: 'test-results/e2e-report.json' }]],
    // The flows share one stack and sign people up as they go; they run in order, one at a time.
    workers: 1,
    fullyParallel: false,
    timeout: 180_000,
    expect: { timeout: 10_000 },
    use: {
        baseURL: 'http://localhost:5173',
        ...devices['Desktop Chrome'],
        trace: 'retain-on-failure',
        screenshot: 'only-on-failure',
    },
    webServer: {
        command: 'npm run dev -- --host localhost --port 5173 --strictPort',
        url: 'http://localhost:5173',
        env: { VITE_API_BASE_URL: process.env.E2E_API_BASE_URL || 'http://localhost:8080' },
        reuseExistingServer: false,
        timeout: 120_000,
    },
});
