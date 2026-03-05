import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=input&', { timeout: 20 * 60 * 1000 });

  const label = page.locator('label');
  await expect(label).toBeVisible();

  // data-slot attribute
  await expect(label).toHaveAttribute('data-slot', 'label');

  // Key Tailwind classes from shadcn
  const classAttr = await label.getAttribute('class');
  expect(classAttr).toContain('flex');
  expect(classAttr).toContain('text-sm');
  expect(classAttr).toContain('font-medium');
  expect(classAttr).toContain('select-none');

  // Label should be associated with the input
  await expect(label).toHaveAttribute('for', 'name');
  await expect(label).toHaveText('Name');
});
