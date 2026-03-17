import { test, expect } from "@playwright/test";

// Tooltip has a 700ms open delay. On slow CI machines (headless Firefox),
// WASM execution can be slower, so use a generous assertion timeout.
const EXPECT_TIMEOUT = { timeout: 15_000 };

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/tooltip", { timeout: 20 * 60 * 1000 });

  // Trigger is a button with correct attributes
  const trigger = page.locator('[data-slot="tooltip-trigger"]');
  await expect(trigger).toBeVisible();
  await expect(trigger).toHaveAttribute("data-state", "closed");

  // Wait for Tailwind CSS reflow to move the trigger into the grid's right
  // column. WebKit defers layout reflow for off-screen elements, so just
  // checking getComputedStyle isn't enough — we need the actual box position.
  await page.waitForFunction(
    (sel) => {
      const el = document.querySelector(sel);
      if (!el) return false;
      return el.getBoundingClientRect().x > 400;
    },
    '[data-slot="tooltip-trigger"]',
  );

  const tooltip = page.getByRole("tooltip");

  // Hovering shows tooltip — Playwright's hover() may not reliably dispatch
  // pointermove events on headless Firefox CI. Use page.mouse.move() with
  // explicit coordinates + dispatch a proper PointerEvent as fallback.
  await trigger.hover();
  const box = await trigger.boundingBox();
  if (box) {
    // Explicit mouse move to trigger center — generates real browser events
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  }
  // Dispatch a proper PointerEvent in case browser-level events didn't fire
  await trigger.evaluate((el) => {
    el.dispatchEvent(new PointerEvent("pointermove", { bubbles: true }));
    el.dispatchEvent(new MouseEvent("mousemove", { bubbles: true }));
  });
  await expect(tooltip).toBeVisible(EXPECT_TIMEOUT);
  await expect(tooltip).toHaveAttribute("data-slot", "tooltip-content");
  await expect(tooltip).toHaveAttribute("data-state", "open");
  await expect(tooltip).toHaveAttribute("data-side", "top");
  await expect(tooltip).toHaveAttribute("data-align", "center");

  // Moving mouse away hides tooltip
  await page.mouse.move(0, 0);
  await expect(tooltip).toHaveCount(0, EXPECT_TIMEOUT);

  // Focus shows tooltip
  await trigger.focus();
  await expect(tooltip).toBeVisible(EXPECT_TIMEOUT);

  // Trigger has aria-describedby when open
  const describedby = await trigger.getAttribute("aria-describedby");
  expect(describedby).toBeTruthy();

  // Escape closes
  await page.keyboard.press("Escape");
  await expect(tooltip).toHaveCount(0, EXPECT_TIMEOUT);
});
