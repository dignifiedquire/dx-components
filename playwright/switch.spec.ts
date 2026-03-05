import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=switch&', { timeout: 20 * 60 * 1000 });

  const switchEl = page.locator('[data-slot="switch"]');
  await expect(switchEl).toBeVisible();

  // data-slot attribute
  await expect(switchEl).toHaveAttribute('data-slot', 'switch');

  // Key Tailwind classes
  const classAttr = await switchEl.getAttribute('class');
  expect(classAttr).toContain('inline-flex');
  expect(classAttr).toContain('rounded-full');
  expect(classAttr).toContain('shrink-0');

  // Thumb has data-slot
  const thumb = switchEl.locator('[data-slot="switch-thumb"]');
  await expect(thumb).toBeVisible();
  const thumbClass = await thumb.getAttribute('class');
  expect(thumbClass).toContain('rounded-full');
  expect(thumbClass).toContain('bg-background');

  // Switch should be unchecked initially
  await expect(switchEl).toHaveAttribute('data-state', 'unchecked');

  // Click toggles to checked
  await switchEl.click();
  await expect(switchEl).toHaveAttribute('data-state', 'checked');

  // Space toggles back
  await page.keyboard.press('Space');
  await expect(switchEl).toHaveAttribute('data-state', 'unchecked');
});
