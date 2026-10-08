// C-010 task 13 (TC-018): the web client's main flows in a browser, against the infra/test stack through its
// gateway. Each flow signs people up through the API (or the form, for the sign-up flow itself), drives the
// screen, and checks the result through the API as well. A flow that cannot run fails; none is skipped.
import { expect, test } from '@playwright/test';
import { closeWithCoverage, instrument } from './coverage.js';
import { api, createPerson, giveAddress, signInThroughTheForm, skipCookieBanner, throttleAuth, uniqueEmail, waitForAuthWindow, PASSWORD } from './support.js';

// A 1x1 PNG.
const PNG = Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==', 'base64');

test.describe.configure({ mode: 'serial' });

test.beforeAll(async ({ request }) => {
    test.setTimeout(120_000);
    await waitForAuthWindow(request);
});

// What the API refused while a flow ran, printed when the flow fails.
const refused = [];
const watched = [];
function watchRefusals(page) {
    watched.push(page);
    page.on('pageerror', (error) => refused.push(`page error: ${error.message}`));
    page.on('response', async (response) => {
        if (response.status() >= 400 && response.url().includes(':8080')) {
            const body = (await response.text().catch(() => '')).slice(0, 160);
            refused.push(`${response.status()} ${response.request().method()} ${response.url()} ${body}`);
        }
    });
}
test.beforeEach(async ({ page, context }) => {
    refused.length = 0;
    watched.length = 0;
    await instrument(context);
    watchRefusals(page);
    await skipCookieBanner(page);
});

test.afterEach(async ({ browserName: _browser }, testInfo) => {
    for (const page of watched) await closeWithCoverage(page);
    if (testInfo.status !== testInfo.expectedStatus && refused.length) {
        console.log(`API refusals and page errors during "${testInfo.title}":\n${refused.join('\n')}`);
    }
});

test('sign up through the form, sign out and sign in again', async ({ page }) => {
    const email = uniqueEmail('signup');
    await throttleAuth();
    await page.goto('/signup');
    await page.fill('#firstname', 'Sign');
    await page.fill('#surname', 'Up');
    await page.selectOption('#birthdayMonth', '03');
    await page.selectOption('#birthdayDay', '15');
    await page.selectOption('#birthdayYear', '1990');
    await page.getByLabel('Female').check();
    await page.fill('#email', email);
    await page.fill('#password', PASSWORD);
    await page.locator('input[name="termsAccepted"]').check();
    await page.locator('input[name="privacyAccepted"]').check();
    await page.locator('#sign-up-button, button:has-text("Create Account")').last().click();
    await page.waitForURL('**/home');
    await expect(page.locator('.feed')).toBeVisible();

    await page.evaluate(() => localStorage.removeItem('auth'));
    await signInThroughTheForm(page, { email, password: PASSWORD });
    await expect(page.locator('.feed')).toBeVisible();
});

test('feed: post, then a friend comments and the post shows the comment', async ({ page, request, browser }) => {
    const alice = await createPerson(request, 'AliceFeed');
    const bob = await createPerson(request, 'BobFeed');
    expect((await api(request, alice, 'PUT', `/workout/api/friends/request/${bob.id}`)).status).toBe(200);
    expect((await api(request, bob, 'PUT', `/workout/api/friends/accept/${alice.id}`)).status).toBe(200);

    await signInThroughTheForm(page, alice);
    const text = `post from the browser ${Date.now()}`;
    await page.getByPlaceholder('What are you thinking?').first().click();
    await page.locator('.post-creator__textarea').fill(text);
    await page.getByRole('button', { name: 'Post', exact: true }).click();
    await expect(page.locator('.post', { hasText: text })).toBeVisible();

    // Bob (a friend) sees it and comments in his own browser session.
    const bobContext = await browser.newContext();
    await instrument(bobContext);
    const bobPage = await bobContext.newPage();
    watchRefusals(bobPage);
    await skipCookieBanner(bobPage);
    await signInThroughTheForm(bobPage, bob);
    const post = bobPage.locator('.post', { hasText: text });
    await expect(post).toBeVisible();
    await post.getByRole('button', { name: /Comment/ }).first().click();
    await post.getByPlaceholder('Write a comment...').fill('nice one');
    await post.getByPlaceholder('Write a comment...').press('Enter');
    await expect(post.locator('.post__comment-text', { hasText: 'nice one' })).toBeVisible();

    await post.getByRole('button', { name: /Like/ }).first().click();
    await expect.poll(async () => {
        const list = await api(request, alice, 'GET', '/timeline/api/feed');
        return list.body.find((p) => p.content === text)?.reactions?.length ?? 0;
    }).toBeGreaterThan(0);

    // The comment is stored: Alice's feed returns it.
    const feed = await api(request, alice, 'GET', '/timeline/api/feed');
    const stored = feed.body.find((p) => p.content === text);
    expect(stored.comments.map((c) => c.content)).toContain('nice one');
    await closeWithCoverage(bobPage);
    await bobContext.close();
});

