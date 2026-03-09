import { test, expect } from "@playwright/test";

test("data slots and classes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/radio_group", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  await expect(preview).toBeVisible();

  // RadioGroup root
  const group = preview.locator('[data-slot="radio-group"]');
  await expect(group).toBeVisible();
  const groupClass = await group.getAttribute("class");
  expect(groupClass).toContain("grid");
  expect(groupClass).toContain("gap-3");

  // RadioGroupItem data-slot and classes
  const items = group.locator('[data-slot="radio-group-item"]');
  await expect(items).toHaveCount(3);
  const itemClass = await items.first().getAttribute("class");
  expect(itemClass).toContain("rounded-full");
  expect(itemClass).toContain("border");
  expect(itemClass).toContain("border-input");
  expect(itemClass).toContain("size-4");

  // RadioGroupIndicator data-slot and classes
  const indicators = group.locator('[data-slot="radio-group-indicator"]');
  await expect(indicators).toHaveCount(3);
  const indicatorClass = await indicators.first().getAttribute("class");
  expect(indicatorClass).toContain("flex");
  expect(indicatorClass).toContain("items-center");
  expect(indicatorClass).toContain("justify-center");

  // SVG circle icon inside indicator (lucide IconCircle)
  const checkedItem = items.nth(1); // "comfortable" is default
  const svg = checkedItem.locator("svg");
  await expect(svg).toBeVisible();
});

test("click selection", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/radio_group", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  const group = preview.locator('[data-slot="radio-group"]');
  const items = group.locator('[data-slot="radio-group-item"]');

  const defaultItem = items.nth(0);
  const comfortableItem = items.nth(1);
  const compactItem = items.nth(2);

  // "Comfortable" is selected by default
  await expect(comfortableItem).toHaveAttribute("data-state", "checked");
  await expect(comfortableItem).toHaveAttribute("aria-checked", "true");
  await expect(defaultItem).toHaveAttribute("data-state", "unchecked");

  // Click "Default" selects it
  await defaultItem.click();
  await expect(defaultItem).toHaveAttribute("data-state", "checked");
  await expect(comfortableItem).toHaveAttribute("data-state", "unchecked");

  // Click "Compact" selects it
  await compactItem.click();
  await expect(compactItem).toHaveAttribute("data-state", "checked");
  await expect(defaultItem).toHaveAttribute("data-state", "unchecked");
});

test("keyboard navigation", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/radio_group", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  const group = preview.locator('[data-slot="radio-group"]');
  const items = group.locator('[data-slot="radio-group-item"]');

  const defaultItem = items.nth(0);
  const comfortableItem = items.nth(1);
  const compactItem = items.nth(2);

  // Focus on compact (click it first)
  await compactItem.click();

  // ArrowDown moves to default (loops)
  await page.keyboard.press("ArrowDown");
  await expect(defaultItem).toBeFocused();

  // ArrowDown to comfortable
  await page.keyboard.press("ArrowDown");
  await expect(comfortableItem).toBeFocused();

  // ArrowDown loops to compact
  await page.keyboard.press("ArrowDown");
  await expect(compactItem).toBeFocused();

  // ArrowUp goes back
  await page.keyboard.press("ArrowUp");
  await expect(comfortableItem).toBeFocused();
});

test("accessibility attributes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/radio_group", { timeout: 20 * 60 * 1000 });

  const preview = page.locator('[data-slot="preview"]').first();
  const group = preview.locator('[data-slot="radio-group"]');

  // Root has role="radiogroup"
  await expect(group).toHaveAttribute("role", "radiogroup");
  await expect(group).toHaveAttribute("data-orientation", "vertical");

  // Items have role="radio"
  const items = group.locator('[data-slot="radio-group-item"]');
  await expect(items.first()).toHaveAttribute("role", "radio");

  // Checked item has aria-checked="true"
  const comfortableItem = items.nth(1);
  await expect(comfortableItem).toHaveAttribute("aria-checked", "true");

  // Unchecked items have aria-checked="false"
  await expect(items.first()).toHaveAttribute("aria-checked", "false");
});
