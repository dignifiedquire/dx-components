import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/dismissable_layer";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="dismissable-layer-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Upstream: dismissable-layer.stories.tsx — "Basic"
// ---------------------------------------------------------------------------

test.describe("DismissableLayer: basic dismiss", () => {
  test("should open and show layer content", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-dismiss"]');

    await section.locator('[data-testid="basic-trigger"]').click();
    await expect(section.locator('[data-testid="basic-layer"]')).toBeVisible();
  });

  test("should dismiss on Escape key", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-dismiss"]');

    await section.locator('[data-testid="basic-trigger"]').click();
    await expect(section.locator('[data-testid="basic-layer"]')).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(
      section.locator('[data-testid="basic-layer"]')
    ).not.toBeVisible();

    await expect(section.locator('[data-testid="basic-dismiss-count"]')).toContainText(
      "Dismiss count: 1"
    );
  });

  test("should dismiss on click outside", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-dismiss"]');

    await section.locator('[data-testid="basic-trigger"]').click();
    await expect(section.locator('[data-testid="basic-layer"]')).toBeVisible();

    // Click outside the layer
    await section.locator('[data-testid="basic-outside-btn"]').click();
    await expect(
      section.locator('[data-testid="basic-layer"]')
    ).not.toBeVisible();

    await expect(section.locator('[data-testid="basic-dismiss-count"]')).toContainText(
      "Dismiss count: 1"
    );
  });

  test("should not dismiss when clicking inside the layer", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-dismiss"]');

    await section.locator('[data-testid="basic-trigger"]').click();
    await expect(section.locator('[data-testid="basic-layer"]')).toBeVisible();

    // Click inside the layer
    await section.locator('[data-testid="basic-layer-input"]').click();
    await expect(section.locator('[data-testid="basic-layer"]')).toBeVisible();

    await expect(section.locator('[data-testid="basic-dismiss-count"]')).toContainText(
      "Dismiss count: 0"
    );
  });

  test("should close via internal close button", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic-dismiss"]');

    await section.locator('[data-testid="basic-trigger"]').click();
    await expect(section.locator('[data-testid="basic-layer"]')).toBeVisible();

    await section.locator('[data-testid="basic-close"]').click();
    await expect(
      section.locator('[data-testid="basic-layer"]')
    ).not.toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Escape key callback
// ---------------------------------------------------------------------------

test.describe("DismissableLayer: escape key callback", () => {
  test("should fire on_escape_key_down callback", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="escape-dismiss"]');

    await section.locator('[data-testid="escape-trigger"]').click();
    await expect(
      section.locator('[data-testid="escape-layer"]')
    ).toBeVisible();

    await page.keyboard.press("Escape");
    await expect(
      section.locator('[data-testid="escape-layer"]')
    ).not.toBeVisible();

    await expect(section.locator('[data-testid="escape-count"]')).toContainText(
      "Escape count: 1"
    );
  });
});

// ---------------------------------------------------------------------------
// Upstream: dismissable-layer.stories.tsx — "Nested"
// Only the topmost layer should respond to Escape
// ---------------------------------------------------------------------------

