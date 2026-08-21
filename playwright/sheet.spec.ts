import { test, expect } from '@playwright/test';

// Scope all locators to the component preview frame so they don't collide
// with the navbar's mobile-nav Sheet, which is also a native `<dialog>` and
// is always present in the DOM (Phase 2 of the top-layer port).
const inPreview = (selector: string) =>
  `[data-slot="preview"] ${selector}`;

/**
 * Close the sheet the way a user would.
 *
 * Sheet content is a native modal `<dialog>`, so ESC is the browser's own
 * cancel/close path and our `use_top_layer` hook only syncs the resulting
 * `close` event back into the open signal. Playwright's WebKit build on Linux
 * does not deliver a synthesized ESC to a modal dialog — the same limitation
 * `dialog.spec.ts:109` documents — so there we invoke `close()` directly,
 * which fires the identical `close` event and exercises the same code path.
 * Chromium and Firefox press the real key.
 */
async function closeWithEscape(
  page: import("@playwright/test").Page,
  browserName: string,
) {
  if (browserName === "webkit") {
    await page
      .locator(inPreview('[data-slot="sheet-content"]'))
      .evaluate((el) => (el as HTMLDialogElement).close());
    return;
  }
  await page.keyboard.press("Escape");
}

test('sheet basic interactions', async ({ page, browserName }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/sheet', { timeout: 20 * 60 * 1000 });
  // Wait for WASM hydration before interacting.
  await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });

  // Open sheet from Right button
  await page.getByRole('button', { name: 'Right' }).click();

  // Assert the sheet content is open
  const sheetContent = page.locator(inPreview('[data-slot="sheet-content"]'));
  await expect(sheetContent).toHaveAttribute('data-state', 'open');

  // Assert the first input is focused (focus trap)
  const nameInput = page.locator('#sheet-demo-name');
  await expect(nameInput).toBeFocused();

  // Tab through focusable elements
  await page.keyboard.press('Tab');
  const usernameInput = page.locator('#sheet-demo-username');
  await expect(usernameInput).toBeFocused();

  // Playwright's WebKit build leaves buttons out of sequential Tab
  // navigation (Safari's default keyboard-navigation behaviour: Tab visits
  // form fields only), so the footer buttons are reachable by Tab on the
  // other engines only. Everything below this block runs on all three.
  if (browserName !== 'webkit') {
    await page.keyboard.press('Tab');
    const saveButton = page.getByRole('button', { name: 'Save changes' });
    await expect(saveButton).toBeFocused();

    await page.keyboard.press('Tab');
    const cancelButton = page.getByRole('button', { name: 'Cancel' });
    await expect(cancelButton).toBeFocused();
  }

  // Hitting escape should close the sheet
  await closeWithEscape(page, browserName);
  await expect(sheetContent).toHaveAttribute('data-state', 'closed');

  // Reopen the sheet
  await page.getByRole('button', { name: 'Right' }).click();
  await expect(sheetContent).toHaveAttribute('data-state', 'open');

  // Click Cancel to close
  await page.getByRole('button', { name: 'Cancel' }).click();
  await expect(sheetContent).toHaveAttribute('data-state', 'closed');
});

test('sheet opens from different sides', async ({ page, browserName }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/sheet', { timeout: 20 * 60 * 1000 });
  // Wait for WASM hydration before interacting.
  await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });

  const sheetContent = page.locator(inPreview('[data-slot="sheet-content"]'));

  // Test Top
  await page.getByRole('button', { name: 'Top' }).click();
  await expect(sheetContent).toHaveAttribute('data-state', 'open');
  await expect(sheetContent).toHaveAttribute('data-side', 'top');
  await closeWithEscape(page, browserName);
  await expect(sheetContent).toHaveAttribute('data-state', 'closed');

  // Test Bottom
  await page.getByRole('button', { name: 'Bottom' }).click();
  await expect(sheetContent).toHaveAttribute('data-state', 'open');
  await expect(sheetContent).toHaveAttribute('data-side', 'bottom');
  await closeWithEscape(page, browserName);
  await expect(sheetContent).toHaveAttribute('data-state', 'closed');

  // Test Left
  await page.getByRole('button', { name: 'Left' }).click();
  await expect(sheetContent).toHaveAttribute('data-state', 'open');
  await expect(sheetContent).toHaveAttribute('data-side', 'left');
  await closeWithEscape(page, browserName);
  await expect(sheetContent).toHaveAttribute('data-state', 'closed');
});
