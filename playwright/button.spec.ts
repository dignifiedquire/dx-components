import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=button&', { timeout: 20 * 60 * 1000 });

  // All variant buttons should be visible
  await expect(page.getByRole('button', { name: 'Default' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Secondary' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Destructive' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Outline' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Ghost' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Link' })).toBeVisible();

  // Variant data attributes
  await expect(page.getByRole('button', { name: 'Default' })).toHaveAttribute('data-variant', 'default');
  await expect(page.getByRole('button', { name: 'Secondary' })).toHaveAttribute('data-variant', 'secondary');
  await expect(page.getByRole('button', { name: 'Destructive' })).toHaveAttribute('data-variant', 'destructive');
  await expect(page.getByRole('button', { name: 'Outline' })).toHaveAttribute('data-variant', 'outline');
  await expect(page.getByRole('button', { name: 'Ghost' })).toHaveAttribute('data-variant', 'ghost');
  await expect(page.getByRole('button', { name: 'Link' })).toHaveAttribute('data-variant', 'link');

  // Size data attributes
  await expect(page.getByRole('button', { name: 'Default' })).toHaveAttribute('data-size', 'default');
  await expect(page.getByRole('button', { name: 'Small' })).toHaveAttribute('data-size', 'sm');
  await expect(page.getByRole('button', { name: 'Large' })).toHaveAttribute('data-size', 'lg');

  // data-slot attribute
  await expect(page.getByRole('button', { name: 'Default' })).toHaveAttribute('data-slot', 'button');

  // Buttons should be clickable and focusable
  let defaultBtn = page.getByRole('button', { name: 'Default' });
  await defaultBtn.click();
  await expect(defaultBtn).toBeVisible();
  await defaultBtn.focus();
  await expect(defaultBtn).toBeFocused();
});
