import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/portal";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="portal-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
  // Wait until the WASM bundle finishes loading so click handlers
  // are wired before we interact. Firefox / WebKit are slower than
  // Chromium here, and without this the toggle click can fire into
  // a not-yet-hydrated button on those engines.
  await page.waitForLoadState("networkidle");
}

// ---------------------------------------------------------------------------
// Portal is now a no-op pass-through. Children render inline where the
// component sits in the tree — there is no PortalHost and no DOM
// re-parenting. Top-layer escape lives on overlay primitives via the
// `popover` attribute / `<dialog>` element (see Top Layer demo).
// ---------------------------------------------------------------------------

test.describe("Portal: base", () => {
  test("portal content is not present until toggled", async ({ page }) => {
    await gotoAndWait(page);
    await expect(
      page.locator('[data-testid="base-portal-content"]')
    ).toHaveCount(0);
  });

  test("portal content appears inline when toggled on", async ({ page }) => {
    await gotoAndWait(page);
    await page.locator('[data-testid="base-toggle"]').click();
    await expect(
      page.locator(
        '[data-testid="inline-container"] [data-testid="base-portal-content"]'
      )
    ).toBeVisible();
  });

  test("portal content disappears when toggled off", async ({ page }) => {
    await gotoAndWait(page);
    const content = page.locator('[data-testid="base-portal-content"]');

    await page.locator('[data-testid="base-toggle"]').click();
    await expect(content).toBeVisible();

    await page.locator('[data-testid="base-toggle"]').click();
    await expect(content).toHaveCount(0);
  });
});

test.describe("Portal: multiple portals", () => {
  test("all three portals render inline when toggled", async ({ page }) => {
    await gotoAndWait(page);
    await page.locator('[data-testid="multi-toggle"]').click();

    const section = page.locator('[data-testid="portal-multi"]');
    await expect(
      section.locator('[data-testid="multi-portal-1"]')
    ).toBeVisible();
    await expect(
      section.locator('[data-testid="multi-portal-2"]')
    ).toBeVisible();
    await expect(
      section.locator('[data-testid="multi-portal-3"]')
    ).toBeVisible();
  });

  test("all portals disappear when toggled off", async ({ page }) => {
    await gotoAndWait(page);

    await page.locator('[data-testid="multi-toggle"]').click();
    await expect(
      page.locator('[data-testid="multi-portal-1"]')
    ).toBeVisible();

    await page.locator('[data-testid="multi-toggle"]').click();
    await expect(page.locator('[data-testid="multi-portal-1"]')).toHaveCount(0);
    await expect(page.locator('[data-testid="multi-portal-2"]')).toHaveCount(0);
    await expect(page.locator('[data-testid="multi-portal-3"]')).toHaveCount(0);
  });
});

// ---------------------------------------------------------------------------
// There is no PortalHost anymore — confirm the slot element is absent.
// ---------------------------------------------------------------------------

test.describe("Portal: no host", () => {
  test("no [data-slot=portal-host] element exists in the document", async ({
    page,
  }) => {
    await gotoAndWait(page);
    await expect(page.locator('[data-slot="portal-host"]')).toHaveCount(0);
  });
});
