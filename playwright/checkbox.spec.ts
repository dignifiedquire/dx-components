import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=checkbox&', { timeout: 20 * 60 * 1000 });

  const checkbox = page.locator('[data-slot="checkbox"]');
  await expect(checkbox).toBeVisible();

  // data-slot attribute
  await expect(checkbox).toHaveAttribute('data-slot', 'checkbox');

  // Key Tailwind classes
  const classAttr = await checkbox.getAttribute('class');
  expect(classAttr).toContain('shrink-0');
  expect(classAttr).toContain('rounded-[4px]');
  expect(classAttr).toContain('border');
  expect(classAttr).toContain('border-input');

  // Indicator has data-slot
  const indicator = checkbox.locator('[data-slot="checkbox-indicator"]');
  await expect(indicator).toBeVisible();
  const indicatorClass = await indicator.getAttribute('class');
  expect(indicatorClass).toContain('grid');
  expect(indicatorClass).toContain('place-content-center');

  // Checkbox should be unchecked initially
  await expect(checkbox).toHaveAttribute('data-state', 'unchecked');

  // Click toggles to checked
  await checkbox.click();
  await expect(checkbox).toHaveAttribute('data-state', 'checked');

  // Space toggles back
  await page.keyboard.press('Space');
  await expect(checkbox).toHaveAttribute('data-state', 'unchecked');
});
