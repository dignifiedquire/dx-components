import { test, expect } from '@playwright/test';

test('badge variants render with correct data attributes and classes', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/badge', { timeout: 20 * 60 * 1000 });

  // Scope to the first preview block to avoid strict mode violations
  const preview = page.locator('[data-slot="preview"]').first();

  // All badges visible within preview
  await expect(preview.getByText('Default', { exact: true })).toBeVisible();
  await expect(preview.getByText('Secondary', { exact: true })).toBeVisible();
  await expect(preview.getByText('Destructive', { exact: true })).toBeVisible();
  await expect(preview.getByText('Outline', { exact: true })).toBeVisible();
  await expect(preview.getByText('Verified')).toBeVisible();

  // data-slot on all badges in preview
  const badges = preview.locator('[data-slot="badge"]');
  await expect(badges).toHaveCount(5);

  // data-variant attributes
  const defaultBadge = preview.locator('[data-slot="badge"]', { hasText: 'Default' });
  await expect(defaultBadge).toHaveAttribute('data-variant', 'default');

  const secondaryBadge = preview.locator('[data-slot="badge"]', { hasText: 'Secondary' }).first();
  await expect(secondaryBadge).toHaveAttribute('data-variant', 'secondary');

  const destructiveBadge = preview.locator('[data-slot="badge"]', { hasText: 'Destructive' });
  await expect(destructiveBadge).toHaveAttribute('data-variant', 'destructive');

  const outlineBadge = preview.locator('[data-slot="badge"]', { hasText: 'Outline' });
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