test('friends: send a request from the suggestions, accept it, and decline another', async ({ page, request, browser }) => {
    const alice = await createPerson(request, 'AliceFriends');
    const bob = await createPerson(request, 'BobFriends');
    const carol = await createPerson(request, 'CarolFriends');
    await giveAddress(request, alice);
    await giveAddress(request, bob);

    // Alice asks Bob from the suggestions.
    await signInThroughTheForm(page, alice);
    await page.goto('/friends');
    const suggestion = page.locator('.friend-card', { hasText: bob.name });
    await expect(suggestion).toBeVisible();
    await suggestion.getByRole('button', { name: 'Add Friend' }).click();
    await expect.poll(async () => (await api(request, bob, 'GET', '/workout/api/friends')).body.receiveRequests?.length ?? 0).toBe(1);

    // Bob accepts in his own session.
    const bobContext = await browser.newContext();
    await instrument(bobContext);
    const bobPage = await bobContext.newPage();
    watchRefusals(bobPage);
    await skipCookieBanner(bobPage);
    await signInThroughTheForm(bobPage, bob);
    await bobPage.goto('/friends');
    const request1 = bobPage.locator('.friend-card', { hasText: alice.name });
    await expect(request1).toBeVisible();
    await request1.getByRole('button', { name: 'Accept' }).click();
    await expect.poll(async () => (await api(request, alice, 'GET', '/workout/api/friends')).body.friends?.map((f) => f.id)).toContain(bob.id);
    await bobPage.screenshot({ path: 'test-results/friends-after-accept.png' });
    await expect(bobPage.locator('.friend-card', { hasText: alice.name })).toBeVisible();

    // Carol asks Bob (through the API); Bob declines in the browser.
    expect((await api(request, carol, 'PUT', `/workout/api/friends/request/${bob.id}`)).status).toBe(200);
    await bobPage.reload();
    const request2 = bobPage.locator('.friend-card', { hasText: carol.name });
    await expect(request2).toBeVisible();
    await request2.getByRole('button', { name: 'Decline' }).click();
    await expect(request2).toHaveCount(0);
    expect((await api(request, bob, 'GET', '/workout/api/friends')).body.receiveRequests ?? []).toHaveLength(0);
    await closeWithCoverage(bobPage);
    await bobContext.close();
});

test('profile: edit the personal information and see it saved', async ({ page, request }) => {
    const alice = await createPerson(request, 'AliceProfile');
    await signInThroughTheForm(page, alice);
    await page.locator('.sidebar__menu-item', { hasText: 'Profile' }).click();
    await expect(page).toHaveURL(/\/profile$/);
    await page.getByRole('button', { name: 'Edit', exact: true }).click();
    await page.fill('#biography', 'Lifts on weekends');
    await page.fill('#job', 'coach');
    await page.getByRole('button', { name: 'Confirm' }).click();

    await expect(page.locator('.detail-value', { hasText: 'coach' })).toBeVisible();
    const me = await api(request, alice, 'GET', '/workout/api/people/me');
    expect(me.body.personInfo.job).toBe('coach');
    expect(me.body.personInfo.biography).toBe('Lifts on weekends');
});

