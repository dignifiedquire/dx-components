import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=dialog&variant=main&', { timeout: 20 * 60 * 1000 });
  await page.getByRole('button', { name: 'Show Dialog' }).click();

  // data-slot assertions
  const overlay = page.locator('[data-slot="dialog-overlay"]');
  await expect(overlay).toHaveAttribute('data-state', 'open');

  const overlayClass = await overlay.getAttribute('class');
  expect(overlayClass).toContain('fixed');
  expect(overlayClass).toContain('inset-0');
  expect(overlayClass).toContain('z-50');
  expect(overlayClass).toContain('bg-black/50');

  const content = overlay.locator('[data-slot="dialog-content"]');
  await expect(content).toBeVisible();

  const contentClass = await content.getAttribute('class');
  expect(contentClass).toContain('fixed');
  expect(contentClass).toContain('rounded-lg');
  expect(contentClass).toContain('border');
  expect(contentClass).toContain('bg-background');
  expect(contentClass).toContain('shadow-lg');

  const title = content.locator('[data-slot="dialog-title"]');
  await expect(title).toHaveText('Item information');

  const description = content.locator('[data-slot="dialog-description"]');
  await expect(description).toHaveText('Here is some additional information about the item.');

  // Focus trap: close button should be focused
  const closeButton = content.getByRole('button');
  await expect(closeButton).toBeFocused();

  // Tab should keep focus within dialog
  await page.keyboard.press('Tab');
  await expect(closeButton).toBeFocused();

  // Escape should close the dialog
  await page.keyboard.press('Escape');
  await expect(overlay).toHaveCount(0);

  // Reopen and test click close
  await page.getByRole('button', { name: 'Show Dialog' }).click();
  await expect(overlay).toHaveAttribute('data-state', 'open');
  await closeButton.click();
  await expect(overlay).toHaveCount(0);
});
