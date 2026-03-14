import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/slot";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });
}

// ---------------------------------------------------------------------------
// Upstream: "given a slotted Trigger — with onClick on itself"
// (slot.test.tsx L9-25)
// ---------------------------------------------------------------------------

test.describe("slotted Trigger: onClick on itself", () => {
  test("should call the onClick passed to the Trigger", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="trigger-self-click"]');
    const button = section.locator("button");
    await button.click();
    const count = section.locator('[data-testid="trigger-self-count"]');
    await expect(count).toContainText("Trigger clicks: 1");
  });
});

// ---------------------------------------------------------------------------
// Upstream: "given a slotted Trigger — with onClick on the child"
// (slot.test.tsx L27-45)
// ---------------------------------------------------------------------------

test.describe("slotted Trigger: onClick on the child", () => {
  test("should call the child's onClick", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="child-click"]');
    const button = section.locator("button");
    await button.click();
    const count = section.locator('[data-testid="child-click-count"]');
    await expect(count).toContainText("Child clicks: 1");
  });
});

// ---------------------------------------------------------------------------
// Upstream: "given a slotted Trigger — with onClick on itself AND the child"
// (slot.test.tsx L47-71)
// In Dioxus, event handlers compose at the component level.
// ---------------------------------------------------------------------------

test.describe("slotted Trigger: click handler fires", () => {
  test("should increment click counter", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="both-click"]');
    const button = section.locator("button");
    await button.click();
    const count = section.locator('[data-testid="both-click-count"]');
    await expect(count).toContainText("Clicks: 1");
  });

  test("multiple clicks accumulate", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="both-click"]');
    const button = section.locator("button");
    await button.click();
    await button.click();
    await button.click();
    const count = section.locator('[data-testid="both-click-count"]');
    await expect(count).toContainText("Clicks: 3");
  });
});

// ---------------------------------------------------------------------------
// Upstream: "given a Button with Slottable — without asChild"
// (slot.test.tsx L114-126)
// "should render a button with icon on the left/right"
// ---------------------------------------------------------------------------

test.describe("Button with Slottable: without asChild", () => {
  test("renders a button element", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="button-no-aschild"]');
    const button = section.locator("button");
    await expect(button).toBeVisible();
  });

  test("has icon on the left", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="button-no-aschild"]');
    const iconLeft = section.locator('[data-testid="icon-left"]');
    await expect(iconLeft).toBeVisible();
    await expect(iconLeft).toContainText("L");
  });

  test("has icon on the right", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="button-no-aschild"]');
    const iconRight = section.locator('[data-testid="icon-right"]');
    await expect(iconRight).toBeVisible();
    await expect(iconRight).toContainText("R");
  });

  test("icons are in correct order (left, content, right)", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="button-no-aschild"]');
    const button = section.locator("button");
    const text = await button.innerHTML();
    const leftIdx = text.indexOf("L");
    const contentIdx = text.indexOf("text");
    const rightIdx = text.indexOf("R");
    expect(leftIdx).toBeLessThan(contentIdx);
    expect(contentIdx).toBeLessThan(rightIdx);
  });
});

// ---------------------------------------------------------------------------
// Upstream: "given a Button with Slottable — with asChild"
// (slot.test.tsx L128-140)
// "should render a link with icon on the left/right"
// ---------------------------------------------------------------------------

test.describe("Button with Slottable: with asChild", () => {
  test("renders an anchor element, not a button", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="button-aschild"]');
    const link = section.locator("a");
    await expect(link).toBeVisible();
    // Should NOT have a button element
    const buttons = await section.locator("button").count();
    expect(buttons).toBe(0);
  });

  test("anchor has href attribute", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="button-aschild"]');
    const link = section.locator("a");
    await expect(link).toHaveAttribute("href", "#slot-link");
  });

  test("renders callback content", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="button-aschild"]');
    const link = section.locator("a");
    await expect(link).toContainText("Link text");
  });
});

// ---------------------------------------------------------------------------
// Attribute forwarding through slot
// (Upstream: verifying data-*/aria-* forwarded through Slot)
// ---------------------------------------------------------------------------

test.describe("attribute forwarding", () => {
  test("data-state is forwarded through slot", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="attr-forwarding"]');
    const button = section.locator("button");
    await expect(button).toHaveAttribute("data-state", "open");
  });

  test("aria-expanded is forwarded through slot", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="attr-forwarding"]');
    const button = section.locator("button");
    await expect(button).toHaveAttribute("aria-expanded", "true");
  });

  test("button's own type attribute is present", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="attr-forwarding"]');
    const button = section.locator("button");
    await expect(button).toHaveAttribute("type", "button");
  });
});
