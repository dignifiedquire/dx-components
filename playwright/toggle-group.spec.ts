import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/toggle_group", { timeout: 20 * 60 * 1000 });

  // Scope to the preview container
  const preview = page.locator('[data-slot="preview"]').first();
  await expect(preview).toBeVisible();

  // Toggle group root (the first demo is "multiple" mode)
  const group = preview.locator('[data-slot="toggle-group"]');
  await expect(group).toBeVisible();
  await expect(group).toHaveAttribute('role', 'group');
  await expect(group).toHaveAttribute('data-orientation', 'horizontal');

  // Toggle items
  const items = group.locator('[data-slot="toggle-group-item"]');
  await expect(items).toHaveCount(3);

  const boldItem = items.nth(0);
  const italicItem = items.nth(1);
  const underlineItem = items.nth(2);

  // Items start off
  await expect(boldItem).toHaveAttribute('data-state', 'off');
  await expect(italicItem).toHaveAttribute('data-state', 'off');

  // Click toggles on (multiple mode — items toggle independently)
  await boldItem.click();
  await expect(boldItem).toHaveAttribute('data-state', 'on');
  await expect(boldItem).toHaveAttribute('aria-pressed', 'true');

  // Click another — both can be on
  await italicItem.click();
  await expect(italicItem).toHaveAttribute('data-state', 'on');
  await expect(boldItem).toHaveAttribute('data-state', 'on');

  // Click again toggles off
  await boldItem.click();
  await expect(boldItem).toHaveAttribute('data-state', 'off');
  await expect(italicItem).toHaveAttribute('data-state', 'on');

  // Keyboard navigation: ArrowRight moves focus
  await italicItem.click();
  await page.keyboard.press("ArrowRight");
  await expect(underlineItem).toBeFocused();

  // ArrowRight loops back
  await page.keyboard.press("ArrowRight");
  await expect(boldItem).toBeFocused();

  // ArrowLeft goes back
  await page.keyboard.press("ArrowLeft");
  await expect(underlineItem).toBeFocused();
});
