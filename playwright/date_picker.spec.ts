import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/component/?name=date_picker&";
const LOAD_TIMEOUT = 20 * 60 * 1000;

test("data slots and classes", async ({ page }) => {
  await page.goto(URL, { timeout: LOAD_TIMEOUT });

  // Assert date-picker root data-slot
  const datePicker = page.locator('[data-slot="date-picker"]').first();
  await expect(datePicker).toBeVisible({ timeout: 30000 });

  const dpClass = await datePicker.getAttribute("class");
  expect(dpClass).toContain("inline-flex");
  expect(dpClass).toContain("items-center");

  // Assert date-picker-input data-slot
  const input = datePicker.locator('[data-slot="date-picker-input"]');
  await expect(input).toBeVisible();

  const inputClass = await input.getAttribute("class");
  expect(inputClass).toContain("flex");
  expect(inputClass).toContain("rounded-lg");
  expect(inputClass).toContain("border");

  // Assert date-picker-segment data-slots
  const segments = datePicker.locator('[data-slot="date-picker-segment"]');
  expect(await segments.count()).toBeGreaterThan(0);

  // Assert date-picker-separator data-slots
  const separators = datePicker.locator('[data-slot="date-picker-separator"]');
  expect(await separators.count()).toBeGreaterThan(0);
});

test("segment keyboard interaction", async ({ page }) => {
  await page.goto(URL, { timeout: LOAD_TIMEOUT });

  const datePicker = page.locator('[data-slot="date-picker"]').first();
  await expect(datePicker).toBeVisible({ timeout: 30000 });

  // Focus the first segment
  const segments = datePicker.locator('[data-slot="date-picker-segment"]');
  await segments.first().click();
  await expect(segments.first()).toBeFocused();

  // Tab to next segment
  await page.keyboard.press("Tab");
  await expect(segments.nth(1)).toBeFocused();
});
