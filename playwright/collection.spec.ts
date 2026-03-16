import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/collection";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="collection-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Upstream: collection.stories.tsx — "Basic"
// ---------------------------------------------------------------------------

test.describe("Collection: basic", () => {
  test("should register 3 items", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic"]');
    await expect(section.locator('[data-testid="count-items"]')).toContainText(
      "Count: 3"
    );
  });

  test("should have correct labels", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic"]');
    const labels = section.locator('[data-testid="labels-items"]');
    await expect(labels).toContainText("Red");
    await expect(labels).toContainText("Green");
    await expect(labels).toContainText("Blue");
  });

  test("should track disabled items", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="basic"]');
    await expect(
      section.locator('[data-testid="disabled-items"]')
    ).toContainText("Green");
  });
});

// ---------------------------------------------------------------------------
// Upstream: collection.stories.tsx — "WithElementInBetween"
// Non-collection elements interspersed should not affect item count
// ---------------------------------------------------------------------------

test.describe("Collection: element in between", () => {
  test("should register 5 items (ignoring non-collection elements)", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="element-between"]');
    await expect(section.locator('[data-testid="count-items"]')).toContainText(
      "Count: 5"
    );
  });

  test("should have correct labels from both groups", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="element-between"]');
    const labels = section.locator('[data-testid="labels-items"]');
    await expect(labels).toContainText("Red");
    await expect(labels).toContainText("Hello");
    await expect(labels).toContainText("World");
  });
});

// ---------------------------------------------------------------------------
// Upstream: collection.stories.tsx — "DynamicInsertion"
// ---------------------------------------------------------------------------

test.describe("Collection: dynamic insertion", () => {
  test("initially has 3 items", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="dynamic-insertion"]');
    await expect(section.locator('[data-testid="count-items"]')).toContainText(
      "Count: 3"
    );
  });

  test("adding Tomato increases count to 4", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="dynamic-insertion"]');
    await section.locator('[data-testid="toggle-tomato"]').click();
    await expect(section.locator('[data-testid="count-items"]')).toContainText(
      "Count: 4"
    );
    await expect(
      section.locator('[data-testid="labels-items"]')
    ).toContainText("Tomato");
  });

  test("removing Tomato decreases count back to 3", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="dynamic-insertion"]');

    // Add
    await section.locator('[data-testid="toggle-tomato"]').click();
    await expect(section.locator('[data-testid="count-items"]')).toContainText(
      "Count: 4"
    );

    // Remove
    await section.locator('[data-testid="toggle-tomato"]').click();
    await expect(section.locator('[data-testid="count-items"]')).toContainText(
      "Count: 3"
    );
  });
});

// ---------------------------------------------------------------------------
// Upstream: collection.stories.tsx — "WithChangingItem"
// ---------------------------------------------------------------------------

test.describe("Collection: changing item state", () => {
  test("Green starts enabled (not in disabled list)", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="changing-item"]');
    const disabled = section.locator('[data-testid="disabled-items"]');
    // Green should NOT be in the disabled list initially
    await expect(disabled).not.toContainText("Green");
  });

  test("disabling Green adds it to disabled list", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="changing-item"]');

    await section.locator('[data-testid="toggle-disabled"]').click();
    await expect(
      section.locator('[data-testid="disabled-items"]')
    ).toContainText("Green");
  });

  test("re-enabling Green removes it from disabled list", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="changing-item"]');

    // Disable
    await section.locator('[data-testid="toggle-disabled"]').click();
    await expect(
      section.locator('[data-testid="disabled-items"]')
    ).toContainText("Green");

    // Re-enable
    await section.locator('[data-testid="toggle-disabled"]').click();
    await expect(
      section.locator('[data-testid="disabled-items"]')
    ).not.toContainText("Green");
  });
});

// ---------------------------------------------------------------------------
// Upstream: collection.stories.tsx — "Nested"
// Nested collections should be scoped independently
// ---------------------------------------------------------------------------

test.describe("Collection: nested", () => {
  test("inner collection has 3 items", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="nested"]');
    await expect(
      section.locator('[data-testid="count-inner"]')
    ).toContainText("Count: 3");
  });

  test("inner collection labels are 2.1, 2.2, 2.3", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="nested"]');
    const labels = section.locator('[data-testid="labels-inner"]');
    await expect(labels).toContainText("2.1");
    await expect(labels).toContainText("2.2");
    await expect(labels).toContainText("2.3");
  });

  test("outer collection has 3 top-level items", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="nested"]');
    await expect(
      section.locator('[data-testid="count-outer"]')
    ).toContainText("Count: 3");
  });

  test("outer collection labels are 1, 2, 3", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="nested"]');
    const labels = section.locator('[data-testid="labels-outer"]');
    await expect(labels).toContainText('"1"');
    await expect(labels).toContainText('"2"');
    await expect(labels).toContainText('"3"');
  });
});
