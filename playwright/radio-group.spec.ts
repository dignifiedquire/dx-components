import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=radio_group&", { timeout: 20 * 60 * 1000 });

  // Radio group root
  const group = page.locator('[data-slot="radio-group"]');
  await expect(group).toBeVisible();
  await expect(group).toHaveAttribute('data-slot', 'radio-group');

  const groupClass = await group.getAttribute('class');
  expect(groupClass).toContain('grid');
  expect(groupClass).toContain('gap-3');

  // Radio items
  const items = group.locator('[data-slot="radio-group-item"]');
  await expect(items).toHaveCount(3);

  const firstItem = items.first();
  const itemClass = await firstItem.getAttribute('class');
  expect(itemClass).toContain('rounded-full');
  expect(itemClass).toContain('border');
  expect(itemClass).toContain('border-input');

  // Click first item selects it
  await firstItem.click();
  await expect(firstItem).toHaveAttribute('data-state', 'checked');

  // Keyboard navigation: ArrowDown moves to Red (skips disabled Green)
  await page.keyboard.press('ArrowDown');
  const secondItem = items.nth(1);
  await expect(secondItem).toBeFocused();

  // ArrowDown wraps to Blue (Green is disabled)
  await page.keyboard.press('ArrowDown');
  await expect(firstItem).toBeFocused();
});
