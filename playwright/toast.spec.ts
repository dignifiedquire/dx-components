import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/toast', { timeout: 20 * 60 * 1000 });

  // Create a toast
  await page.getByRole('button', { name: 'Info (60s)' }).click();

  // Assert viewport data-slot
  const viewport = page.locator('[data-slot="toast-viewport"]');
  await expect(viewport).toBeVisible();

  await expect(viewport).toHaveAttribute('data-slot', 'toast-viewport');

  // Assert toast data-slot
  const toast = page.locator('[data-slot="toast"]');
  await expect(toast).toBeVisible();
  await expect(toast).toHaveAttribute('data-slot', 'toast');

  // Assert sub-component data-slots
  await expect(page.locator('[data-slot="toast-title"]')).toBeVisible();
  await expect(page.locator('[data-slot="toast-description"]')).toBeVisible();
  await expect(page.locator('[data-slot="toast-close"]')).toBeVisible();

  // Create another toast
  await page.getByRole('button', { name: 'Info (60s)' }).click();
  const closeButtons = page.locator('[data-slot="toast-close"]');
  await expect(closeButtons).toHaveCount(2);

  // Hover and close the first toast
  await closeButtons.first().hover();
  await closeButtons.first().click();
  await expect(closeButtons).toHaveCount(1);

  // Hover and close the second toast
  await closeButtons.first().hover();
  await closeButtons.first().click();
  await expect(closeButtons).toHaveCount(0);
});
