import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/visually_hidden";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="visually-hidden-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Upstream: visually-hidden.stories.tsx — "Basic"
// Button with hidden text and visible icon
// ---------------------------------------------------------------------------

test.describe("VisuallyHidden: basic", () => {
  test("button is visible", async ({ page }) => {
    await gotoAndWait(page);
    const button = page.locator('[data-testid="save-button"]');
    await expect(button).toBeVisible();
  });

  test("visible icon is rendered", async ({ page }) => {
    await gotoAndWait(page);
    const icon = page.locator('[data-testid="visible-icon"]');
    await expect(icon).toBeVisible();
  });

  test("hidden text exists in the DOM", async ({ page }) => {
    await gotoAndWait(page);
    const hidden = page.locator('[data-testid="hidden-text"]');
    // The element exists but is not visually visible (clipped to 1x1)
    await expect(hidden).toBeAttached();
    await expect(hidden).toContainText("Save the file");
  });

  test("button accessible name includes hidden text", async ({ page }) => {
    await gotoAndWait(page);
    const button = page.locator('[data-testid="save-button"]');
    // The button's text content should include the hidden text
    const text = await button.textContent();
    expect(text).toContain("Save the file");
  });
});

// ---------------------------------------------------------------------------
// VisuallyHidden styles verification
// ---------------------------------------------------------------------------

test.describe("VisuallyHidden: styles", () => {
  test("has position: absolute", async ({ page }) => {
    await gotoAndWait(page);
    const span = page.locator('[data-testid="hidden-span"]');
    const position = await span.evaluate(
      (el) => window.getComputedStyle(el).position
    );
    expect(position).toBe("absolute");
  });

  test("has 1x1 pixel dimensions", async ({ page }) => {
    await gotoAndWait(page);
    const span = page.locator('[data-testid="hidden-span"]');
    const box = await span.boundingBox();
    // Clipped to 1px x 1px
    expect(box).toBeTruthy();
    expect(box!.width).toBeLessThanOrEqual(1);
    expect(box!.height).toBeLessThanOrEqual(1);
  });

  test("has overflow: hidden", async ({ page }) => {
    await gotoAndWait(page);
    const span = page.locator('[data-testid="hidden-span"]');
    const overflow = await span.evaluate(
      (el) => window.getComputedStyle(el).overflow
    );
    expect(overflow).toBe("hidden");
  });

  test("renders a span element", async ({ page }) => {
    await gotoAndWait(page);
    const span = page.locator('[data-testid="hidden-span"]');
    const tagName = await span.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("span");
  });

  test("has data-slot attribute", async ({ page }) => {
    await gotoAndWait(page);
    const span = page.locator('[data-testid="hidden-span"]');
    await expect(span).toHaveAttribute("data-slot", "visually-hidden");
  });
});
