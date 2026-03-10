import { test, expect } from '@playwright/test';

const URL = "http://127.0.0.1:8080/docs/components/pagination";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

test.describe("Pagination", () => {
  test("renders with correct data-slot attributes", async ({ page }) => {
    await gotoAndWait(page);
    const preview = page.locator('[data-slot="preview"]').first();

    // Root pagination element
    const pagination = preview.locator('[data-slot="pagination"]');
    await expect(pagination).toBeAttached();

    // Navigation wrapper
    const content = preview.locator('[data-slot="pagination-content"]');
    await expect(content).toBeAttached();

    // Pagination items
    const items = preview.locator('[data-slot="pagination-item"]');
    const count = await items.count();
    expect(count).toBeGreaterThanOrEqual(4);
  });

  test("has previous and next links", async ({ page }) => {
    await gotoAndWait(page);
    const preview = page.locator('[data-slot="preview"]').first();

    const prev = preview.locator('[data-slot="pagination-previous"]');
    await expect(prev).toBeAttached();
    await expect(prev).toHaveAttribute("href", "#");

    const next = preview.locator('[data-slot="pagination-next"]');
    await expect(next).toBeAttached();
    await expect(next).toHaveAttribute("href", "#");
  });

  test("has page links", async ({ page }) => {
    await gotoAndWait(page);
    const preview = page.locator('[data-slot="preview"]').first();

    const links = preview.locator('[data-slot="pagination-link"]');
    const count = await links.count();
    expect(count).toBeGreaterThanOrEqual(3);
  });

  test("active link has is-active attribute", async ({ page }) => {
    await gotoAndWait(page);
    const preview = page.locator('[data-slot="preview"]').first();

    const activeLink = preview.locator('[data-slot="pagination-link"][data-active="true"]');
    await expect(activeLink).toBeAttached();
  });

  test("has ellipsis element", async ({ page }) => {
    await gotoAndWait(page);
    const preview = page.locator('[data-slot="preview"]').first();

    const ellipsis = preview.locator('[data-slot="pagination-ellipsis"]');
    await expect(ellipsis).toBeAttached();
  });

  test("renders as nav element with correct role", async ({ page }) => {
    await gotoAndWait(page);
    const preview = page.locator('[data-slot="preview"]').first();

    const pagination = preview.locator('[data-slot="pagination"]');
    const tagName = await pagination.evaluate(el => el.tagName.toLowerCase());
    expect(tagName).toBe("nav");
  });

  test("previous/next have correct text", async ({ page }) => {
    await gotoAndWait(page);
    const preview = page.locator('[data-slot="preview"]').first();

    const prev = preview.locator('[data-slot="pagination-previous"]');
    await expect(prev).toContainText("Previous");

    const next = preview.locator('[data-slot="pagination-next"]');
    await expect(next).toContainText("Next");
  });
});
