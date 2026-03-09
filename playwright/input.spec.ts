import { test, expect } from '@playwright/test';

test('input renders with correct data-slot and classes', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/input', { timeout: 20 * 60 * 1000 });

  const input = page.locator('[data-slot="input"]').first();
  await expect(input).toBeVisible();

  // data-slot
  await expect(input).toHaveAttribute('data-slot', 'input');

  // Base classes
  await expect(input).toHaveClass(/rounded-md/);
  await expect(input).toHaveClass(/border/);
  await expect(input).toHaveClass(/bg-transparent/);
  await expect(input).toHaveClass(/h-9/);

  // Renders as <input>
  const tagName = await input.evaluate(el => el.tagName.toLowerCase());
  expect(tagName).toBe('input');
});

test('input accepts user input', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/input', { timeout: 20 * 60 * 1000 });

  await page.getByRole('textbox', { name: 'Enter your name' }).fill('name');
  await expect(page.locator('#input-greeting')).toContainText('Hello, name!');
});
