import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/progress";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (progress bar rendered). */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="progress"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Progress – basic rendering
// ---------------------------------------------------------------------------

test.describe("progress rendering", () => {
  test("progress bar is visible with correct data-slot", async ({ page }) => {
    await gotoAndWait(page);
    const progress = page.locator('[data-slot="progress"]').first();
    await expect(progress).toBeVisible();
    await expect(progress).toHaveAttribute("data-slot", "progress");
  });

  test("renders as a div element", async ({ page }) => {
    await gotoAndWait(page);
    const progress = page.locator('[data-slot="progress"]').first();
    const tagName = await progress.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("div");
  });

  test("indicator sub-component is visible", async ({ page }) => {
    await gotoAndWait(page);
    const indicator = page.locator('[data-slot="progress-indicator"]').first();
    await expect(indicator).toBeVisible();
    await expect(indicator).toHaveAttribute("data-slot", "progress-indicator");
  });
});

// ---------------------------------------------------------------------------
// Progress – ARIA attributes
// ---------------------------------------------------------------------------

test.describe("progress ARIA attributes", () => {
  test("has progressbar role", async ({ page }) => {
    await gotoAndWait(page);
    const progress = page.locator('[data-slot="progress"]').first();
    await expect(progress).toHaveAttribute("role", "progressbar");
  });

  test("has aria-valuemin of 0", async ({ page }) => {
    await gotoAndWait(page);
    const progress = page.locator('[data-slot="progress"]').first();
    await expect(progress).toHaveAttribute("aria-valuemin", "0");
  });

  test("has aria-valuemax of 100", async ({ page }) => {
    await gotoAndWait(page);
    const progress = page.locator('[data-slot="progress"]').first();
    await expect(progress).toHaveAttribute("aria-valuemax", "100");
  });

  test("has aria-valuenow attribute", async ({ page }) => {
    await gotoAndWait(page);
    const progress = page.locator('[data-slot="progress"]').first();
    const value = await progress.getAttribute("aria-valuenow");
    expect(value).not.toBeNull();
    const numValue = Number(value);
    expect(numValue).toBeGreaterThanOrEqual(0);
    expect(numValue).toBeLessThanOrEqual(100);
  });
});

// ---------------------------------------------------------------------------
// Progress – data attributes
// ---------------------------------------------------------------------------

test.describe("progress data attributes", () => {
  test("has data-state attribute", async ({ page }) => {
    await gotoAndWait(page);
    const progress = page.locator('[data-slot="progress"]').first();
    const state = await progress.getAttribute("data-state");
    expect(state).not.toBeNull();
    // data-state should be "loading" or "complete"
    expect(["loading", "complete"]).toContain(state);
  });

  test("indicator has data-state attribute", async ({ page }) => {
    await gotoAndWait(page);
    const indicator = page.locator('[data-slot="progress-indicator"]').first();
    const state = await indicator.getAttribute("data-state");
    expect(state).not.toBeNull();
    expect(["loading", "complete"]).toContain(state);
  });

  test("indicator has data-value attribute", async ({ page }) => {
    await gotoAndWait(page);
    const indicator = page.locator('[data-slot="progress-indicator"]').first();
    const value = await indicator.getAttribute("data-value");
    expect(value).not.toBeNull();
    const numValue = Number(value);
    expect(numValue).toBeGreaterThanOrEqual(0);
    expect(numValue).toBeLessThanOrEqual(100);
  });

  test("indicator has data-max attribute", async ({ page }) => {
    await gotoAndWait(page);
    const indicator = page.locator('[data-slot="progress-indicator"]').first();
    await expect(indicator).toHaveAttribute("data-max", "100");
  });
});

// ---------------------------------------------------------------------------
// Progress – animation / value changes over time
// ---------------------------------------------------------------------------

test.describe("progress animation", () => {
  test("value changes over time as progress increments", async ({ page }) => {
    await gotoAndWait(page);
    const progress = page.locator('[data-slot="progress"]').first();

    // Capture the initial value
    const initialValue = Number(await progress.getAttribute("aria-valuenow"));

    // Wait for the value to change (the demo animates progress)
    await expect(async () => {
      const currentValue = Number(await progress.getAttribute("aria-valuenow"));
      expect(currentValue).not.toBe(initialValue);
    }).toPass({ timeout: 10_000 });
  });
});

// ---------------------------------------------------------------------------
// Progress – accessibility
// ---------------------------------------------------------------------------

test.describe("progress accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="progress"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
