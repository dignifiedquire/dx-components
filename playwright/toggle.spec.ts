import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=toggle&', { timeout: 20 * 60 * 1000 });

  const toggle = page.locator('[data-slot="toggle"]').first();
  await expect(toggle).toBeVisible();

  // data-slot attribute
  await expect(toggle).toHaveAttribute('data-slot', 'toggle');

  // data-variant and data-size defaults
  await expect(toggle).toHaveAttribute('data-variant', 'default');
  await expect(toggle).toHaveAttribute('data-size', 'default');

  // Key Tailwind classes
  const classAttr = await toggle.getAttribute('class');
  expect(classAttr).toContain('inline-flex');
  expect(classAttr).toContain('rounded-md');
  expect(classAttr).toContain('text-sm');
  expect(classAttr).toContain('font-medium');
  expect(classAttr).toContain('bg-transparent');

  // Toggle should be off initially
  await expect(toggle).toHaveAttribute('data-state', 'off');

  // Click toggles on
  await toggle.click();
  await expect(toggle).toHaveAttribute('data-state', 'on');

  // Space toggles off
  await page.keyboard.press('Space');
  await expect(toggle).toHaveAttribute('data-state', 'off');

  // Outline variant
  const outline = page.locator('[data-variant="outline"]');
  await expect(outline).toBeVisible();
  await expect(outline).toHaveAttribute('data-slot', 'toggle');
  const outlineClass = await outline.getAttribute('class');
  expect(outlineClass).toContain('border');
  expect(outlineClass).toContain('shadow-xs');

  // Small size
  const small = page.locator('[data-size="sm"]');
  await expect(small).toBeVisible();
  const smallClass = await small.getAttribute('class');
  expect(smallClass).toContain('h-8');
  expect(smallClass).toContain('min-w-8');

  // Large size
  const large = page.locator('[data-size="lg"]');
  await expect(large).toBeVisible();
  const largeClass = await large.getAttribute('class');
  expect(largeClass).toContain('h-10');
  expect(largeClass).toContain('min-w-10');
});
