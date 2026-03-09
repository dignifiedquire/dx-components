import { test, expect } from '@playwright/test';

test('textarea renders with correct data-slot and classes', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/textarea', { timeout: 20 * 60 * 1000 });

  const textarea = page.locator('[data-slot="textarea"]');
  await expect(textarea).toBeVisible();

  // data-slot
  await expect(textarea).toHaveAttribute('data-slot', 'textarea');

  // Base classes
  await expect(textarea).toHaveClass(/rounded-md/);
  await expect(textarea).toHaveClass(/border/);
  await expect(textarea).toHaveClass(/bg-transparent/);

  // Renders as <textarea>
  const tagName = await textarea.evaluate(el => el.tagName.toLowerCase());
  expect(tagName).toBe('textarea');
});

test('textarea accepts input', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/textarea', { timeout: 20 * 60 * 1000 });

  const textarea = page.locator('[data-slot="textarea"]').first();
  await textarea.fill('Hello World');
  await expect(page.locator('#textarea-message')).toContainText('Hello World');
});
