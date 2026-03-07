import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/toggle";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (toggle rendered). */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="toggle"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Basic rendering
// ---------------------------------------------------------------------------

test.describe("toggle rendering", () => {
  test("renders toggle buttons", async ({ page }) => {
    await gotoAndWait(page);
    const toggles = page.locator('[data-slot="toggle"]');
    const count = await toggles.count();
    expect(count).toBeGreaterThanOrEqual(1);
  });

  test("toggle is a button element", async ({ page }) => {
    await gotoAndWait(page);
    const toggle = page.locator('[data-slot="toggle"]').first();
    const tagName = await toggle.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("button");
  });

  test("toggle has type=button", async ({ page }) => {
    await gotoAndWait(page);
    const toggle = page.locator('[data-slot="toggle"]').first();
    await expect(toggle).toHaveAttribute("type", "button");
  });
});

// ---------------------------------------------------------------------------
// ARIA attributes
// ---------------------------------------------------------------------------

test.describe("toggle ARIA", () => {
  test("has aria-pressed=false when off", async ({ page }) => {
    await gotoAndWait(page);
    const toggle = page.locator('[data-slot="toggle"]').first();
    await expect(toggle).toHaveAttribute("aria-pressed", "false");
  });

  test("has aria-pressed=true when on", async ({ page }) => {
    await gotoAndWait(page);
    const toggle = page.locator('[data-slot="toggle"]').first();
    await toggle.click();
    await expect(toggle).toHaveAttribute("aria-pressed", "true");
  });
});

// ---------------------------------------------------------------------------
// Data attributes
// ---------------------------------------------------------------------------

test.describe("toggle data attributes", () => {
  test("has data-state=off initially", async ({ page }) => {
    await gotoAndWait(page);
    const toggle = page.locator('[data-slot="toggle"]').first();
    await expect(toggle).toHaveAttribute("data-state", "off");
  });

  test("has data-state=on after click", async ({ page }) => {
    await gotoAndWait(page);
    const toggle = page.locator('[data-slot="toggle"]').first();
    await toggle.click();
    await expect(toggle).toHaveAttribute("data-state", "on");
  });
});

// ---------------------------------------------------------------------------
// Interaction
// ---------------------------------------------------------------------------

test.describe("toggle interaction", () => {
  test("click toggles on then off", async ({ page }) => {
    await gotoAndWait(page);
    const toggle = page.locator('[data-slot="toggle"]').first();

    await expect(toggle).toHaveAttribute("data-state", "off");
    await expect(toggle).toHaveAttribute("aria-pressed", "false");

    // Click to turn on
    await toggle.click();
    await expect(toggle).toHaveAttribute("data-state", "on");
    await expect(toggle).toHaveAttribute("aria-pressed", "true");

    // Click to turn off
    await toggle.click();
    await expect(toggle).toHaveAttribute("data-state", "off");
    await expect(toggle).toHaveAttribute("aria-pressed", "false");
  });

  test("Space key toggles state", async ({ page }) => {
    await gotoAndWait(page);
    const toggle = page.locator('[data-slot="toggle"]').first();
    await toggle.focus();

    await expect(toggle).toHaveAttribute("data-state", "off");

    await page.keyboard.press("Space");
    await expect(toggle).toHaveAttribute("data-state", "on");

    await page.keyboard.press("Space");
    await expect(toggle).toHaveAttribute("data-state", "off");
  });
});

// ---------------------------------------------------------------------------
// Disabled state
// ---------------------------------------------------------------------------

test.describe("toggle disabled", () => {
  test("disabled toggle has disabled attribute", async ({ page }) => {
    await gotoAndWait(page);
    const disabled = page.locator('[data-slot="toggle"][disabled]');
    await expect(disabled.first()).toBeVisible();
    await expect(disabled.first()).toBeDisabled();
  });

  test("disabled toggle has data-disabled attribute", async ({ page }) => {
    await gotoAndWait(page);
    const disabled = page.locator('[data-slot="toggle"][disabled]');
    await expect(disabled.first()).toHaveAttribute("data-disabled", "");
  });

  test("click does nothing when disabled", async ({ page }) => {
    await gotoAndWait(page);
    const disabled = page.locator('[data-slot="toggle"][disabled]').first();
    const stateBefore = await disabled.getAttribute("data-state");
    await disabled.click({ force: true });
    await expect(disabled).toHaveAttribute("data-state", stateBefore!);
  });
});

// ---------------------------------------------------------------------------
// Accessibility
// ---------------------------------------------------------------------------

test.describe("toggle accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="toggle"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
