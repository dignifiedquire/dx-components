import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const MAIN = "http://127.0.0.1:8080/component/block/accordion/main";
const MULTIPLE = "http://127.0.0.1:8080/component/block/accordion/multiple";
const DISABLED = "http://127.0.0.1:8080/component/block/accordion/disabled";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (accordion root rendered). */
async function gotoAndWait(page: import("@playwright/test").Page, url: string) {
  await page.goto(url, TIMEOUT);
  await page.locator('[data-slot="accordion"]').waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Single accordion (main variant)
// Ported from radix-ui-primitives/packages/react/accordion/src/accordion.test.tsx
// ---------------------------------------------------------------------------

test.describe("single accordion", () => {
  test("renders all items", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const items = page.locator('[data-slot="collapsible"]');
    await expect(items).toHaveCount(3);
  });

  test("first item is default open", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const items = page.locator('[data-slot="collapsible"]');
    await expect(items.first()).toHaveAttribute("data-state", "open");
    const content = items.first().locator('[data-slot="collapsible-content"]');
    await expect(content).toBeVisible();
  });

  test("click trigger shows content", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    await triggers.nth(1).click();

    const items = page.locator('[data-slot="collapsible"]');
    await expect(items.nth(1)).toHaveAttribute("data-state", "open");
    const content = items.nth(1).locator('[data-slot="collapsible-content"]');
    await expect(content).toBeVisible();
  });

  test("click hides previous in single mode", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const items = page.locator('[data-slot="collapsible"]');
    const triggers = page.locator('[data-slot="collapsible-trigger"]');

    // First item starts open
    await expect(items.first()).toHaveAttribute("data-state", "open");

    // Click second trigger
    await triggers.nth(1).click();

    // Second is now open, first is closed
    await expect(items.nth(1)).toHaveAttribute("data-state", "open");
    await expect(items.first()).toHaveAttribute("data-state", "closed");
  });

  test("click open trigger again collapses it", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const items = page.locator('[data-slot="collapsible"]');
    const triggers = page.locator('[data-slot="collapsible-trigger"]');

    // First item starts open
    await expect(items.first()).toHaveAttribute("data-state", "open");

    // Click first trigger again to collapse
    await triggers.first().click();
    await expect(items.first()).toHaveAttribute("data-state", "closed");
  });

  // --- Keyboard navigation (matching Radix accordion.test.tsx) ---

  test("ArrowDown moves focus to next trigger", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    await triggers.first().focus();
    await page.keyboard.press("ArrowDown");
    await expect(triggers.nth(1)).toBeFocused();
  });

  test("ArrowDown wraps to first trigger", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    await triggers.nth(2).focus();
    await page.keyboard.press("ArrowDown");
    await expect(triggers.first()).toBeFocused();
  });

  test("ArrowUp moves focus to previous trigger", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    await triggers.nth(2).focus();
    await page.keyboard.press("ArrowUp");
    await expect(triggers.nth(1)).toBeFocused();
  });

  test("ArrowUp wraps to last trigger", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    await triggers.first().focus();
    await page.keyboard.press("ArrowUp");
    await expect(triggers.nth(2)).toBeFocused();
  });

  test("Home focuses first trigger", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    await triggers.nth(1).focus();
    await page.keyboard.press("Home");
    await expect(triggers.first()).toBeFocused();
  });

  test("End focuses last trigger", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    await triggers.first().focus();
    await page.keyboard.press("End");
    await expect(triggers.nth(2)).toBeFocused();
  });

  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="accordion"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// Multiple accordion (multiple variant)
// ---------------------------------------------------------------------------

