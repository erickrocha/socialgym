import { expect } from '@playwright/test';

export const API = process.env.E2E_API_BASE_URL || 'http://localhost:8080';
export const PASSWORD = 'Str0ng!Password';

// Sign-up and sign-in share one limit per address (20 a minute). The flows make more than that between them,
// so each one waits its turn here; the limit stays as it is in the product.
const AUTH_CALLS_PER_MINUTE = 18;
const authCalls = [];
export async function throttleAuth() {
    const now = Date.now();
    while (authCalls.length && now - authCalls[0] > 60_000) authCalls.shift();
    if (authCalls.length >= AUTH_CALLS_PER_MINUTE) {
        const wait = authCalls[0] + 61_000 - now;
        await new Promise((resolve) => setTimeout(resolve, wait));
        return throttleAuth();
    }
    authCalls.push(Date.now());
}

let counter = 0;
export function uniqueEmail(label) {
    counter += 1;
    return `${label}-${Date.now()}-${counter}@example.test`;
}

/** A person created straight through the REST API (the flows that are not about sign-up start here). */
export async function createPerson(request, base) {
    // The stack keeps everyone created in earlier runs, so names carry a suffix to stay unique.
    const label = `${base}${Math.random().toString(36).slice(2, 7)}`;
    const email = uniqueEmail(label);
    await throttleAuth();
    const response = await request.post(`${API}/signup`, {
        data: {
            firstname: label, surname: 'Web', dateOfBirth: '1990-01-01', gender: 'X', email, password: PASSWORD,
            termsVersion: '1.0.0', privacyVersion: '1.0.0', termsAccepted: true, privacyAccepted: true,
        },
    });
    expect(response.status(), 'sign-up through the API').toBe(200);
    const tokens = await response.json();
    const me = await (await request.get(`${API}/workout/api/people/me`, { headers: { authorization: `Bearer ${tokens.accessToken}` } })).json();
    return { email, password: PASSWORD, token: tokens.accessToken, id: me.id, uuid: me.uuid, name: `${label} Web` };
}

/** Accepts the cookie banner ahead of time, as the accessibility tests do. */
export async function skipCookieBanner(page) {
    await page.addInitScript(() => {
        localStorage.setItem('socialgym-cookie-consent-v1', JSON.stringify({ necessary: true, analytics: false }));
    });
}

export async function signInThroughTheForm(page, person) {
    await throttleAuth();
    await page.goto('/login');
    await page.fill('#email', person.email);
    await page.fill('#password', person.password);
    await page.getByRole('button', { name: 'Log In' }).click();
    await page.waitForURL('**/home');
}

/** REST call as `person`, for arranging state and for checking what the UI did. */
export async function api(request, person, method, path, data) {
    const response = await request.fetch(`${API}${path}`, { method, data, headers: { authorization: `Bearer ${person.token}` } });
    const text = await response.text();
    return { status: response.status(), body: text ? JSON.parse(text) : null };
}

/** A saved current address with coordinates: friend suggestions are searched around it. */
export async function giveAddress(request, person) {
    const { status, body } = await api(request, person, 'POST', '/workout/api/people/me/address', {
        personId: person.id, addressLine1: 'Rua Augusta 100', locality: 'Sao Paulo', administrativeArea: 'SP',
        countryCode: 'BR', postalCode: '01304-000', latitude: -23.5535, longitude: -46.6599, current: true,
    });
    expect(status, `address: ${JSON.stringify(body)}`).toBeLessThan(300);
}

/**
 * A run right after another one finds the sign-in limit still used up. One probe tells; when it is
 * used up, wait the minute out so the flows' own pacing (throttleAuth) starts from an empty window.
 */
export async function waitForAuthWindow(request) {
    const probe = () => request.post(`${API}/login`, { form: { email: 'nobody@example.test', password: 'x' } });
    if ((await probe()).status() === 429) {
        await new Promise((resolve) => setTimeout(resolve, 61_000));
    }
}
