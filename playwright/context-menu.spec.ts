import { test, expect } from '@playwright/test';

test('pointer navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const menu = page.locator('[data-slot="context-menu"]');
  await expect(menu).toBeVisible();

  const trigger = menu.locator('[data-slot="context-menu-trigger"]');
  await expect(trigger).toBeVisible();

  await trigger.click({ button: 'right' });

  const content = page.locator('[data-slot="context-menu-content"]');
  await expect(content).toHaveAttribute('data-state', 'open');

  // Assert key Tailwind classes on content
  const contentClass = await content.getAttribute('class');
  expect(contentClass).toContain('z-50');
  expect(contentClass).toContain('rounded-md');
  expect(contentClass).toContain('border');
  expect(contentClass).toContain('bg-popover');
  expect(contentClass).toContain('shadow-md');

  // Assert items have data-slot and key classes
  const items = content.locator('[data-slot="context-menu-item"]');
  await expect(items).toHaveCount(4);

  const itemClass = await items.first().getAttribute('class');
  expect(itemClass).toContain('flex');
  expect(itemClass).toContain('cursor-default');
  expect(itemClass).toContain('rounded-sm');
  expect(itemClass).toContain('text-sm');

  // Click on the "Edit" menu item
  await page.getByRole('menuitem', { name: 'Edit' }).click();
  // Assert the context menu is closed after clicking
  await expect(content).toHaveCount(0);
});

test('keyboard navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=context_menu&', { timeout: 20 * 60 * 1000 });

  const trigger = page.locator('[data-slot="context-menu-trigger"]');
  await trigger.click({ button: 'right' });

  const content = page.locator('[data-slot="context-menu-content"]');
  await expect(content).toHaveAttribute('data-state', 'open');

  // Hit escape to close the context menu
  await page.keyboard.press('Escape');
  await expect(content).toHaveCount(0);

  // Reopen the context menu
  await trigger.click({ button: 'right' });
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('menuitem', { name: 'Edit' })).toBeFocused();

  // Move down to the "Duplicate" menu item (skipping disabled "Undo")
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('menuitem', { name: 'Duplicate' })).toBeFocused();

  // Hit Enter to select
  await page.keyboard.press('Enter');
  await expect(content).toHaveCount(0);
  await expect(page.getByText('Selected: Duplicate')).toBeVisible();
});
