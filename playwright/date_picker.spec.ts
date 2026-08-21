import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/date_picker";
const LOAD_TIMEOUT = 20 * 60 * 1000;

test("data slots and classes", async ({ page }) => {
  await page.goto(URL, { timeout: LOAD_TIMEOUT });
  // Wait for WASM hydration before interacting.
  await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });

  // Assert date-picker root data-slot
  const datePicker = page.locator('[data-slot="date-picker"]').first();
  await expect(datePicker).toBeVisible({ timeout: 30000 });

  await expect(datePicker).toHaveAttribute('data-slot', 'date-picker');

  // Assert date-picker-input data-slot
  const input = datePicker.locator('[data-slot="date-picker-input"]');
  await expect(input).toBeVisible();
  await expect(input).toHaveAttribute('data-slot', 'date-picker-input');

  // Assert date-picker-segment data-slots
  const segments = datePicker.locator('[data-slot="date-picker-segment"]');
  expect(await segments.count()).toBeGreaterThan(0);

  // Assert date-picker-separator data-slots
  const separators = datePicker.locator('[data-slot="date-picker-separator"]');
  expect(await separators.count()).toBeGreaterThan(0);
});

test("segment keyboard interaction", async ({ page }) => {
  await page.goto(URL, { timeout: LOAD_TIMEOUT });
  // Wait for WASM hydration before interacting.
  await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });

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

// ---------------------------------------------------------------------------
// Popover behaviour
//
// The date picker's calendar lives in a Popover whose content id is forwarded
// through `DatePickerPopoverContent`. That id is load-bearing: Presence watches
// the element by id for exit animations, and the dismissable layer resolves its
// own element by id to tell inside interactions from outside ones. When the id
// was dropped, every click inside the calendar read as an outside click.
// ---------------------------------------------------------------------------

test.describe("date picker popover", () => {
  async function openCalendar(page: import("@playwright/test").Page) {
    await page.goto("http://127.0.0.1:8080/docs/components/date_picker", {
      timeout: 20 * 60 * 1000,
    });
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    const trigger = page
      .locator('[data-slot="date-picker"] [data-slot="popover-trigger"]')
      .first();
    await trigger.click();
    const content = page.locator('[data-slot="popover-content"]').first();
    await expect(content).toBeVisible();
    return { trigger, content };
  }

  test("content keeps a non-empty id", async ({ page }) => {
    const { content } = await openCalendar(page);
    const id = await content.getAttribute("id");
    expect(id).toBeTruthy();
  });

  test("clicking inside the calendar does not close it", async ({ page }) => {
    const { content } = await openCalendar(page);
    // Click the calendar's own padding — inside the popover, no control under
    // the pointer, so nothing but the dismissal logic can react.
    const box = await content.boundingBox();
    expect(box).not.toBeNull();
    await page.mouse.click(box!.x + box!.width / 2, box!.y + 3);
    await expect(content).toBeVisible();
  });

  test("Escape closes the calendar", async ({ page }) => {
    const { content } = await openCalendar(page);
    await page.keyboard.press("Escape");
    await expect(content).toHaveCount(0);
  });
});