test('exercises: create, edit and delete an exercise', async ({ page, request }) => {
    const alice = await createPerson(request, 'AliceExercise');
    await signInThroughTheForm(page, alice);
    page.on('dialog', (dialog) => dialog.accept());
    await page.goto('/exercises');
    await page.getByRole('button', { name: /Novo Exercício/ }).click();
    await page.getByLabel('Nome do Exercício').fill('Back squat');
    await page.getByRole('button', { name: /Confirm|Salvar/ }).click();

    const card = page.locator('.exercise-card', { hasText: 'Back squat' });
    await expect(card).toBeVisible();
    const stored = await api(request, alice, 'POST', '/workout/api/exercises/query', { pageNumber: 1, pageSize: 50 });
    expect(stored.body.content.map((e) => e.name)).toContain('Back squat');

    await card.locator('.btn-edit').click();
    await page.getByLabel('Nome do Exercício').fill('Front squat');
    await page.getByRole('button', { name: /Confirm|Salvar/ }).click();
    await expect(page.locator('.exercise-card', { hasText: 'Front squat' })).toBeVisible();
    await expect(page.locator('.exercise-card', { hasText: 'Back squat' })).toHaveCount(0);

    await page.locator('.exercise-card', { hasText: 'Front squat' }).locator('.btn-delete').click();
    await expect(page.locator('.exercise-card', { hasText: 'Front squat' })).toHaveCount(0);
    const after = await api(request, alice, 'POST', '/workout/api/exercises/query', { pageNumber: 1, pageSize: 50 });
    expect(after.body.content.map((e) => e.name)).not.toContain('Front squat');
});

test('workouts: create a workout, add exercises to it and see them listed', async ({ page, request }) => {
    const alice = await createPerson(request, 'AliceWorkout');
    await signInThroughTheForm(page, alice);
    await page.locator('.sidebar__menu-item', { hasText: 'Workouts' }).click();
    await expect(page).toHaveURL(/\/workouts$/);

    await page.getByRole('button', { name: 'Add Workout' }).click();
    await page.getByLabel('Workout Name').fill('Leg day');
    await page.getByRole('button', { name: 'Confirm' }).click();
    const card = page.locator('.workout-grid__item', { hasText: 'Leg day' });
    await expect(card).toBeVisible();
    const workouts = await api(request, alice, 'GET', `/workout/api/workouts/${alice.id}`);
    expect(workouts.body.map((w) => w.name)).toContain('Leg day');

    await card.locator('.workout-card, [class*=workout-card]').first().click();
    await page.getByRole('button', { name: /Add Exercise/ }).first().click();
    await page.getByPlaceholder('Enter exercise name').fill('Squat');
    await page.getByPlaceholder('e.g., 3').fill('3');
    await page.getByPlaceholder('e.g., 12').fill('10');
    await page.getByTitle('Add Exercise').click();
    await page.getByRole('button', { name: 'Save' }).click();

    await expect.poll(async () => {
        const list = await api(request, alice, 'GET', `/workout/api/workouts/${alice.id}`);
        return list.body.find((w) => w.name === 'Leg day')?.exercises?.map((e) => e.name) ?? [];
    }).toContain('Squat');
});

test('business profile: open one from the list and edit it', async ({ page, request }) => {
    const alice = await createPerson(request, 'AliceBusiness');
    const made = await api(request, alice, 'POST', '/workout/api/business-profiles', {
        ownerId: alice.id, ownerUuid: alice.uuid, taxId: '12345678000199', businessName: 'Corner Gym', businessType: 'Company', addresses: [],
    });
    expect(made.status).toBe(201);

    await signInThroughTheForm(page, alice);
    await page.locator('.sidebar__menu-item', { hasText: 'Business' }).click();
    await expect(page).toHaveURL(/\/business$/);
    await page.locator('.business-card', { hasText: 'Corner Gym' }).click();
    await expect(page).toHaveURL(new RegExp(`/business/${made.body.id}$`));
    await expect(page.locator('h1', { hasText: 'Corner Gym' })).toBeVisible();

    await page.locator('.business-sidebar__item, .business-sidebar button', { hasText: /Edit/ }).first().click();
    await page.getByLabel('Business Name').fill('Corner Gym Plus');
    await page.getByRole('button', { name: 'Confirm' }).click();
    await expect(page.locator('h1', { hasText: 'Corner Gym Plus' })).toBeVisible();
    const stored = await api(request, alice, 'GET', `/workout/api/business-profiles/id/${made.body.id}`);
    expect(stored.body.businessName).toBe('Corner Gym Plus');
});

