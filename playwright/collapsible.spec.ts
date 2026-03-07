import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/collapsible";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Basic rendering
// ---------------------------------------------------------------------------

test.describe("collapsible rendering", () => {
  test("renders collapsible root", async ({ page }) => {
    await gotoAndWait(page);
    const collapsible = page.locator('[data-slot="preview"] [data-slot="collapsible"]');
    await expect(collapsible.first()).toBeVisible();
  });

  test("renders trigger as a button", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-slot="preview"] [data-slot="collapsible-trigger"]');
    await expect(trigger.first()).toBeVisible();
    const tagName = await trigger.first().evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("button");
  });

  test("trigger has type=button", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-slot="preview"] [data-slot="collapsible-trigger"]');
    await expect(trigger.first()).toHaveAttribute("type", "button");
  });

  test("renders content container", async ({ page }) => {
    await gotoAndWait(page);
    const content = page.locator('[data-slot="preview"] [data-slot="collapsible-content"]');
    const count = await content.count();
    expect(count).toBeGreaterThanOrEqual(1);
  });
});

// ---------------------------------------------------------------------------
// Data attributes
// ---------------------------------------------------------------------------

test.describe("collapsible data attributes", () => {
  test("root has data-state=closed initially", async ({ page }) => {
    await gotoAndWait(page);
    const collapsible = page.locator('[data-slot="preview"] [data-slot="collapsible"]').first();
    await expect(collapsible).toHaveAttribute("data-state", "closed");
  });

  test("trigger has data-state=closed initially", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-slot="preview"] [data-slot="collapsible-trigger"]').first();
    await expect(trigger).toHaveAttribute("data-state", "closed");
  });

  test("content has data-state=closed initially", async ({ page }) => {
    await gotoAndWait(page);
    const content = page.locator('[data-slot="preview"] [data-slot="collapsible-content"]').first();
    await expect(content).toHaveAttribute("data-state", "closed");
  });
});

// ---------------------------------------------------------------------------
// ARIA attributes
// ---------------------------------------------------------------------------

test.describe("collapsible ARIA", () => {
  test("trigger has aria-expanded=false when closed", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-slot="preview"] [data-slot="collapsible-trigger"]').first();
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
  });

  test("trigger has aria-expanded=true when open", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-slot="preview"] [data-slot="collapsible-trigger"]').first();
    await trigger.click();
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
  });

  test("trigger has aria-controls linking to content", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-slot="preview"] [data-slot="collapsible-trigger"]').first();
    const ariaControls = await trigger.getAttribute("aria-controls");
    expect(ariaControls).toBeTruthy();

    // The aria-controls value should match the content's ID
    const content = page.locator('[data-slot="preview"] [data-slot="collapsible-content"]').first();
    const contentId = await content.getAttribute("id");
    expect(ariaControls).toBe(contentId);
  });
});

// ---------------------------------------------------------------------------
// Interaction
// ---------------------------------------------------------------------------

test.describe("collapsible interaction", () => {
  test("click opens then closes", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-slot="preview"] [data-slot="collapsible-trigger"]').first();
    const collapsible = page.locator('[data-slot="preview"] [data-slot="collapsible"]').first();
    const content = page.locator('[data-slot="preview"] [data-slot="collapsible-content"]').first();

    // Initially closed
    await expect(collapsible).toHaveAttribute("data-state", "closed");
    await expect(trigger).toHaveAttribute("aria-expanded", "false");

    // Click to open
    await trigger.click();
    await expect(collapsible).toHaveAttribute("data-state", "open");
    await expect(trigger).toHaveAttribute("data-state", "open");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    await expect(content).toHaveAttribute("data-state", "open");

    // Click to close
    await trigger.click();
    await expect(collapsible).toHaveAttribute("data-state", "closed");
    await expect(trigger).toHaveAttribute("data-state", "closed");
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
  });

  test("Space key toggles", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-slot="preview"] [data-slot="collapsible-trigger"]').first();

    await trigger.focus();
    await expect(trigger).toHaveAttribute("data-state", "closed");

    await page.keyboard.press("Space");
    await expect(trigger).toHaveAttribute("data-state", "open");

    await page.keyboard.press("Space");
    await expect(trigger).toHaveAttribute("data-state", "closed");
  });

  test("Enter key toggles", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-slot="preview"] [data-slot="collapsible-trigger"]').first();

    await trigger.focus();
    await expect(trigger).toHaveAttribute("data-state", "closed");

    await page.keyboard.press("Enter");
    await expect(trigger).toHaveAttribute("data-state", "open");

    await page.keyboard.press("Enter");
    await expect(trigger).toHaveAttribute("data-state", "closed");
  });
});

// ---------------------------------------------------------------------------
// Accessibility
// ---------------------------------------------------------------------------

test.describe("collapsible accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="preview"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
