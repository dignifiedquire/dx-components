import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/roving_focus";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="roving-focus-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Upstream: roving-focus-group.stories.tsx — "Basic" (horizontal, no loop)
// ---------------------------------------------------------------------------

test.describe("RovingFocus: horizontal no loop", () => {
  test("ArrowRight moves focus to next focusable item", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-horizontal"]');
    const buttons = section.locator("button[data-value]");

    // Focus first button
    await buttons.nth(0).focus();
    await expect(buttons.nth(0)).toBeFocused();

    // ArrowRight -> Two
    await page.keyboard.press("ArrowRight");
    await expect(buttons.nth(1)).toBeFocused();
  });

  test("ArrowRight skips disabled items", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-horizontal"]');
    const buttons = section.locator("button[data-value]");

    // Focus Two
    await buttons.nth(1).focus();
    await expect(buttons.nth(1)).toBeFocused();

    // ArrowRight -> should skip Three (disabled) -> Four
    await page.keyboard.press("ArrowRight");
    await expect(buttons.nth(3)).toBeFocused();
  });

  test("ArrowRight at end does NOT loop", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-horizontal"]');
    const buttons = section.locator("button[data-value]");

    // Focus Four (last)
    await buttons.nth(3).focus();
    await expect(buttons.nth(3)).toBeFocused();

    // ArrowRight -> should stay on Four (no loop)
    await page.keyboard.press("ArrowRight");
    await expect(buttons.nth(3)).toBeFocused();
  });

  test("ArrowLeft moves focus to previous item", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-horizontal"]');
    const buttons = section.locator("button[data-value]");

    // Focus Two
    await buttons.nth(1).focus();
    await expect(buttons.nth(1)).toBeFocused();

    // ArrowLeft -> One
    await page.keyboard.press("ArrowLeft");
    await expect(buttons.nth(0)).toBeFocused();
  });

  test("vertical arrows do nothing in horizontal orientation", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-horizontal"]');
    const buttons = section.locator("button[data-value]");

    await buttons.nth(0).focus();
    await page.keyboard.press("ArrowDown");
    await expect(buttons.nth(0)).toBeFocused();

    await page.keyboard.press("ArrowUp");
    await expect(buttons.nth(0)).toBeFocused();
  });
});

// ---------------------------------------------------------------------------
// Upstream: "Basic" (horizontal + looping)
// ---------------------------------------------------------------------------

test.describe("RovingFocus: horizontal with loop", () => {
  test("ArrowRight at end loops to first", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-horizontal-loop"]');
    const buttons = section.locator("button[data-value]");

    // Focus Four (last)
    await buttons.nth(3).focus();
    await expect(buttons.nth(3)).toBeFocused();

    // ArrowRight -> should loop to One
    await page.keyboard.press("ArrowRight");
    await expect(buttons.nth(0)).toBeFocused();
  });

  test("ArrowLeft at start loops to last", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-horizontal-loop"]');
    const buttons = section.locator("button[data-value]");

    // Focus One (first)
    await buttons.nth(0).focus();
    await expect(buttons.nth(0)).toBeFocused();

    // ArrowLeft -> should loop to Four (skipping disabled Three)
    await page.keyboard.press("ArrowLeft");
    await expect(buttons.nth(3)).toBeFocused();
  });
});

// ---------------------------------------------------------------------------
// Upstream: "Basic" (vertical, no loop)
// ---------------------------------------------------------------------------

test.describe("RovingFocus: vertical no loop", () => {
  test("ArrowDown moves focus to next", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-vertical"]');
    const buttons = section.locator("button[data-value]");

    await buttons.nth(0).focus();
    await page.keyboard.press("ArrowDown");
    await expect(buttons.nth(1)).toBeFocused();
  });

  test("ArrowUp moves focus to previous", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-vertical"]');
    const buttons = section.locator("button[data-value]");

    await buttons.nth(1).focus();
    await page.keyboard.press("ArrowUp");
    await expect(buttons.nth(0)).toBeFocused();
  });

  test("horizontal arrows do nothing in vertical orientation", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-vertical"]');
    const buttons = section.locator("button[data-value]");

    await buttons.nth(0).focus();
    await page.keyboard.press("ArrowRight");
    await expect(buttons.nth(0)).toBeFocused();

    await page.keyboard.press("ArrowLeft");
    await expect(buttons.nth(0)).toBeFocused();
  });
});

// ---------------------------------------------------------------------------
// Upstream: "Basic" (vertical + looping)
// ---------------------------------------------------------------------------

test.describe("RovingFocus: vertical with loop", () => {
  test("ArrowDown at end loops to first", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-vertical-loop"]');
    const buttons = section.locator("button[data-value]");

    // Focus Four (last)
    await buttons.nth(3).focus();
    await page.keyboard.press("ArrowDown");
    await expect(buttons.nth(0)).toBeFocused();
  });

  test("ArrowUp at start loops to last", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-vertical-loop"]');
    const buttons = section.locator("button[data-value]");

    await buttons.nth(0).focus();
    await page.keyboard.press("ArrowUp");
    await expect(buttons.nth(3)).toBeFocused();
  });
});

// ---------------------------------------------------------------------------
// Upstream: "EdgeCases" — dynamic insertion + disabling
// ---------------------------------------------------------------------------

test.describe("RovingFocus: edge cases", () => {
  test("dynamically inserted item becomes focusable", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="edge-cases"]');

    // Add Extra
    await section.locator('[data-testid="toggle-extra"]').click();
    const extra = section.locator('button[data-value="extra"]');
    await expect(extra).toBeVisible();

    // Focus Extra
    await extra.focus();
    await expect(extra).toBeFocused();
  });

  test("disabling an item makes it unfocusable via arrow keys", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="edge-cases"]');

    // Disable One
    await section.locator('[data-testid="toggle-disable-one"]').click();

    // Focus Three (first focusable after Two which is always disabled)
    const three = section.locator('button[data-value="three"]');
    await three.focus();
    await expect(three).toBeFocused();

    // ArrowLeft should skip disabled One and Two
    // (behavior depends on whether there's a focusable item before Three)
    await page.keyboard.press("ArrowLeft");
    // Should not focus on disabled One
    const one = section.locator('button[data-value="one"]');
    await expect(one).not.toBeFocused();
  });

  test("Tab moves focus out of the group", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="edge-cases"]');

    const three = section.locator('button[data-value="three"]');
    await three.focus();
    await expect(three).toBeFocused();

    // Tab should move focus to the outside button
    await page.keyboard.press("Tab");
    const outside = section.locator('[data-testid="outside-button"]');
    await expect(outside).toBeFocused();
  });
});
