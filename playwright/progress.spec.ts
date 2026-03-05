import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=progress&', { timeout: 20 * 60 * 1000 });

  const progress = page.getByRole('progressbar');
  await expect(progress).toBeVisible();

  // data-slot attribute
  await expect(progress).toHaveAttribute('data-slot', 'progress');

  // data-state attribute
  await expect(progress).toHaveAttribute('data-state', 'loading');

  // Key Tailwind classes from shadcn
  const classAttr = await progress.getAttribute('class');
  expect(classAttr).toContain('overflow-hidden');
  expect(classAttr).toContain('rounded-full');
  expect(classAttr).toContain('bg-primary/20');

  // Indicator data-slot
  const indicator = page.locator('[data-slot="progress-indicator"]');
  await expect(indicator).toBeVisible();

  const indicatorClass = await indicator.getAttribute('class');
  expect(indicatorClass).toContain('bg-primary');
  expect(indicatorClass).toContain('transition-all');
});
