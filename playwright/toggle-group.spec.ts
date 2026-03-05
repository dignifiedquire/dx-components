import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=toggle_group&', { timeout: 20 * 60 * 1000 });

  // Toggle group root
  const group = page.locator('[data-slot="toggle-group"]');
  await expect(group).toBeVisible();
  await expect(group).toHaveAttribute('data-slot', 'toggle-group');
  await expect(group).toHaveAttribute('data-variant', 'default');
  await expect(group).toHaveAttribute('data-size', 'default');
  await expect(group).toHaveAttribute('data-orientation', 'horizontal');

  // Key Tailwind classes on group
  const groupClass = await group.getAttribute('class');
  expect(groupClass).toContain('flex');
  expect(groupClass).toContain('items-center');
  expect(groupClass).toContain('rounded-md');

  // Toggle items (rendered as toggle buttons via Toggle primitive)
  const items = group.locator('[data-slot="toggle"]');
  await expect(items).toHaveCount(3);

  // Items inherit variant/size from group
  const firstItem = items.first();
  await expect(firstItem).toHaveAttribute('data-variant', 'default');
  await expect(firstItem).toHaveAttribute('data-size', 'default');

  // Item Tailwind classes include group overrides
  const itemClass = await firstItem.getAttribute('class');
  expect(itemClass).toContain('shrink-0');
  expect(itemClass).toContain('rounded-none');
  expect(itemClass).toContain('inline-flex');

  // Toggle behavior: items start off
  await expect(firstItem).toHaveAttribute('data-state', 'off');

  // Click toggles on
  await firstItem.click();
  await expect(firstItem).toHaveAttribute('data-state', 'on');

  // Keyboard navigation: ArrowRight moves focus
  await page.keyboard.press('ArrowRight');
  const secondItem = items.nth(1);
  await expect(secondItem).toBeFocused();

  // Space toggles second item
  await page.keyboard.press('Space');
  await expect(secondItem).toHaveAttribute('data-state', 'on');
});
