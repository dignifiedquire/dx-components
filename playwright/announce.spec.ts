import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/announce";
const TIMEOUT = { timeout: 20 * 60 * 1000 };
const EXPECT_TIMEOUT = { timeout: 15_000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="announce-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Inline element attributes
// ---------------------------------------------------------------------------

test.describe("Announce: polite (default)", () => {
  test("renders with data-slot=announce", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-polite"]');
    await expect(el).toHaveAttribute("data-slot", "announce");
  });

  test("has role=status", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-polite"]');
    await expect(el).toHaveAttribute("role", "status");
  });

  test("has aria-live=polite", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-polite"]');
    await expect(el).toHaveAttribute("aria-live", "polite");
  });

  test("has aria-atomic=false by default", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-polite"]');
    await expect(el).toHaveAttribute("aria-atomic", "false");
  });

  test("renders text content", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-polite"]');
    await expect(el).toContainText("Status update: task completed");
  });
});

test.describe("Announce: assertive", () => {
  test("has role=alert", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-assertive"]');
    await expect(el).toHaveAttribute("role", "alert");
  });

  test("has aria-live=assertive", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-assertive"]');
    await expect(el).toHaveAttribute("aria-live", "assertive");
  });
});

test.describe("Announce: off", () => {
  test("has role=none", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-off"]');
    await expect(el).toHaveAttribute("role", "none");
  });

  test("has aria-live=off", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-off"]');
    await expect(el).toHaveAttribute("aria-live", "off");
  });
});

test.describe("Announce: custom role", () => {
  test("log role overrides default", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-log"]');
    await expect(el).toHaveAttribute("role", "log");
    await expect(el).toHaveAttribute("aria-live", "polite");
  });
});

test.describe("Announce: region identifier", () => {
  test("renders with correct text", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-identified"]');
    await expect(el).toContainText("Toast: saved successfully");
  });
});

test.describe("Announce: atomic", () => {
  test("has aria-atomic=true", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-atomic"]');
    await expect(el).toHaveAttribute("aria-atomic", "true");
  });
});

test.describe("Announce: aria-relevant", () => {
  test("has aria-relevant attribute", async ({ page }) => {
    await gotoAndWait(page);
    const el = page.locator('[data-testid="announce-relevant"]');
    await expect(el).toHaveAttribute("aria-relevant", "additions removals");
  });
});

// ---------------------------------------------------------------------------
// Global live regions (wasm-only — created at document.body)
// ---------------------------------------------------------------------------

test.describe("Announce: global live regions", () => {
  test("polite creates a global live region at body", async ({ page }) => {
    await gotoAndWait(page);
    // Use first() since multiple polite regions may exist (atomic, relevant variants)
    const region = page
      .locator(
        "body > [data-radix-announce-region][aria-live=polite][role=status]",
      )
      .first();
    await expect(region).toBeAttached(EXPECT_TIMEOUT);
  });

  test("assertive creates a global live region at body", async ({ page }) => {
    await gotoAndWait(page);
    const region = page.locator(
      "body > [data-radix-announce-region][aria-live=assertive][role=alert]",
    );
    await expect(region).toBeAttached(EXPECT_TIMEOUT);
  });

  test("identified region uses custom data attribute", async ({ page }) => {
    await gotoAndWait(page);
    const region = page.locator(
      "body > [data-radix-announce-region-toast-region]",
    );
    await expect(region).toBeAttached(EXPECT_TIMEOUT);
  });

  test("global live region is visually hidden", async ({ page }) => {
    await gotoAndWait(page);
    const region = page
      .locator("body > [data-radix-announce-region][aria-live=polite]")
      .first();
    const style = await region.getAttribute("style");
    expect(style).toContain("position: absolute");
    expect(style).toContain("width: 1px");
    expect(style).toContain("height: 1px");
    expect(style).toContain("overflow: hidden");
  });

  test("global live region mirrors content", async ({ page }) => {
    await gotoAndWait(page);
    // The polite+status region without atomic or relevant attributes
    // contains mirrored content from the polite announce demo
    const region = page
      .locator(
        "body > [data-radix-announce-region][aria-live=polite][role=status]",
      )
      .first();
    await expect(region).toContainText("Status update: task completed");
  });
});
