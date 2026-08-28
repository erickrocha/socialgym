import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

async function expectLevelA(page) {
    const result = await new AxeBuilder({ page })
        .withTags(['wcag2a', 'wcag21a'])
        .analyze();
    expect(result.violations).toEqual([]);
}

test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
        localStorage.setItem('socialgym-cookie-consent-v1', JSON.stringify({ necessary: true, analytics: false }));
    });
});

test('login has no WCAG 2.1 level A violations', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('form')).toBeVisible();
    await expectLevelA(page);
});

test('signup has no WCAG 2.1 level A violations', async ({ page }) => {
    await page.goto('/signup');
    await expect(page.locator('form')).toBeVisible();
    await expectLevelA(page);
});

test('feed has no WCAG 2.1 level A violations', async ({ page }) => {
    const payload = btoa(JSON.stringify({ exp: Math.floor(Date.now() / 1000) + 3600 }))
        .replaceAll('+', '-').replaceAll('/', '_').replaceAll('=', '');
    await page.addInitScript((jwtPayload) => {
        localStorage.setItem('auth', JSON.stringify({ accessToken: `x.${jwtPayload}.x`, tokenType: 'Bearer' }));
    }, payload);
    await page.route('https://localhost/**', async (route) => {
        if (route.request().url().includes('/workout/api/people/me')) {
            await route.fulfill({ json: { id: 1, uuid: 'person-1', firstname: 'Pessoa', surname: 'Teste', gender: 'other' } });
        } else {
            await route.fulfill({ json: [] });
        }
    });
    await page.goto('/home');
    await expect(page.locator('.feed')).toBeVisible();
    await expectLevelA(page);
});