test.describe("multiple accordion", () => {
  test("items 1 and 2 are default open", async ({ page }) => {
    await gotoAndWait(page, MULTIPLE);
    const items = page.locator('[data-slot="collapsible"]');
    await expect(items.nth(0)).toHaveAttribute("data-state", "open");
    await expect(items.nth(1)).toHaveAttribute("data-state", "open");
    await expect(items.nth(2)).toHaveAttribute("data-state", "closed");
  });

  test("click opens additional item without closing others", async ({ page }) => {
    await gotoAndWait(page, MULTIPLE);
    const items = page.locator('[data-slot="collapsible"]');
    const triggers = page.locator('[data-slot="collapsible-trigger"]');

    // Click third trigger
    await triggers.nth(2).click();

    // All three should be open
    await expect(items.nth(0)).toHaveAttribute("data-state", "open");
    await expect(items.nth(1)).toHaveAttribute("data-state", "open");
    await expect(items.nth(2)).toHaveAttribute("data-state", "open");
  });

  test("click open trigger closes only that item", async ({ page }) => {
    await gotoAndWait(page, MULTIPLE);
    const items = page.locator('[data-slot="collapsible"]');
    const triggers = page.locator('[data-slot="collapsible-trigger"]');

    // Close first item
    await triggers.nth(0).click();

    // First closed, second still open
    await expect(items.nth(0)).toHaveAttribute("data-state", "closed");
    await expect(items.nth(1)).toHaveAttribute("data-state", "open");
  });

  test("keyboard navigation works", async ({ page }) => {
    await gotoAndWait(page, MULTIPLE);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    await triggers.first().focus();
    await page.keyboard.press("ArrowDown");
    await expect(triggers.nth(1)).toBeFocused();
  });

  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page, MULTIPLE);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="accordion"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// Disabled accordion (disabled variant)
// ---------------------------------------------------------------------------

test.describe("disabled accordion", () => {
  test("triggers are disabled", async ({ page }) => {
    await gotoAndWait(page, DISABLED);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    const count = await triggers.count();
    for (let i = 0; i < count; i++) {
      await expect(triggers.nth(i)).toBeDisabled();
    }
  });

  test("click does nothing when disabled", async ({ page }) => {
    await gotoAndWait(page, DISABLED);
    const triggers = page.locator('[data-slot="collapsible-trigger"]');
    const items = page.locator('[data-slot="collapsible"]');

    // Try clicking first trigger
    await triggers.first().click({ force: true });

    // All items should remain closed
    const count = await items.count();
    for (let i = 0; i < count; i++) {
      await expect(items.nth(i)).toHaveAttribute("data-state", "closed");
    }
  });

  test("no accessibility violations", async ({ page }) => {
    await gotoAndWait(page, DISABLED);
    const results = await new AxeBuilder({ page })
      .include('[data-slot="accordion"]')
      .analyze();
    expect(results.violations).toEqual([]);
  });
});

// ---------------------------------------------------------------------------
// Styled classes (shadcn match)
// ---------------------------------------------------------------------------

test.describe("styled classes", () => {
  test("item has correct classes", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const item = page.locator('[data-slot="collapsible"]').first();
    const cls = await item.getAttribute("class");
    expect(cls).toContain("border-b");
    expect(cls).toContain("last:border-b-0");
  });

  test("trigger has shadcn classes", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const trigger = page.locator('[data-slot="collapsible-trigger"]').first();
    const cls = await trigger.getAttribute("class");
    expect(cls).toContain("flex");
    expect(cls).toContain("flex-1");
    expect(cls).toContain("items-start");
    expect(cls).toContain("justify-between");
    expect(cls).toContain("gap-4");
    expect(cls).toContain("rounded-md");
    expect(cls).toContain("py-4");
    expect(cls).toContain("text-left");
    expect(cls).toContain("text-sm");
    expect(cls).toContain("font-medium");
    expect(cls).toContain("transition-all");
    expect(cls).toContain("hover:underline");
    expect(cls).toContain("disabled:pointer-events-none");
    expect(cls).toContain("disabled:opacity-50");
  });

  test("trigger header is h3 with class flex", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const header = page.locator('[data-slot="accordion-header"]').first();
    const tagName = await header.evaluate((el) => el.tagName.toLowerCase());
    expect(tagName).toBe("h3");
    const cls = await header.getAttribute("class");
    expect(cls).toContain("flex");
  });

  test("chevron SVG present with correct classes", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const trigger = page.locator('[data-slot="collapsible-trigger"]').first();
    const svg = trigger.locator("svg");
    await expect(svg).toBeVisible();
    const cls = await svg.getAttribute("class");
    expect(cls).toContain("pointer-events-none");
    expect(cls).toContain("size-4");
    expect(cls).toContain("shrink-0");
    expect(cls).toContain("translate-y-0.5");
    expect(cls).toContain("text-muted-foreground");
    expect(cls).toContain("transition-transform");
    expect(cls).toContain("duration-200");

    // Chevron path
    const path = svg.locator("path");
    await expect(path).toHaveAttribute("d", "m6 9 6 6 6-6");
  });

  test("content has animation classes when open", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    // First item is default open
    const content = page.locator('[data-slot="collapsible-content"]').first();
    const cls = await content.getAttribute("class");
    expect(cls).toContain("overflow-hidden");
    expect(cls).toContain("text-sm");
  });

  test("content inner div has padding classes", async ({ page }) => {
    await gotoAndWait(page, MAIN);
    const content = page.locator('[data-slot="collapsible-content"]').first();
    // Inner wrapper div (first child of measurement div)
    const innerDiv = content.locator("div > div").first();
    const cls = await innerDiv.getAttribute("class");
    expect(cls).toContain("pt-0");
    expect(cls).toContain("pb-4");
  });
});
