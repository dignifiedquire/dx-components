import { test, expect } from '@playwright/test';

test('hover navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=navbar&', { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const navbar = page.locator('[data-slot="navbar"]');
  await expect(navbar).toBeVisible();

  const triggers = page.locator('[data-slot="navbar-trigger"]');
  await expect(triggers).toHaveCount(2);

  const triggerClass = await triggers.first().getAttribute('class');
  expect(triggerClass).toContain('flex');
  expect(triggerClass).toContain('rounded-sm');
  expect(triggerClass).toContain('text-sm');
  expect(triggerClass).toContain('font-medium');

  // Hover Inputs trigger
  await page.getByRole('menuitem', { name: 'Inputs' }).hover();
  const content = page.locator('[data-slot="navbar-content"]').first();
  await expect(content).toBeVisible();

  const contentClass = await content.getAttribute('class');
  expect(contentClass).toContain('z-50');
  expect(contentClass).toContain('rounded-md');
  expect(contentClass).toContain('border');
  expect(contentClass).toContain('bg-popover');

  // Click Calendar
  const calendar = page.getByRole('menuitem', { name: 'Calendar' });
  await calendar.hover();
  await calendar.click();
  await expect(page).toHaveURL(/.*name=calendar/);
});

test('mobile navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=navbar&', { timeout: 20 * 60 * 1000 });
  await page.getByRole('menuitem', { name: 'Inputs' }).tap();
  await page.getByRole('menuitem', { name: 'Calendar' }).tap();
  await expect(page).toHaveURL(/.*name=calendar/);
});

test('keyboard navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=navbar&', { timeout: 20 * 60 * 1000 });

  // Focus via menubar role
  await page.locator('[role="menubar"]').focus();

  await page.keyboard.press('ArrowRight');
  await expect(page.getByRole('menuitem', { name: 'Information' })).toBeFocused();
  await page.keyboard.press('ArrowLeft');
  await expect(page.getByRole('menuitem', { name: 'Inputs' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('menuitem', { name: 'Calendar' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/.*name=calendar/);
});
