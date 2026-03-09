import { test, expect } from "@playwright/test";

test("data slots and classes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/toggle_group", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  await expect(preview).toBeVisible();

  // ToggleGroup root
  const group = preview.locator('[data-slot="toggle-group"]');
  await expect(group).toBeVisible();
  const groupClass = await group.getAttribute("class");
  expect(groupClass).toContain("flex");
  expect(groupClass).toContain("items-center");
  expect(groupClass).toContain("rounded-md");

  // ToggleGroupItem data-slot and classes
  const items = group.locator('[data-slot="toggle-group-item"]');
  await expect(items).toHaveCount(3);
  const itemClass = await items.first().getAttribute("class");
  expect(itemClass).toContain("inline-flex");
  expect(itemClass).toContain("items-center");
  expect(itemClass).toContain("justify-center");
  expect(itemClass).toContain("text-sm");
  expect(itemClass).toContain("font-medium");
});

test("multiple mode toggle", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/toggle_group", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  const group = preview.locator('[data-slot="toggle-group"]');
  const items = group.locator('[data-slot="toggle-group-item"]');

  const boldItem = items.nth(0);
  const italicItem = items.nth(1);

  // Items start off
  await expect(boldItem).toHaveAttribute("data-state", "off");
  await expect(italicItem).toHaveAttribute("data-state", "off");

  // Click toggles on (multiple mode)
  await boldItem.click();
  await expect(boldItem).toHaveAttribute("data-state", "on");
  await expect(boldItem).toHaveAttribute("aria-pressed", "true");

  // Both can be on
  await italicItem.click();
  await expect(italicItem).toHaveAttribute("data-state", "on");
  await expect(boldItem).toHaveAttribute("data-state", "on");

  // Click again toggles off
  await boldItem.click();
  await expect(boldItem).toHaveAttribute("data-state", "off");
  await expect(italicItem).toHaveAttribute("data-state", "on");
});

test("keyboard navigation", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/toggle_group", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  const group = preview.locator('[data-slot="toggle-group"]');
  const items = group.locator('[data-slot="toggle-group-item"]');

  const boldItem = items.nth(0);
  const italicItem = items.nth(1);
  const underlineItem = items.nth(2);

  await italicItem.click();

  // ArrowRight moves to underline
  await page.keyboard.press("ArrowRight");
  await expect(underlineItem).toBeFocused();

  // ArrowRight loops back
  await page.keyboard.press("ArrowRight");
  await expect(boldItem).toBeFocused();

  // ArrowLeft goes back
  await page.keyboard.press("ArrowLeft");
  await expect(underlineItem).toBeFocused();
});

test("accessibility attributes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/toggle_group", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  const group = preview.locator('[data-slot="toggle-group"]');

  // Multiple mode: role="group", items use aria-pressed
  await expect(group).toHaveAttribute("role", "group");
  await expect(group).toHaveAttribute("data-orientation", "horizontal");

  const items = group.locator('[data-slot="toggle-group-item"]');
  await expect(items.first()).toHaveAttribute("aria-pressed", "false");
});
