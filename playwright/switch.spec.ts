import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/switch";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (switch rendered). */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Basic rendering
// ---------------------------------------------------------------------------

test.describe("switch rendering", () => {
  test("renders switch element", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    await expect(switchEl).toBeVisible();
  });

  test("switch is a button element", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    const tagName = await switchEl.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("button");
  });

  test("switch has type=button", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    await expect(switchEl).toHaveAttribute("type", "button");
  });

  test("switch contains a thumb", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    const thumb = switchEl.locator('[data-slot="switch-thumb"]');
    await expect(thumb).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// ARIA attributes
// ---------------------------------------------------------------------------

test.describe("switch ARIA", () => {
  test("has role=switch", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    await expect(switchEl).toHaveAttribute("role", "switch");
  });

  test("has aria-checked=false when unchecked", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    await expect(switchEl).toHaveAttribute("aria-checked", "false");
  });

  test("has aria-checked=true when checked", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    await switchEl.click();
    await expect(switchEl).toHaveAttribute("aria-checked", "true");
  });
});

// ---------------------------------------------------------------------------
// Data attributes
// ---------------------------------------------------------------------------

test.describe("switch data attributes", () => {
  test("has data-state=unchecked initially", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    await expect(switchEl).toHaveAttribute("data-state", "unchecked");
  });

  test("has data-state=checked after click", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    await switchEl.click();
    await expect(switchEl).toHaveAttribute("data-state", "checked");
  });

  test("thumb data-state syncs with switch", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    const thumb = switchEl.locator('[data-slot="switch-thumb"]');

    await expect(thumb).toHaveAttribute("data-state", "unchecked");

    await switchEl.click();
    await expect(thumb).toHaveAttribute("data-state", "checked");

    await switchEl.click();
    await expect(thumb).toHaveAttribute("data-state", "unchecked");
  });
});

// ---------------------------------------------------------------------------
// Hidden input
// ---------------------------------------------------------------------------

test.describe("switch hidden input", () => {
  test("has hidden checkbox input for form submission", async ({ page }) => {
    await gotoAndWait(page);
    const hiddenInput = page.locator('[data-slot="preview"] input[type="checkbox"][aria-hidden="true"]').first();
    await expect(hiddenInput).toBeAttached();
    await expect(hiddenInput).toHaveAttribute("tabindex", "-1");
  });
});

// ---------------------------------------------------------------------------
// Interaction
// ---------------------------------------------------------------------------

test.describe("switch interaction", () => {
  test("click toggles checked then unchecked", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();

    await expect(switchEl).toHaveAttribute("data-state", "unchecked");
    await expect(switchEl).toHaveAttribute("aria-checked", "false");

    // Click to check
    await switchEl.click();
    await expect(switchEl).toHaveAttribute("data-state", "checked");
    await expect(switchEl).toHaveAttribute("aria-checked", "true");

    // Click to uncheck
    await switchEl.click();
    await expect(switchEl).toHaveAttribute("data-state", "unchecked");
    await expect(switchEl).toHaveAttribute("aria-checked", "false");
  });

  test("Space key toggles state", async ({ page }) => {
    await gotoAndWait(page);
    const switchEl = page.locator('[data-slot="preview"] [data-slot="switch"]').first();
    await switchEl.focus();

    await expect(switchEl).toHaveAttribute("data-state", "unchecked");

    await page.keyboard.press("Space");
    await expect(switchEl).toHaveAttribute("data-state", "checked");

    await page.keyboard.press("Space");
    await expect(switchEl).toHaveAttribute("data-state", "unchecked");
  });
});

// ---------------------------------------------------------------------------
// Accessibility
// ---------------------------------------------------------------------------

test.describe("switch accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="preview"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
