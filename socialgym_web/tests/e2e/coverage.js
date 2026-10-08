import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

// Line coverage of the flows: with VITE_COVERAGE=true the Vite dev server instruments the sources, the page
// keeps its counters in window.__coverage__, and they are written out when the document unloads (a
// navigation or a page closed with runBeforeUnload). `nyc report` turns the files into a report.
export const COVERAGE = process.env.VITE_COVERAGE === 'true';
const OUTPUT = '.nyc_output';

export async function instrument(context) {
    if (!COVERAGE) return;
    await context.exposeFunction('__collectCoverage', (json) => {
        fs.mkdirSync(OUTPUT, { recursive: true });
        fs.writeFileSync(path.join(OUTPUT, `${crypto.randomUUID()}.json`), json);
    });
    await context.addInitScript(() => {
        window.addEventListener('beforeunload', () => {
            if (window.__coverage__) window.__collectCoverage(JSON.stringify(window.__coverage__));
        });
    });
}

/** Closes a page so that its last document reports its counters. */
export async function closeWithCoverage(page) {
    if (COVERAGE && !page.isClosed()) await page.close({ runBeforeUnload: true });
}
