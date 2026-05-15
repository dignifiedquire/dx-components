import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const URL = "http://127.0.0.1:8080/docs/components/avatar";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (avatar rendered). */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Basic rendering
// ---------------------------------------------------------------------------

test.describe("avatar rendering", () => {
  test("renders multiple avatars", async ({ page }) => {
    await gotoAndWait(page);
    const avatars = page.locator('[data-slot="preview"] [data-slot="avatar"]');
    const count = await avatars.count();
    expect(count).toBeGreaterThanOrEqual(4);
  });

  test("avatar root is a span element", async ({ page }) => {
    await gotoAndWait(page);
    const avatar = page.locator('[data-slot="preview"] [data-slot="avatar"]').first();
    const tagName = await avatar.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("span");
  });
});

// ---------------------------------------------------------------------------
// Data attributes
// ---------------------------------------------------------------------------

test.describe("avatar data attributes", () => {
  test("avatar root has data-slot=avatar", async ({ page }) => {
    await gotoAndWait(page);
    const avatar = page.locator('[data-slot="preview"] [data-slot="avatar"]').first();
    await expect(avatar).toHaveAttribute("data-slot", "avatar");
  });

  test("avatar image has data-slot=avatar-image", async ({ page }) => {
    await gotoAndWait(page);
    const avatar = page.locator('[data-slot="preview"] [data-slot="avatar"]').first();
    const image = avatar.locator('[data-slot="avatar-image"]');
    await expect(image).toBeAttached();
    await expect(image).toHaveAttribute("data-slot", "avatar-image");
  });
});

// ---------------------------------------------------------------------------
// Image loading
// ---------------------------------------------------------------------------

test.describe("avatar image loading", () => {
  test("basic avatar image becomes visible when loaded", async ({ page }) => {
    await gotoAndWait(page);
    const avatar = page.locator('[data-slot="preview"] [data-slot="avatar"]').first();
    const image = avatar.locator('[data-slot="avatar-image"]');

    // Wait for image to load and become visible
    await expect(image).toBeVisible({ timeout: 10_000 });

    // Verify it is an img element
    const tagName = await image.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("img");
  });

  test("basic avatar image has a src attribute", async ({ page }) => {
    await gotoAndWait(page);
    const avatar = page.locator('[data-slot="preview"] [data-slot="avatar"]').first();
    const image = avatar.locator('[data-slot="avatar-image"]');
    const src = await image.getAttribute("src");
    expect(src).toBeTruthy();
  });
});

// ---------------------------------------------------------------------------
// Error state / fallback
// ---------------------------------------------------------------------------

// The shipped demo mirrors shadcn's avatar-demo (all valid image URLs), so
// no avatar is permanently in the fallback state. To exercise the real Radix
// fallback path we abort the avatar image requests, which keeps every
// AvatarImage from reaching `loaded` and forces AvatarFallback to render.
test.describe("avatar fallback", () => {
  async function gotoWithBrokenImages(page: import("@playwright/test").Page) {
    await page.route(/github\.com\/.*\.png/, (route) => route.abort());
    await gotoAndWait(page);
  }

  test("fallback renders when the image fails to load", async ({ page }) => {
    await gotoWithBrokenImages(page);
    const fallback = page
      .locator('[data-slot="preview"] [data-slot="avatar"]')
      .first()
      .locator('[data-slot="avatar-fallback"]');
    await expect(fallback).toBeVisible({ timeout: 10_000 });
    await expect(fallback).toContainText("CN");
  });

  test("fallback has data-slot=avatar-fallback and is a span", async ({
    page,
  }) => {
    await gotoWithBrokenImages(page);
    const fallback = page
      .locator('[data-slot="preview"] [data-slot="avatar"]')
      .first()
      .locator('[data-slot="avatar-fallback"]');
    await expect(fallback).toBeVisible({ timeout: 10_000 });
    await expect(fallback).toHaveAttribute("data-slot", "avatar-fallback");
    const tagName = await fallback.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("span");
  });

  test("image element is not rendered while in the fallback state", async ({
    page,
  }) => {
    await gotoWithBrokenImages(page);
    const firstAvatar = page
      .locator('[data-slot="preview"] [data-slot="avatar"]')
      .first();
    const fallback = firstAvatar.locator('[data-slot="avatar-fallback"]');
    await expect(fallback).toBeVisible({ timeout: 10_000 });
    // Radix only mounts AvatarImage once it has loaded; on abort it stays out.
    const image = firstAvatar.locator('[data-slot="avatar-image"]');
    if ((await image.count()) > 0) {
      await expect(image).not.toBeVisible();
    }
  });
});

// ---------------------------------------------------------------------------
// Accessibility
// ---------------------------------------------------------------------------

test.describe("avatar accessibility", () => {
  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="preview"]')
      .disableRules(["color-contrast"])
      .analyze();
    expect(results.violations).toEqual([]);
  });
});
