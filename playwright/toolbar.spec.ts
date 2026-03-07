import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/toolbar", { timeout: 20 * 60 * 1000 });

  // Scope to the preview container
  const preview = page.locator('[data-slot="preview"]').first();
  await expect(preview).toBeVisible();

  // Toolbar root
  const toolbar = preview.locator('[data-slot="toolbar"]');
  await expect(toolbar).toBeVisible();
  await expect(toolbar).toHaveAttribute('role', 'toolbar');
  await expect(toolbar).toHaveAttribute('data-orientation', 'horizontal');

  // Separator
  const separator = toolbar.locator('[data-slot="toolbar-separator"]');
  await expect(separator).toBeVisible();
  await expect(separator).toHaveAttribute('data-orientation', 'vertical');

  // Toggle group items (6 total: 3 formatting + 3 alignment)
  const items = toolbar.locator('[data-slot="toggle-group-item"]');
  await expect(items).toHaveCount(6);

  // All start off except "Align Left" (default_value)
  const alignLeft = items.nth(3);
  await expect(alignLeft).toHaveAttribute('data-state', 'on');

  // Click Bold toggles it on
  const bold = items.nth(0);
  await bold.click();
  await expect(bold).toHaveAttribute('data-state', 'on');

  // Click Italic — both Bold and Italic on (multiple mode)
  const italic = items.nth(1);
  await italic.click();
  await expect(italic).toHaveAttribute('data-state', 'on');
  await expect(bold).toHaveAttribute('data-state', 'on');

  // Click Align Center — switches from Align Left (single mode)
  const alignCenter = items.nth(4);
  await alignCenter.click();
  await expect(alignCenter).toHaveAttribute('data-state', 'on');
  await expect(alignLeft).toHaveAttribute('data-state', 'off');

  // Keyboard navigation: ArrowRight moves through all toolbar items
  await bold.click();
  await page.keyboard.press("ArrowRight");
  await expect(italic).toBeFocused();

  await page.keyboard.press("ArrowRight");
  const underline = items.nth(2);
  await expect(underline).toBeFocused();
});
