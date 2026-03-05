import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=dropdown_menu&variant=main&', { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const menu = page.locator('[data-slot="dropdown-menu"]');
  await expect(menu).toBeVisible();

  const trigger = menu.locator('[data-slot="dropdown-menu-trigger"]');
  await expect(trigger).toBeVisible();
  await expect(trigger).toHaveAttribute('data-state', 'closed');

  // Open menu
  await trigger.click();
  await expect(trigger).toHaveAttribute('data-state', 'open');

  const content = page.locator('[data-slot="dropdown-menu-content"]');
  await expect(content).toBeVisible();

  const contentClass = await content.getAttribute('class');
  expect(contentClass).toContain('z-50');
  expect(contentClass).toContain('rounded-md');
  expect(contentClass).toContain('border');
  expect(contentClass).toContain('bg-popover');
  expect(contentClass).toContain('shadow-md');

  const items = content.locator('[data-slot="dropdown-menu-item"]');
  await expect(items).toHaveCount(4);

  const itemClass = await items.first().getAttribute('class');
  expect(itemClass).toContain('flex');
  expect(itemClass).toContain('cursor-default');
  expect(itemClass).toContain('rounded-sm');
  expect(itemClass).toContain('text-sm');

  // Keyboard navigation
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('option', { name: 'Edit' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('option', { name: 'Undo' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('option', { name: 'Duplicate' })).toBeFocused();

  // Select item
  await page.keyboard.press('Enter');
  await expect(trigger).toHaveAttribute('data-state', 'closed');
  await expect(page.getByText('Selected: Duplicate')).toBeVisible();

  // Escape closes menu
  await trigger.click();
  await expect(trigger).toHaveAttribute('data-state', 'open');
  await page.keyboard.press('Escape');
  await expect(trigger).toHaveAttribute('data-state', 'closed');

  // Click outside closes menu
  await trigger.click();
  await expect(trigger).toHaveAttribute('data-state', 'open');
  await page.locator('body').click({ position: { x: 0, y: 0 } });
  await expect(trigger).toHaveAttribute('data-state', 'closed');
});
