import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=separator&', { timeout: 20 * 60 * 1000 });

  const separator = page.locator('[data-slot="separator"]').first();
  await expect(separator).toBeVisible();

  // data-slot attribute
  await expect(separator).toHaveAttribute('data-slot', 'separator');

  // data-orientation attribute
  await expect(separator).toHaveAttribute('data-orientation', 'horizontal');

  // Key Tailwind classes from shadcn
  const classAttr = await separator.getAttribute('class');
  expect(classAttr).toContain('shrink-0');
  expect(classAttr).toContain('bg-border');
});
