import { test, expect } from '@playwright/test';

test('skeleton renders with correct data-slot and animation classes', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/skeleton', { timeout: 20 * 60 * 1000 });

  // Multiple skeleton elements rendered
  const skeletons = page.locator('[data-slot="skeleton"]');
  await expect(skeletons.first()).toBeVisible();
  const count = await skeletons.count();
  expect(count).toBeGreaterThanOrEqual(5); // 3 in info demo + 3 in card demo

  // Base classes on all skeletons
  for (let i = 0; i < count; i++) {
    const skeleton = skeletons.nth(i);
    await expect(skeleton).toHaveClass(/animate-pulse/);
    await expect(skeleton).toHaveClass(/bg-accent/);
  }

  // Renders as <div>
  const tagName = await skeletons.first().evaluate(el => el.tagName.toLowerCase());
  expect(tagName).toBe('div');
});
