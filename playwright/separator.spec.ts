import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/separator";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (separator rendered). */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Separator – basic rendering
// ---------------------------------------------------------------------------

test.describe("separator rendering", () => {
  test("separator is visible with correct data-slot", async ({ page }) => {
    await gotoAndWait(page);
    const separator = page.locator('[data-slot="preview"] [data-slot="separator"]').first();
    await expect(separator).toBeVisible();
    await expect(separator).toHaveAttribute("data-slot", "separator");
  });

  test("renders as a div element", async ({ page }) => {
    await gotoAndWait(page);
    const separator = page.locator('[data-slot="preview"] [data-slot="separator"]').first();
    const tagName = await separator.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("div");
  });

  test("has horizontal orientation by default", async ({ page }) => {
    await gotoAndWait(page);
    const separator = page.locator('[data-slot="preview"] [data-slot="separator"]').first();
    await expect(separator).toHaveAttribute("data-orientation", "horizontal");
  });
});

// ---------------------------------------------------------------------------
// Separator – ARIA / role attributes
// ---------------------------------------------------------------------------

test.describe("separator ARIA attributes", () => {
  test("decorative separator has role none", async ({ page }) => {
    await gotoAndWait(page);
    const separator = page.locator('[data-slot="preview"] [data-slot="separator"]').first();
    // The demo uses decorative: true, so the role should be "none"
    await expect(separator).toHaveAttribute("role", "none");
  });
});

// ---------------------------------------------------------------------------
// Separator – accessibility
// ---------------------------------------------------------------------------

test.describe("separator accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="preview"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
