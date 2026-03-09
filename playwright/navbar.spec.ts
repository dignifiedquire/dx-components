import { test, expect } from '@playwright/test';

test('hover navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/navbar', { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const navbar = page.locator('[data-slot="navbar"]');
  await expect(navbar).toBeVisible();

  const triggers = page.locator('[data-slot="navbar-trigger"]');
  await expect(triggers).toHaveCount(2);

  // Verify data-slot attributes are present
  await expect(triggers.first()).toHaveAttribute('data-slot', 'navbar-trigger');

  // Hover Inputs trigger
  await page.getByRole('menuitem', { name: 'Inputs' }).hover();
  const content = page.locator('[data-slot="navbar-content"]').first();
  await expect(content).toBeVisible();

  // Verify content data-slot
  await expect(content).toHaveAttribute('data-slot', 'navbar-content');

  // Click Calendar
  const calendar = page.getByRole('menuitem', { name: 'Calendar' });
  await calendar.hover();
  await calendar.click();
  await expect(page).toHaveURL(/.*components\/calendar/);
});

test('mobile navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/navbar', { timeout: 20 * 60 * 1000 });
  await page.getByRole('menuitem', { name: 'Inputs' }).tap();
  await page.getByRole('menuitem', { name: 'Calendar' }).tap();
  await expect(page).toHaveURL(/.*components\/calendar/);
});

test('keyboard navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/navbar', { timeout: 20 * 60 * 1000 });

  // Focus via menubar role
  await page.locator('[role="menubar"]').focus();

  await page.keyboard.press('ArrowRight');
  await expect(page.getByRole('menuitem', { name: 'Information' })).toBeFocused();
  await page.keyboard.press('ArrowLeft');
  await expect(page.getByRole('menuitem', { name: 'Inputs' })).toBeFocused();
  await page.keyboard.press('ArrowDown');
  await expect(page.getByRole('menuitem', { name: 'Calendar' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/.*components\/calendar/);
});