test.describe("DismissableLayer: nested layers", () => {
  test("escape should dismiss only the inner (topmost) layer", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="nested-dismiss"]');

    // Open outer
    await section.locator('[data-testid="nested-outer-trigger"]').click();
    await expect(
      section.locator('[data-testid="nested-outer-layer"]')
    ).toBeVisible();

    // Open inner
    await section.locator('[data-testid="nested-inner-trigger"]').click();
    await expect(
      section.locator('[data-testid="nested-inner-layer"]')
    ).toBeVisible();

    // Press Escape — only inner should close
    await page.keyboard.press("Escape");
    await expect(
      section.locator('[data-testid="nested-inner-layer"]')
    ).not.toBeVisible();
    await expect(
      section.locator('[data-testid="nested-outer-layer"]')
    ).toBeVisible();

    await expect(
      section.locator('[data-testid="nested-inner-dismiss-count"]')
    ).toContainText("Inner dismiss count: 1");
    await expect(
      section.locator('[data-testid="nested-outer-dismiss-count"]')
    ).toContainText("Outer dismiss count: 0");

    // Press Escape again — outer should close
    await page.keyboard.press("Escape");
    await expect(
      section.locator('[data-testid="nested-outer-layer"]')
    ).not.toBeVisible();

    await expect(
      section.locator('[data-testid="nested-outer-dismiss-count"]')
    ).toContainText("Outer dismiss count: 1");
  });

  test("clicking outside should dismiss layers", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="nested-dismiss"]');

    // Open outer + inner
    await section.locator('[data-testid="nested-outer-trigger"]').click();
    await expect(
      section.locator('[data-testid="nested-outer-layer"]')
    ).toBeVisible();

    await section.locator('[data-testid="nested-inner-trigger"]').click();
    await expect(
      section.locator('[data-testid="nested-inner-layer"]')
    ).toBeVisible();

    // Click completely outside both layers
    await section.locator('[data-testid="nested-outside-btn"]').click();

    // Both layers should be dismissed (outer's on_dismiss closes both)
    await expect(
      section.locator('[data-testid="nested-outer-layer"]')
    ).not.toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Upstream: dismissable-layer.stories.tsx — "Basic" with prevention
// on_escape_key_down calls prevent_default() => escape should NOT dismiss
// on_pointer_down_outside does NOT call prevent_default() => click outside SHOULD dismiss
// ---------------------------------------------------------------------------

test.describe("DismissableLayer: prevention", () => {
  test("escape should be prevented (layer stays open)", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="prevent-dismiss"]');

    await section.locator('[data-testid="prevent-trigger"]').click();
    await expect(
      section.locator('[data-testid="prevent-layer"]')
    ).toBeVisible();

    // Press Escape — callback fires but prevent_default() prevents dismissal
    await page.keyboard.press("Escape");

    // Layer should still be visible
    await expect(
      section.locator('[data-testid="prevent-layer"]')
    ).toBeVisible();

    // Callback was called
    await expect(
      section.locator('[data-testid="prevent-escape-count"]')
    ).toContainText("Escape callback count: 1");
  });

  test("pointer down outside should NOT be prevented (layer closes)", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="prevent-dismiss"]');

    await section.locator('[data-testid="prevent-trigger"]').click();
    await expect(
      section.locator('[data-testid="prevent-layer"]')
    ).toBeVisible();

    // Click outside — callback fires, no prevent_default(), so dismissal happens
    await section.locator('[data-testid="prevent-outside-btn"]').click();

    await expect(
      section.locator('[data-testid="prevent-layer"]')
    ).not.toBeVisible();

    // Callback was called
    await expect(
      section.locator('[data-testid="prevent-pointer-count"]')
    ).toContainText("Pointer outside callback count: 1");
  });
});

// ---------------------------------------------------------------------------
// Upstream: dismissable-layer.stories.tsx — "Basic" with disableOutsidePointerEvents
// ---------------------------------------------------------------------------

test.describe("DismissableLayer: disableOutsidePointerEvents", () => {
  test("body pointer-events should be none when layer is open", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="disable-pe-dismiss"]');

    // Before opening, body pointer-events should not be "none"
    const bodyPEBefore = await page.evaluate(
      () => document.body.style.pointerEvents
    );
    expect(bodyPEBefore).not.toBe("none");

    await section.locator('[data-testid="disable-pe-trigger"]').click();
    await expect(
      section.locator('[data-testid="disable-pe-layer"]')
    ).toBeVisible();

    // Body pointer-events should now be "none"
    const bodyPEDuring = await page.evaluate(
      () => document.body.style.pointerEvents
    );
    expect(bodyPEDuring).toBe("none");

    // Dismiss via escape
    await page.keyboard.press("Escape");
    await expect(
      section.locator('[data-testid="disable-pe-layer"]')
    ).not.toBeVisible();

    // Body pointer-events should be restored
    const bodyPEAfter = await page.evaluate(
      () => document.body.style.pointerEvents
    );
    expect(bodyPEAfter).not.toBe("none");
  });
});
