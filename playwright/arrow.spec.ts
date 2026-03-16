import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/arrow";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="arrow-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Upstream: arrow.stories.tsx — "Styled"
// ---------------------------------------------------------------------------

test.describe("Arrow: styled", () => {
  test("renders an SVG element", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="styled-arrow"]');
    await expect(arrow).toBeVisible();
    const tagName = await arrow.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("svg");
  });

  test("has correct width and height", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="styled-arrow"]');
    await expect(arrow).toHaveAttribute("width", "20");
    await expect(arrow).toHaveAttribute("height", "10");
  });

  test("has viewBox attribute", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="styled-arrow"]');
    await expect(arrow).toHaveAttribute("viewBox", "0 0 30 10");
  });

  test("has preserveAspectRatio attribute", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="styled-arrow"]');
    await expect(arrow).toHaveAttribute("preserveAspectRatio", "none");
  });

  test("contains a path child with triangle points", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="styled-arrow"]');
    // Dioxus renders SVG path (not polygon) for the default triangle shape
    const pathEl = arrow.locator("path");
    await expect(pathEl).toHaveAttribute("d", "M0,0 L30,0 L15,10 Z");
  });
});

// ---------------------------------------------------------------------------
// Upstream: arrow.stories.tsx — "CustomSizes"
// ---------------------------------------------------------------------------

test.describe("Arrow: custom sizes", () => {
  test("40x10 arrow has correct dimensions", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="arrow-40x10"]');
    await expect(arrow).toHaveAttribute("width", "40");
    await expect(arrow).toHaveAttribute("height", "10");
  });

  test("50x30 arrow has correct dimensions", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="arrow-50x30"]');
    await expect(arrow).toHaveAttribute("width", "50");
    await expect(arrow).toHaveAttribute("height", "30");
  });

  test("20x100 arrow has correct dimensions", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="arrow-20x100"]');
    await expect(arrow).toHaveAttribute("width", "20");
    await expect(arrow).toHaveAttribute("height", "100");
  });
});

// ---------------------------------------------------------------------------
// Upstream: arrow.test.tsx — "given a default Arrow"
// Tests width/height attributes with defaults (10, 5)
// ---------------------------------------------------------------------------

test.describe("Arrow: default size", () => {
  test("has default width of 10", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="arrow-default"]');
    await expect(arrow).toHaveAttribute("width", "10");
  });

  test("has default height of 5", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="arrow-default"]');
    await expect(arrow).toHaveAttribute("height", "5");
  });
});

// ---------------------------------------------------------------------------
// Upstream: arrow.stories.tsx — "CustomArrow"
// Children replace the default polygon
// ---------------------------------------------------------------------------

test.describe("Arrow: custom children", () => {
  test("renders custom children instead of polygon", async ({ page }) => {
    await gotoAndWait(page);
    const arrow = page.locator('[data-testid="arrow-custom"]');

    // Should have the custom rect
    const rect = arrow.locator('[data-testid="custom-rect"]');
    await expect(rect).toBeVisible();

    // Should NOT have the default polygon
    const polygon = arrow.locator("polygon");
    await expect(polygon).not.toBeVisible();
  });
});
