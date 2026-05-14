import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/accessible_icon";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="accessible-icon-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Upstream: accessible-icon.stories.tsx — "Styled"
// ---------------------------------------------------------------------------

test.describe("AccessibleIcon: styled", () => {
  test("icon is wrapped in an aria-hidden span", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    // The icon wrapper has aria-hidden="true" so screen readers skip the
    // graphic and announce the visually-hidden label instead. This is the
    // Dioxus equivalent of Radix's cloneElement(aria-hidden=true) on the
    // SVG — we wrap because VNodes are immutable.
    const ariaHidden = section.locator('[aria-hidden="true"]');
    await expect(ariaHidden).toBeAttached();
    // The SVG lives inside that wrapper.
    await expect(ariaHidden.locator('[data-testid="icon-svg"]')).toBeAttached();
  });

  test("SVG is rendered inside the icon", async ({ page }) => {
    await gotoAndWait(page);
    const svg = page.locator('[data-testid="icon-svg"]');
    await expect(svg).toBeAttached();
    const tagName = await svg.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("svg");
  });

  test("SVG has correct dimensions", async ({ page }) => {
    await gotoAndWait(page);
    const svg = page.locator('[data-testid="icon-svg"]');
    await expect(svg).toHaveAttribute("width", "24");
    await expect(svg).toHaveAttribute("height", "24");
  });
});

// ---------------------------------------------------------------------------
// Upstream: accesible-icon.test.tsx — label and accessibility
// ---------------------------------------------------------------------------

test.describe("AccessibleIcon: accessibility", () => {
  test("visually hidden label is in the DOM", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const label = section.locator('[data-slot="visually-hidden"]');
    await expect(label).toBeAttached();
    await expect(label).toContainText("Close");
  });

  test("visually hidden label is not visible", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const label = section.locator('[data-slot="visually-hidden"]');
    // Visually hidden elements have 1x1 pixel size
    const box = await label.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeLessThanOrEqual(1);
    expect(box!.height).toBeLessThanOrEqual(1);
  });

  test("button has accessible name from label", async ({ page }) => {
    await gotoAndWait(page);
    const button = page.locator('[data-testid="icon-button"]');
    // The button's accessible name should include "Close" from the visually hidden label
    const name = await button.evaluate(
      (el) => el.textContent?.trim() || "",
    );
    expect(name).toContain("Close");
  });
});

// ---------------------------------------------------------------------------
// Upstream: accessible-icon.stories.tsx — "Chromatic"
// ---------------------------------------------------------------------------

test.describe("AccessibleIcon: inline (chromatic)", () => {
  test("inline icon renders within text", async ({ page }) => {
    await gotoAndWait(page);
    const text = page.locator('[data-testid="inline-text"]');
    await expect(text).toContainText("Some text with an inline accessible icon");
  });

  test("inline SVG is rendered", async ({ page }) => {
    await gotoAndWait(page);
    const svg = page.locator('[data-testid="inline-svg"]');
    await expect(svg).toBeAttached();
  });

  test("inline icon has visually hidden label", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="chromatic"]');
    const label = section.locator('[data-slot="visually-hidden"]');
    await expect(label).toContainText("Close");
  });
});
