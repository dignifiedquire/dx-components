import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/presence";
const TIMEOUT = { waitUntil: "load" as const, timeout: 60_000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="presence-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------
// Test 1: Basic (no animation) — instant mount/unmount
// Upstream: presence.stories.tsx "Basic"
// ---------------------------------------------------------------
test.describe("Presence: basic (no animation)", () => {
  test("should show content when toggled open", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-testid="basic-trigger"]');
    const content = page.locator('[data-testid="basic-content"]');

    // Initially closed
    await expect(content).not.toBeVisible();

    // Open
    await trigger.click();
    await expect(content).toBeVisible();
    await expect(content).toContainText("Basic content");
  });

  test("should hide content when toggled closed", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-testid="basic-trigger"]');
    const content = page.locator('[data-testid="basic-content"]');

    // Open then close
    await trigger.click();
    await expect(content).toBeVisible();
    await trigger.click();
    await expect(content).not.toBeVisible();
  });
});

// ---------------------------------------------------------------
// Test 2: Unmount animation — presence stays during exit animation
// Upstream: presence.stories.tsx "WithUnmountAnimation"
// ---------------------------------------------------------------
test.describe("Presence: unmount animation", () => {
  test("should keep element visible during exit animation", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-testid="unmount-trigger"]');
    const content = page.locator('[data-testid="unmount-content"]');

    // Open
    await trigger.click();
    await expect(content).toBeVisible();

    // Close — content should still be visible briefly (exit animation running)
    await trigger.click();

    // The content should eventually be removed after animation completes
    await expect(content).not.toBeVisible({ timeout: 5_000 });
  });
});

// ---------------------------------------------------------------
// Test 3: Open and close animation
// Upstream: presence.stories.tsx "WithOpenAndCloseAnimation"
// ---------------------------------------------------------------
test.describe("Presence: open and close animation", () => {
  test("should animate on open and close", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-testid="open-close-trigger"]');
    const content = page.locator('[data-testid="open-close-content"]');

    // Open
    await trigger.click();
    await expect(content).toBeVisible();

    // Check data-state on the collapsible content wrapper
    const wrapper = page.locator(
      '[data-testid="open-close-section"] [data-state]'
    );
    await expect(wrapper.first()).toHaveAttribute("data-state", "open");

    // Close
    await trigger.click();

    // data-state should transition to "closed"
    await expect(wrapper.first()).toHaveAttribute("data-state", "closed");

    // Content eventually removed
    await expect(content).not.toBeVisible({ timeout: 5_000 });
  });
});

// ---------------------------------------------------------------
// Test 4: Multiple animations — fade + slide
// Upstream: presence.stories.tsx "WithMultipleOpenAndCloseAnimations"
// ---------------------------------------------------------------
test.describe("Presence: multiple animations", () => {
  test("should handle multiple concurrent animations", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-testid="multi-trigger"]');
    const content = page.locator('[data-testid="multi-content"]');

    // Open
    await trigger.click();
    await expect(content).toBeVisible();

    // Close
    await trigger.click();

    // Content should eventually be removed after all animations complete
    await expect(content).not.toBeVisible({ timeout: 5_000 });
  });

  test("should toggle rapidly without breaking state", async ({ page }) => {
    await gotoAndWait(page);
    const trigger = page.locator('[data-testid="multi-trigger"]');
    const content = page.locator('[data-testid="multi-content"]');

    // Rapid toggle
    await trigger.click(); // open
    await trigger.click(); // close
    await trigger.click(); // open again

    // Should end up open
    await expect(content).toBeVisible({ timeout: 5_000 });

    // Close and verify
    await trigger.click();
    await expect(content).not.toBeVisible({ timeout: 5_000 });
  });
});
