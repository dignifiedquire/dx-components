import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/checkbox";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (checkbox rendered). */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Basic rendering
// ---------------------------------------------------------------------------

test.describe("checkbox rendering", () => {
  test("renders checkbox elements", async ({ page }) => {
    await gotoAndWait(page);
    const checkboxes = page.locator('[data-slot="preview"] [data-slot="checkbox"]');
    const count = await checkboxes.count();
    expect(count).toBeGreaterThanOrEqual(1);
  });

  test("checkbox is a button element", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    const tagName = await checkbox.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("button");
  });

  test("checkbox has type=button", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    await expect(checkbox).toHaveAttribute("type", "button");
  });

  test("checkbox contains an indicator", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    const indicator = checkbox.locator('[data-slot="checkbox-indicator"]');
    await expect(indicator).toBeAttached();
  });
});

// ---------------------------------------------------------------------------
// ARIA attributes
// ---------------------------------------------------------------------------

test.describe("checkbox ARIA", () => {
  test("has role=checkbox", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    await expect(checkbox).toHaveAttribute("role", "checkbox");
  });

  test("has aria-checked=false when unchecked", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    await expect(checkbox).toHaveAttribute("aria-checked", "false");
  });

  test("has aria-checked=true when checked", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    await checkbox.click();
    await expect(checkbox).toHaveAttribute("aria-checked", "true");
  });
});

// ---------------------------------------------------------------------------
// Data attributes
// ---------------------------------------------------------------------------

test.describe("checkbox data attributes", () => {
  test("has data-state=unchecked initially", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    await expect(checkbox).toHaveAttribute("data-state", "unchecked");
  });

  test("has data-state=checked after click", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    await checkbox.click();
    await expect(checkbox).toHaveAttribute("data-state", "checked");
  });

  test("indicator data-state syncs with checkbox", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    const indicator = checkbox.locator('[data-slot="checkbox-indicator"]');

    await expect(indicator).toHaveAttribute("data-state", "unchecked");

    await checkbox.click();
    await expect(indicator).toHaveAttribute("data-state", "checked");

    await checkbox.click();
    await expect(indicator).toHaveAttribute("data-state", "unchecked");
  });
});

// ---------------------------------------------------------------------------
// Hidden input
// ---------------------------------------------------------------------------

test.describe("checkbox hidden input", () => {
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

test.describe("checkbox interaction", () => {
  test("click toggles checked then unchecked", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();

    await expect(checkbox).toHaveAttribute("data-state", "unchecked");
    await expect(checkbox).toHaveAttribute("aria-checked", "false");

    // Click to check
    await checkbox.click();
    await expect(checkbox).toHaveAttribute("data-state", "checked");
    await expect(checkbox).toHaveAttribute("aria-checked", "true");

    // Click to uncheck
    await checkbox.click();
    await expect(checkbox).toHaveAttribute("data-state", "unchecked");
    await expect(checkbox).toHaveAttribute("aria-checked", "false");
  });

  test("Space key toggles state", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    await checkbox.focus();

    await expect(checkbox).toHaveAttribute("data-state", "unchecked");

    // Space activates the button (native behavior)
    await page.keyboard.press("Space");
    await expect(checkbox).toHaveAttribute("data-state", "checked");

    // Space again to uncheck
    await page.keyboard.press("Space");
    await expect(checkbox).toHaveAttribute("data-state", "unchecked");
  });

  test("Enter key does not toggle state", async ({ page }) => {
    await gotoAndWait(page);
    const checkbox = page.locator('[data-slot="preview"] [data-slot="checkbox"]').first();
    await checkbox.focus();

    await expect(checkbox).toHaveAttribute("data-state", "unchecked");

    // Enter is prevented on checkbox (WAI-ARIA)
    await page.keyboard.press("Enter");
    await expect(checkbox).toHaveAttribute("data-state", "unchecked");
  });
});

// ---------------------------------------------------------------------------
// Disabled state
// ---------------------------------------------------------------------------

test.describe("checkbox disabled", () => {
  test("disabled checkbox has disabled attribute", async ({ page }) => {
    await gotoAndWait(page);
    const disabled = page.locator('[data-slot="preview"] [data-slot="checkbox"][disabled]');
    await expect(disabled.first()).toBeVisible();
    await expect(disabled.first()).toBeDisabled();
  });

  test("disabled checkbox has data-disabled attribute", async ({ page }) => {
    await gotoAndWait(page);
    const disabled = page.locator('[data-slot="preview"] [data-slot="checkbox"][disabled]');
    await expect(disabled.first()).toHaveAttribute("data-disabled", "");
  });

  test("click does nothing when disabled", async ({ page }) => {
    await gotoAndWait(page);
    const disabled = page.locator('[data-slot="preview"] [data-slot="checkbox"][disabled]').first();
    const stateBefore = await disabled.getAttribute("data-state");
    await disabled.click({ force: true });
    await expect(disabled).toHaveAttribute("data-state", stateBefore!);
  });
});

// ---------------------------------------------------------------------------
// Accessibility
// ---------------------------------------------------------------------------

test.describe("checkbox accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="preview"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
