import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=aspect_ratio&', { timeout: 20 * 60 * 1000 });

  const aspectRatio = page.locator('[data-slot="aspect-ratio"]');
  await expect(aspectRatio).toBeVisible();

  // data-slot attribute
  await expect(aspectRatio).toHaveAttribute('data-slot', 'aspect-ratio');
});
