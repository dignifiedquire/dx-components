import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/focus_scope";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="focus-scope-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Upstream: focus-scope.test.tsx — "given a default FocusScope"
// Tests focus trapping with loop=true, trapped=true
// ---------------------------------------------------------------------------

test.describe("FocusScope: basic trapped + looping", () => {
  test("should trap focus inside scope and loop forward", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-trap"]');

    // Open the trapped scope
    await section.locator('[data-testid="trap-trigger"]').click();
    await expect(section.locator('[data-testid="trap-scope"]')).toBeVisible();

    // Focus should auto-move inside the scope on mount
    const focusedInScope = await page.evaluate(() => {
      const scope = document.querySelector('[data-testid="trap-scope"]');
      return scope?.contains(document.activeElement);
    });
    expect(focusedInScope).toBe(true);

    // Focus the first input explicitly
    await section.locator('[data-testid="name-input"]').focus();
    await expect(section.locator('[data-testid="name-input"]')).toBeFocused();

    // Tab forward: Name → Email
    await page.keyboard.press("Tab");
    await expect(section.locator('[data-testid="email-input"]')).toBeFocused();

    // Tab forward: Email → Submit
    await page.keyboard.press("Tab");
    await expect(section.locator('[data-testid="submit-btn"]')).toBeFocused();

    // Tab forward: Submit → Close
    await page.keyboard.press("Tab");
    await expect(section.locator('[data-testid="close-trap"]')).toBeFocused();

    // Tab forward from last: should loop back to Name (first tabbable)
    await page.keyboard.press("Tab");
    await expect(section.locator('[data-testid="name-input"]')).toBeFocused();
  });

  test("should loop backward with Shift+Tab from first element", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-trap"]');

    await section.locator('[data-testid="trap-trigger"]').click();
    await expect(section.locator('[data-testid="trap-scope"]')).toBeVisible();

    // Focus the first input
    await section.locator('[data-testid="name-input"]').focus();
    await expect(section.locator('[data-testid="name-input"]')).toBeFocused();

    // Shift+Tab from first should go to last (Close button)
    await page.keyboard.press("Shift+Tab");
    await expect(section.locator('[data-testid="close-trap"]')).toBeFocused();
  });

  test("should keep focus inside scope (not escape to outside inputs)", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-trap"]');

    await section.locator('[data-testid="trap-trigger"]').click();
    await expect(section.locator('[data-testid="trap-scope"]')).toBeVisible();

    // Tab several times — focus must never land on outside inputs
    for (let i = 0; i < 8; i++) {
      await page.keyboard.press("Tab");
      const focusedInScope = await page.evaluate(() => {
        const scope = document.querySelector('[data-testid="trap-scope"]');
        return scope?.contains(document.activeElement);
      });
      expect(focusedInScope).toBe(true);
    }
  });
});

// ---------------------------------------------------------------------------
// Upstream: focus-scope.test.tsx — "first focusable has a negative tabindex"
// ---------------------------------------------------------------------------

test.describe("FocusScope: negative tabindex skipping", () => {
  test("should skip element with negative tabindex on Tab", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="negative-tabindex"]');

    await section.locator('[data-testid="neg-trap-trigger"]').click();
    await expect(
      section.locator('[data-testid="neg-trap-scope"]')
    ).toBeVisible();

    // Focus the last button, then Tab forward — should skip tabindex=-1 input
    await section.locator('[data-testid="neg-close"]').focus();
    await expect(section.locator('[data-testid="neg-close"]')).toBeFocused();

    await page.keyboard.press("Tab");
    // Should loop to Email (skipping Name which has tabindex=-1)
    await expect(
      section.locator('[data-testid="neg-email-input"]')
    ).toBeFocused();
  });

  test("should skip element with negative tabindex on Shift+Tab", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="negative-tabindex"]');

    await section.locator('[data-testid="neg-trap-trigger"]').click();
    await expect(
      section.locator('[data-testid="neg-trap-scope"]')
    ).toBeVisible();

    // Focus Email, then Shift+Tab — should skip Name (tabindex=-1) and go to Close
    await section.locator('[data-testid="neg-email-input"]').focus();
    await expect(
      section.locator('[data-testid="neg-email-input"]')
    ).toBeFocused();

    await page.keyboard.press("Shift+Tab");
    await expect(section.locator('[data-testid="neg-close"]')).toBeFocused();
  });
});

// ---------------------------------------------------------------------------
// Upstream: focus-scope.stories.tsx — "Multiple"
// ---------------------------------------------------------------------------

test.describe("FocusScope: multiple scopes", () => {
  test("opening second scope should trap focus in second scope", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="multiple-scopes"]');

    // Open scope 1
    await section.locator('[data-testid="multi-trap1-trigger"]').click();
    await expect(
      section.locator('[data-testid="multi-scope-1"]')
    ).toBeVisible();

    // Focus should be inside scope 1
    const inScope1 = await page.evaluate(() => {
      const scope = document.querySelector('[data-testid="multi-scope-1"]');
      return scope?.contains(document.activeElement);
    });
    expect(inScope1).toBe(true);

    // Close scope 1
    await section.locator('[data-testid="multi1-close"]').click();
    await expect(
      section.locator('[data-testid="multi-scope-1"]')
    ).not.toBeVisible();

    // Open scope 2
    await section.locator('[data-testid="multi-trap2-trigger"]').click();
    await expect(
      section.locator('[data-testid="multi-scope-2"]')
    ).toBeVisible();

    // Focus should be inside scope 2
    const inScope2 = await page.evaluate(() => {
      const scope = document.querySelector('[data-testid="multi-scope-2"]');
      return scope?.contains(document.activeElement);
    });
    expect(inScope2).toBe(true);

    // Tab should cycle within scope 2
    await section.locator('[data-testid="multi2-first"]').focus();
    await page.keyboard.press("Tab");
    await expect(
      section.locator('[data-testid="multi2-last"]')
    ).toBeFocused();

    await page.keyboard.press("Tab");
    await expect(
      section.locator('[data-testid="multi2-close"]')
    ).toBeFocused();

    // Loop back to first
    await page.keyboard.press("Tab");
    await expect(
      section.locator('[data-testid="multi2-first"]')
    ).toBeFocused();
  });
});
