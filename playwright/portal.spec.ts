import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/portal";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="portal-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Upstream: portal.stories.tsx — "Base"
// Content renders through Portal into PortalHost.
// Dioxus portals may briefly render inline before settling into PortalHost,
// so we scope content queries to the host.
// ---------------------------------------------------------------------------

test.describe("Portal: base", () => {
  test("portal content is not visible in host until toggled", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const host = page.locator('[data-slot="portal-host"]');
    await expect(
      host.locator('[data-testid="base-portal-content"]')
    ).not.toBeVisible();
  });

  test("portal content appears in host when toggled on", async ({ page }) => {
    await gotoAndWait(page);
    await page.locator('[data-testid="base-toggle"]').click();
    const host = page.locator('[data-slot="portal-host"]');
    await expect(
      host.locator('[data-testid="base-portal-content"]')
    ).toBeVisible();
  });

  test("portal content renders inside PortalHost, not the overflow container", async ({
    page,
  }) => {
    await gotoAndWait(page);
    await page.locator('[data-testid="base-toggle"]').click();

    // The portal content should be inside [data-slot="portal-host"],
    // NOT inside the overflow container.
    const inHost = page.locator(
      '[data-slot="portal-host"] [data-testid="base-portal-content"]'
    );
    await expect(inHost).toBeVisible();
  });

  test("portal content disappears from host when toggled off", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const host = page.locator('[data-slot="portal-host"]');

    await page.locator('[data-testid="base-toggle"]').click();
    await expect(
      host.locator('[data-testid="base-portal-content"]')
    ).toBeVisible();

    await page.locator('[data-testid="base-toggle"]').click();
    await expect(
      host.locator('[data-testid="base-portal-content"]')
    ).not.toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Upstream: portal.stories.tsx — "Chromatic" zIndex and order section
// Multiple portals render and maintain order
// ---------------------------------------------------------------------------

test.describe("Portal: multiple portals", () => {
  test("all three portals render in host when toggled", async ({ page }) => {
    await gotoAndWait(page);
    await page.locator('[data-testid="multi-toggle"]').click();

    const host = page.locator('[data-slot="portal-host"]');
    await expect(
      host.locator('[data-testid="multi-portal-1"]')
    ).toBeVisible();
    await expect(
      host.locator('[data-testid="multi-portal-2"]')
    ).toBeVisible();
    await expect(
      host.locator('[data-testid="multi-portal-3"]')
    ).toBeVisible();
  });

  test("all portals render inside PortalHost", async ({ page }) => {
    await gotoAndWait(page);
    await page.locator('[data-testid="multi-toggle"]').click();

    const host = page.locator('[data-slot="portal-host"]');
    await expect(
      host.locator('[data-testid="multi-portal-1"]')
    ).toBeVisible();
    await expect(
      host.locator('[data-testid="multi-portal-2"]')
    ).toBeVisible();
    await expect(
      host.locator('[data-testid="multi-portal-3"]')
    ).toBeVisible();
  });

  test("all portals disappear from host when toggled off", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const host = page.locator('[data-slot="portal-host"]');

    await page.locator('[data-testid="multi-toggle"]').click();
    await expect(
      host.locator('[data-testid="multi-portal-1"]')
    ).toBeVisible();

    await page.locator('[data-testid="multi-toggle"]').click();
    await expect(
      host.locator('[data-testid="multi-portal-1"]')
    ).not.toBeVisible();
    await expect(
      host.locator('[data-testid="multi-portal-2"]')
    ).not.toBeVisible();
    await expect(
      host.locator('[data-testid="multi-portal-3"]')
    ).not.toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Portal renders in PortalHost, not inline
// ---------------------------------------------------------------------------

test.describe("Portal: location", () => {
  test("portal content renders in PortalHost, not in the section", async ({
    page,
  }) => {
    await gotoAndWait(page);

    // Content should exist inside the PortalHost
    const inHost = page.locator(
      '[data-slot="portal-host"] [data-testid="location-portal-content"]'
    );
    await expect(inHost).toBeVisible();

    // It should NOT be inside the portal-location section
    const inSection = page.locator(
      '[data-testid="portal-location"] [data-testid="location-portal-content"]'
    );
    await expect(inSection).not.toBeVisible();
  });
});