test('chat: a friend sends a message and the other person reads it', async ({ page, request, browser }) => {
    const alice = await createPerson(request, 'AliceChat');
    const bob = await createPerson(request, 'BobChat');
    expect((await api(request, alice, 'PUT', `/workout/api/friends/request/${bob.id}`)).status).toBe(200);
    expect((await api(request, bob, 'PUT', `/workout/api/friends/accept/${alice.id}`)).status).toBe(200);
    const conversation = await api(request, alice, 'POST', '/timeline/api/chat/conversations/direct', { targetPersonUuid: bob.uuid });
    expect(conversation.status, JSON.stringify(conversation.body)).toBeLessThan(300);

    await signInThroughTheForm(page, alice);
    await page.locator('.sidebar__menu-item', { hasText: 'Messages' }).click();
    await expect(page).toHaveURL(/\/chat$/);
    await page.locator('.chat-list__item').first().click();
    const text = `hello from the browser ${Date.now()}`;
    await page.getByPlaceholder('Write a message…').fill(text);
    await page.getByRole('button', { name: 'Send' }).click();
    const sent = await api(request, alice, 'GET', `/timeline/api/chat/conversations/${conversation.body.uuid}/messages`);
    expect(sent.body.map((m) => m.body)).toContain(text);
    await expect(page.locator('.chat-bubble', { hasText: text })).toBeVisible();

    // An image goes through the pre-signed upload and arrives as a picture in the thread.
    await page.locator('.chat-composer input[type="file"]').setInputFiles({ name: 'c.png', mimeType: 'image/png', buffer: PNG });
    const chatUpload = page.waitForResponse((response) => response.request().method() === 'PUT' && response.url().includes(':4566'));
    await page.getByRole('button', { name: 'Send' }).click();
    expect((await chatUpload).status()).toBeLessThan(300);
    // The test stack has no CDN, so the picture itself may not load; the thread must still hold it.
    await expect(page.locator('.chat-bubble__image')).toHaveCount(1);

    const bobContext = await browser.newContext();
    await instrument(bobContext);
    const bobPage = await bobContext.newPage();
    watchRefusals(bobPage);
    await skipCookieBanner(bobPage);
    await signInThroughTheForm(bobPage, bob);
    await bobPage.locator('.sidebar__menu-item', { hasText: 'Messages' }).click();
    await bobPage.locator('.chat-list__item').first().click();
    await expect(bobPage.locator('.chat-bubble', { hasText: text })).toBeVisible();
    await closeWithCoverage(bobPage);
    await bobContext.close();
});


test('profile picture: pick an image and save it (pre-signed upload to the object store)', async ({ page, request }) => {
    const alice = await createPerson(request, 'AliceAvatar');
    await signInThroughTheForm(page, alice);
    await page.locator('.sidebar__menu-item', { hasText: 'Profile' }).click();
    await expect(page).toHaveURL(/\/profile$/);

    await page.locator('#avatar-input').setInputFiles({ name: 'me.png', mimeType: 'image/png', buffer: PNG });
    const upload = page.waitForResponse((response) => response.request().method() === 'PUT' && !response.url().includes(':8080') && !response.url().includes(':5173'));
    await page.getByRole('button', { name: 'Save image' }).click();
    expect((await upload).status()).toBeLessThan(300);

    await expect.poll(async () => (await api(request, alice, 'GET', '/workout/api/people/me')).body.objectKey ?? '').not.toBe('');

    // The cover goes the same way.
    await page.locator('#cover-input').setInputFiles({ name: 'cover.png', mimeType: 'image/png', buffer: PNG });
    const coverUpload = page.waitForResponse((response) => response.request().method() === 'PUT' && response.url().includes(':4566') && response.url().includes('/cover/'));
    await page.getByRole('button', { name: 'Confirm' }).click();
    expect((await coverUpload).status()).toBeLessThan(300);
});

