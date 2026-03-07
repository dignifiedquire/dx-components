import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/label";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (label rendered). */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Label – basic rendering
// ---------------------------------------------------------------------------

test.describe("label rendering", () => {
  test("label is visible with correct data-slot", async ({ page }) => {
    await gotoAndWait(page);
    const label = page.locator('[data-slot="preview"] [data-slot="label"]').first();
    await expect(label).toBeVisible();
    await expect(label).toHaveAttribute("data-slot", "label");
  });

  test("renders as a label element", async ({ page }) => {
    await gotoAndWait(page);
    const label = page.locator('[data-slot="preview"] [data-slot="label"]').first();
    const tagName = await label.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("label");
  });

  test("displays correct text content", async ({ page }) => {
    await gotoAndWait(page);
    const label = page.locator('[data-slot="preview"] [data-slot="label"]').first();
    await expect(label).toHaveText("Name");
  });
});

// ---------------------------------------------------------------------------
// Label – association with input
// ---------------------------------------------------------------------------

test.describe("label association", () => {
  test("has for attribute pointing to input", async ({ page }) => {
    await gotoAndWait(page);
    const label = page.locator('[data-slot="preview"] [data-slot="label"]').first();
    await expect(label).toHaveAttribute("for", "name");
  });

  test("associated input exists", async ({ page }) => {
    await gotoAndWait(page);
    const input = page.locator("#name");
    await expect(input).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Label – accessibility
// ---------------------------------------------------------------------------

test.describe("label accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="preview"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
