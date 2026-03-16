import { test, expect } from "@playwright/test";

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

  // Hovering shows tooltip
  await trigger.hover();
  await expect(tooltip).toBeVisible();
  await expect(tooltip).toHaveAttribute("data-slot", "tooltip-content");
  await expect(tooltip).toHaveAttribute("data-state", "open");
  await expect(tooltip).toHaveAttribute("data-side", "top");
  await expect(tooltip).toHaveAttribute("data-align", "center");

  // Moving mouse away hides tooltip
  await page.mouse.move(0, 0);
  await expect(tooltip).toHaveCount(0);

  // Focus shows tooltip
  await trigger.focus();
  await expect(tooltip).toBeVisible();

  // Trigger has aria-describedby when open
  const describedby = await trigger.getAttribute("aria-describedby");
  expect(describedby).toBeTruthy();

  // Escape closes
  await page.keyboard.press("Escape");
  await expect(tooltip).toHaveCount(0);
});
