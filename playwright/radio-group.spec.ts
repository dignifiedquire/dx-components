import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/radio_group", { timeout: 20 * 60 * 1000 });

  // Scope to the preview container
  const preview = page.locator('[data-slot="preview"]').first();
  await expect(preview).toBeVisible();

  // RadioGroup root
  const group = preview.locator('[data-slot="radio-group"]');
  await expect(group).toBeVisible();
  await expect(group).toHaveAttribute('role', 'radiogroup');
  await expect(group).toHaveAttribute('data-orientation', 'vertical');

  // Radio items
  const items = group.locator('[data-slot="radio-group-item"]');
  await expect(items).toHaveCount(3);

  const defaultItem = items.nth(0);
  const comfortableItem = items.nth(1);
  const compactItem = items.nth(2);

  // "Comfortable" is selected by default (default_value)
  await expect(comfortableItem).toHaveAttribute('data-state', 'checked');
  await expect(comfortableItem).toHaveAttribute('aria-checked', 'true');
  await expect(defaultItem).toHaveAttribute('data-state', 'unchecked');
  await expect(compactItem).toHaveAttribute('data-state', 'unchecked');

  // Click "Default" selects it, deselects "Comfortable"
  await defaultItem.click();
  await expect(defaultItem).toHaveAttribute('data-state', 'checked');
  await expect(defaultItem).toHaveAttribute('aria-checked', 'true');
  await expect(comfortableItem).toHaveAttribute('data-state', 'unchecked');

  // Click "Compact" selects it
  await compactItem.click();
  await expect(compactItem).toHaveAttribute('data-state', 'checked');
  await expect(defaultItem).toHaveAttribute('data-state', 'unchecked');

  // Keyboard navigation: ArrowDown moves focus
  await compactItem.click();
  await page.keyboard.press("ArrowDown");
  await expect(defaultItem).toBeFocused();

  // ArrowDown again moves to Comfortable
  await page.keyboard.press("ArrowDown");
  await expect(comfortableItem).toBeFocused();

  // ArrowDown loops back to Compact
  await page.keyboard.press("ArrowDown");
  await expect(compactItem).toBeFocused();

  // ArrowUp goes back to Comfortable
  await page.keyboard.press("ArrowUp");
  await expect(comfortableItem).toBeFocused();

  // Indicators
  const indicators = group.locator('[data-slot="radio-group-indicator"]');
  await expect(indicators).toHaveCount(3);
});
