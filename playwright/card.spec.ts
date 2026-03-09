import { test, expect } from '@playwright/test';

test('card renders with correct data-slot attributes and structure', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/card', { timeout: 20 * 60 * 1000 });

  // Card root
  const card = page.locator('[data-slot="card"]');
  await expect(card).toBeVisible();
  await expect(card).toHaveClass(/rounded-xl/);
  await expect(card).toHaveClass(/border/);
  await expect(card).toHaveClass(/shadow-sm/);

  // Card header
  const header = page.locator('[data-slot="card-header"]');
  await expect(header).toBeVisible();

  // Card title
  const title = page.locator('[data-slot="card-title"]');
  await expect(title).toBeVisible();
  await expect(title).toContainText('Login to your account');

  // Card description
  const description = page.locator('[data-slot="card-description"]');
  await expect(description).toBeVisible();
  await expect(description).toHaveClass(/text-muted-foreground/);

  // Card action
  const action = page.locator('[data-slot="card-action"]');
  await expect(action).toBeVisible();

  // Card content
  const content = page.locator('[data-slot="card-content"]');
  await expect(content).toBeVisible();

  // Card footer
  const footer = page.locator('[data-slot="card-footer"]');
  await expect(footer).toBeVisible();

  // Input fields in card
  await expect(page.locator('[data-slot="input"]')).toHaveCount(2);
});
