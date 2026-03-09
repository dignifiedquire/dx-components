import { test, expect } from '@playwright/test';

test('badge variants render with correct data attributes and classes', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/badge', { timeout: 20 * 60 * 1000 });

  // All badges visible
  await expect(page.getByText('Default', { exact: true })).toBeVisible();
  await expect(page.getByText('Secondary', { exact: true })).toBeVisible();
  await expect(page.getByText('Destructive', { exact: true })).toBeVisible();
  await expect(page.getByText('Outline', { exact: true })).toBeVisible();
  await expect(page.getByText('Verified')).toBeVisible();

  // data-slot on all badges
  const badges = page.locator('[data-slot="badge"]');
  await expect(badges).toHaveCount(5);

  // data-variant attributes
  const defaultBadge = page.locator('[data-slot="badge"]', { hasText: 'Default' });
  await expect(defaultBadge).toHaveAttribute('data-variant', 'default');

  const secondaryBadge = page.locator('[data-slot="badge"]', { hasText: 'Secondary' }).first();
  await expect(secondaryBadge).toHaveAttribute('data-variant', 'secondary');

  const destructiveBadge = page.locator('[data-slot="badge"]', { hasText: 'Destructive' });
  await expect(destructiveBadge).toHaveAttribute('data-variant', 'destructive');

  const outlineBadge = page.locator('[data-slot="badge"]', { hasText: 'Outline' });
  await expect(outlineBadge).toHaveAttribute('data-variant', 'outline');

  // Badges render as <span>
  await expect(defaultBadge).toHaveAttribute('data-slot', 'badge');
  const tagName = await defaultBadge.evaluate(el => el.tagName.toLowerCase());
  expect(tagName).toBe('span');

  // Base classes present
  await expect(defaultBadge).toHaveClass(/rounded-full/);
  await expect(defaultBadge).toHaveClass(/text-xs/);
  await expect(defaultBadge).toHaveClass(/font-medium/);
});
