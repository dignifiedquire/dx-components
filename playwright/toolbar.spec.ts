import { test, expect } from "@playwright/test";

test("data slots and classes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/toolbar", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  await expect(preview).toBeVisible();

  // Toolbar root
  const toolbar = preview.locator('[data-slot="toolbar"]');
  await expect(toolbar).toBeVisible();
  const toolbarClass = await toolbar.getAttribute("class");
  expect(toolbarClass).toContain("flex");
  expect(toolbarClass).toContain("items-center");
  expect(toolbarClass).toContain("rounded-md");
  expect(toolbarClass).toContain("border");
  expect(toolbarClass).toContain("bg-background");

  // Separator data-slot and classes
  const separator = toolbar.locator('[data-slot="toolbar-separator"]');
  await expect(separator).toBeVisible();
  const sepClass = await separator.getAttribute("class");
  expect(sepClass).toContain("bg-border");

  // Toggle group items
  const items = toolbar.locator('[data-slot="toggle-group-item"]');
  await expect(items).toHaveCount(6);
});

test("toggle interactions", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/toolbar", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  const toolbar = preview.locator('[data-slot="toolbar"]');
  const items = toolbar.locator('[data-slot="toggle-group-item"]');

  // "Align Left" is default selected (single mode group)
  const alignLeft = items.nth(3);
  await expect(alignLeft).toHaveAttribute("data-state", "on");

  // Click Bold toggles it on (multiple mode group)
  const bold = items.nth(0);
  await bold.click();
  await expect(bold).toHaveAttribute("data-state", "on");

  // Click Italic — both on (multiple mode)
  const italic = items.nth(1);
  await italic.click();
  await expect(italic).toHaveAttribute("data-state", "on");
  await expect(bold).toHaveAttribute("data-state", "on");

  // Click Align Center — switches from Align Left (single mode)
  const alignCenter = items.nth(4);
  await alignCenter.click();
  await expect(alignCenter).toHaveAttribute("data-state", "on");
  await expect(alignLeft).toHaveAttribute("data-state", "off");
});

test("accessibility attributes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/toolbar", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  const toolbar = preview.locator('[data-slot="toolbar"]');

  // Root has role="toolbar"
  await expect(toolbar).toHaveAttribute("role", "toolbar");
  await expect(toolbar).toHaveAttribute("data-orientation", "horizontal");

  // Separator has role="separator" or aria-orientation
  const separator = toolbar.locator('[data-slot="toolbar-separator"]');
  await expect(separator).toHaveAttribute("data-orientation", "vertical");
});
