import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/label";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="label-demo"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Label – basic rendering
// ---------------------------------------------------------------------------

test.describe("Label: rendering", () => {
  test("label is visible with correct data-slot", async ({ page }) => {
    await gotoAndWait(page);
    const label = page.locator('[data-testid="label-demo"] [data-slot="label"]');
    await expect(label).toBeVisible();
    await expect(label).toHaveAttribute("data-slot", "label");
  });

  test("renders as a label element", async ({ page }) => {
    await gotoAndWait(page);
    const label = page.locator('[data-testid="label-demo"] [data-slot="label"]');
    const tagName = await label.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("label");
  });

  test("displays correct text content", async ({ page }) => {
    await gotoAndWait(page);
    const label = page.locator('[data-testid="label-demo"] [data-slot="label"]');
    await expect(label).toHaveText("Accept terms and conditions");
  });
});

// ---------------------------------------------------------------------------
// Label – association with control (upstream: WithControl story)
// ---------------------------------------------------------------------------

test.describe("Label: association", () => {
  test("has for attribute pointing to checkbox", async ({ page }) => {
    await gotoAndWait(page);
    const label = page.locator('[data-testid="label-demo"] [data-slot="label"]');
    await expect(label).toHaveAttribute("for", "terms");
  });

  test("associated checkbox exists", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-testid="label-demo"] #terms');
    await expect(checkbox).toBeAttached();
  });
});

// ---------------------------------------------------------------------------
// Label – double-click text selection prevention (upstream: onMouseDown)
// ---------------------------------------------------------------------------

test.describe("Label: double-click prevention", () => {
  test("double-click on label does not select text", async ({ page }) => {
    await gotoAndWait(page);
    const label = page.locator('[data-testid="label-demo"] [data-slot="label"]');
    // Double-click the label
    await label.dblclick();
    // Check that no text is selected
    const selection = await page.evaluate(() => window.getSelection()?.toString() ?? "");
    expect(selection).toBe("");
  });
});

// ---------------------------------------------------------------------------
// Label – accessibility
// ---------------------------------------------------------------------------

test.describe("Label: accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-testid="label-demo"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