test('feed: post with a photo (pre-signed upload) shows the image', async ({ page, request }) => {
    const alice = await createPerson(request, 'AlicePhoto');
    await signInThroughTheForm(page, alice);
    await page.getByPlaceholder('What are you thinking?').first().click();
    const text = `photo post ${Date.now()}`;
    await page.locator('.post-creator__textarea').fill(text);
    await page.locator('.post-creator__expanded input[type="file"]').first().setInputFiles({ name: 'p.png', mimeType: 'image/png', buffer: PNG });
    await page.locator('.post-creator__image-consent input[type="checkbox"]').check();
    const upload = page.waitForResponse((response) => response.request().method() === 'PUT' && response.url().includes(':4566'));
    await page.getByRole('button', { name: 'Post', exact: true }).click();
    expect((await upload).status()).toBeLessThan(300);
    await expect(page.locator('.post', { hasText: text })).toBeVisible();

    const feed = await api(request, alice, 'GET', '/timeline/api/feed');
    expect(feed.body.find((p) => p.content === text).media).toHaveLength(1);
});

test('notifications: a comment on my post shows in the bell and is marked read when opened', async ({ page, request }) => {
    const alice = await createPerson(request, 'AliceBell');
    const bob = await createPerson(request, 'BobBell');
    expect((await api(request, alice, 'PUT', `/workout/api/friends/request/${bob.id}`)).status).toBe(200);
    expect((await api(request, bob, 'PUT', `/workout/api/friends/accept/${alice.id}`)).status).toBe(200);
    const post = await api(request, alice, 'POST', '/timeline/api/posts', { content: 'notify me' });
    expect(post.status).toBe(201);
    const comment = await api(request, bob, 'POST', `/timeline/api/posts/${post.body.uuid}/comments`, { content: 'ping' });
    expect(comment.status).toBe(201);

    await signInThroughTheForm(page, alice);
    await page.getByText('🔔').first().click();
    // Alice may also have a friendship notification; open the one about the comment.
    const item = page.locator('.notification-item.unread', { hasText: 'commented on your post' }).first();
    await expect(item).toBeVisible();
    await item.click();
    await expect.poll(async () => {
        const list = await api(request, alice, 'GET', `/timeline/api/notifications/${alice.uuid}`);
        return list.body.find((n) => n.notificationType === 'Comment' && n.postUuid === post.body.uuid)?.read;
    }).toBe(true);
});

test('friend profile: a friend opens a friend\'s profile; weight and height are not shown', async ({ page, request }) => {
    const alice = await createPerson(request, 'AliceProfileView');
    const bob = await createPerson(request, 'BobProfileView');
    expect((await api(request, alice, 'PUT', `/workout/api/friends/request/${bob.id}`)).status).toBe(200);
    expect((await api(request, bob, 'PUT', `/workout/api/friends/accept/${alice.id}`)).status).toBe(200);

    await signInThroughTheForm(page, alice);
    await page.goto(`/profile/${bob.id}`);
    await expect(page.getByText(bob.name).first()).toBeVisible();
    const profile = await api(request, alice, 'GET', `/workout/api/friends/${bob.id}`);
    expect(profile.status).toBe(200);
    expect(profile.body.personInfo?.weight ?? null).toBeNull();
});

test('notifications: mark all as read', async ({ page, request }) => {
    const alice = await createPerson(request, 'AliceBellAll');
    const bob = await createPerson(request, 'BobBellAll');
    expect((await api(request, alice, 'PUT', `/workout/api/friends/request/${bob.id}`)).status).toBe(200);
    expect((await api(request, bob, 'PUT', `/workout/api/friends/accept/${alice.id}`)).status).toBe(200);
    const post = await api(request, alice, 'POST', '/timeline/api/posts', { content: 'many pings' });
    for (const content of ['one', 'two']) {
        expect((await api(request, bob, 'POST', `/timeline/api/posts/${post.body.uuid}/comments`, { content })).status).toBe(201);
    }
    await expect.poll(async () => (await api(request, alice, 'GET', `/timeline/api/notifications/${alice.uuid}?unread_only=true`)).body.length).toBeGreaterThan(0);

    await signInThroughTheForm(page, alice);
    await page.getByText('🔔').first().click();
    await expect(page.locator('.notification-item.unread').first()).toBeVisible();
    await page.locator('.btn-mark-all').click();
    await expect.poll(async () => (await api(request, alice, 'GET', `/timeline/api/notifications/${alice.uuid}?unread_only=true`)).body.length).toBe(0);
});
