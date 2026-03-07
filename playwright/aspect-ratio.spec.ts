import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/aspect_ratio";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (aspect-ratio wrapper rendered). */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// AspectRatio – basic rendering
// ---------------------------------------------------------------------------

test.describe("aspect-ratio rendering", () => {
  test("wrapper is visible with correct data-slot", async ({ page }) => {
    await gotoAndWait(page);
    const wrapper = page.locator('[data-slot="preview"] [data-slot="aspect-ratio"]').first();
    await expect(wrapper).toBeVisible();
    await expect(wrapper).toHaveAttribute("data-slot", "aspect-ratio");
  });

  test("wrapper has data-radix-aspect-ratio-wrapper attribute", async ({ page }) => {
    await gotoAndWait(page);
    const wrapper = page.locator('[data-slot="preview"] [data-slot="aspect-ratio"]').first();
    await expect(wrapper).toHaveAttribute("data-radix-aspect-ratio-wrapper", "");
  });

  test("renders as a div element", async ({ page }) => {
    await gotoAndWait(page);
    const wrapper = page.locator('[data-slot="preview"] [data-slot="aspect-ratio"]').first();
    const tagName = await wrapper.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("div");
  });
});

// ---------------------------------------------------------------------------
// AspectRatio – ratio calculation
// ---------------------------------------------------------------------------

test.describe("aspect-ratio dimensions", () => {
  test("wrapper has padding-bottom for 4:3 ratio (75%)", async ({ page }) => {
    await gotoAndWait(page);
    const wrapper = page.locator('[data-slot="preview"] [data-slot="aspect-ratio"]').first();
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("padding-bottom: 75%");
  });

  test("inner div has absolute positioning", async ({ page }) => {
    await gotoAndWait(page);
    const wrapper = page.locator('[data-slot="preview"] [data-slot="aspect-ratio"]').first();
    const innerDiv = wrapper.locator(":scope > div").first();
    const style = await innerDiv.getAttribute("style");
    expect(style).toContain("position: absolute");
  });
});

// ---------------------------------------------------------------------------
// AspectRatio – accessibility
// ---------------------------------------------------------------------------

test.describe("aspect-ratio accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="preview"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
