import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/aspect_ratio";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="aspect-ratio-demo"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Default (16/9)
// ---------------------------------------------------------------------------

test.describe("AspectRatio: default (16/9)", () => {
  test("wrapper has data-radix-aspect-ratio-wrapper attribute", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-demo"]');
    const wrapper = section.locator('[data-radix-aspect-ratio-wrapper=""]');
    await expect(wrapper).toBeAttached();
  });

  test("wrapper has padding-bottom for 16/9 ratio (56.25%)", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-demo"]');
    const wrapper = section.locator('[data-radix-aspect-ratio-wrapper=""]');
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("padding-bottom: 56.25%");
  });

  test("wrapper has relative positioning", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-demo"]');
    const wrapper = section.locator('[data-radix-aspect-ratio-wrapper=""]');
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("position: relative");
  });

  test("inner div has data-slot=aspect-ratio", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-demo"]');
    const inner = section.locator('[data-slot="aspect-ratio"]');
    await expect(inner).toBeAttached();
  });

  test("inner div has absolute positioning", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-demo"]');
    const inner = section.locator('[data-slot="aspect-ratio"]');
    const style = await inner.getAttribute("style");
    expect(style).toContain("position: absolute");
  });

  test("renders image inside", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-demo"]');
    const img = section.locator("img");
    await expect(img).toBeAttached();
    await expect(img).toHaveAttribute("alt", "Photo");
  });

  test("inner div receives class prop", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-demo"]');
    const inner = section.locator('[data-slot="aspect-ratio"]');
    const className = await inner.getAttribute("class");
    expect(className).toContain("rounded-lg");
    expect(className).toContain("bg-muted");
  });
});

// ---------------------------------------------------------------------------
// Square (1/1)
// ---------------------------------------------------------------------------

test.describe("AspectRatio: square (1/1)", () => {
  test("wrapper has padding-bottom 100%", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-square"]');
    const wrapper = section.locator('[data-radix-aspect-ratio-wrapper=""]');
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("padding-bottom: 100%");
  });

  test("inner div has data-slot=aspect-ratio", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-square"]');
    const inner = section.locator('[data-slot="aspect-ratio"]');
    await expect(inner).toBeAttached();
  });

  test("renders image inside", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-square"]');
    const img = section.locator("img");
    await expect(img).toBeAttached();
    await expect(img).toHaveAttribute("alt", "Photo");
  });

  test("image has grayscale class", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-square"]');
    const img = section.locator("img");
    const className = await img.getAttribute("class");
    expect(className).toContain("grayscale");
  });
});

// ---------------------------------------------------------------------------
// Portrait (9/16)
// ---------------------------------------------------------------------------

test.describe("AspectRatio: portrait (9/16)", () => {
  test("wrapper has correct padding-bottom for 9/16 ratio", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-portrait"]');
    const wrapper = section.locator('[data-radix-aspect-ratio-wrapper=""]');
    const style = await wrapper.getAttribute("style");
    // 100 / (9/16) = 177.777...%
    expect(style).toMatch(/padding-bottom: 177\.7/);
  });

  test("inner div has data-slot=aspect-ratio", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-portrait"]');
    const inner = section.locator('[data-slot="aspect-ratio"]');
    await expect(inner).toBeAttached();
  });

  test("renders image inside", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-portrait"]');
    const img = section.locator("img");
    await expect(img).toBeAttached();
    await expect(img).toHaveAttribute("alt", "Photo");
  });

  test("image has grayscale class", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="aspect-ratio-portrait"]');
    const img = section.locator("img");
    const className = await img.getAttribute("class");
    expect(className).toContain("grayscale");
  });
});

// ---------------------------------------------------------------------------
// Accessibility (upstream: aspect-ratio.test.tsx)
// ---------------------------------------------------------------------------

test.describe("AspectRatio: accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-testid="aspect-ratio-demo"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
