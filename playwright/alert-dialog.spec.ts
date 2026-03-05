import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=alert_dialog&variant=main&', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Show Alert Dialog' }).click();

  // data-slot assertions
  const overlay = page.locator('[data-slot="alert-dialog-overlay"]');
  await expect(overlay).toHaveAttribute('data-state', 'open');

  const overlayClass = await overlay.getAttribute('class');
  expect(overlayClass).toContain('fixed');
  expect(overlayClass).toContain('inset-0');
  expect(overlayClass).toContain('z-50');

  const content = overlay.locator('[data-slot="alert-dialog-content"]');
  await expect(content).toBeVisible();

  const contentClass = await content.getAttribute('class');
  expect(contentClass).toContain('fixed');
  expect(contentClass).toContain('rounded-lg');
  expect(contentClass).toContain('border');
  expect(contentClass).toContain('bg-background');

  const title = content.locator('[data-slot="alert-dialog-title"]');
  await expect(title).toHaveText('Delete item');

  const description = content.locator('[data-slot="alert-dialog-description"]');
  await expect(description).toContainText('Are you sure');

  const actions = content.locator('[data-slot="alert-dialog-actions"]');
  await expect(actions).toBeVisible();

  // Cancel button should be focused
  const cancelButton = page.getByRole('button', { name: 'Cancel' });
  await expect(cancelButton).toBeFocused();

  // Tab cycles within dialog
  await page.keyboard.press('Tab');
  await page.keyboard.press('Tab');
  await expect(cancelButton).toBeFocused();

  // Escape should close
  await page.keyboard.press('Escape');
  await expect(overlay).toHaveCount(0);

  // Reopen and test confirm action
  await page.getByRole('button', { name: 'Show Alert Dialog' }).click();
  await expect(overlay).toHaveAttribute('data-state', 'open');
  const confirmButton = page.getByRole('button', { name: 'Delete' });
  await confirmButton.click();
  await expect(overlay).toHaveCount(0);
});
