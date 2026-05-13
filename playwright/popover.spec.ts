import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/popover";

test.describe("popover", () => {
  test("trigger accessibility attributes when closed", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    await expect(trigger).toBeVisible();
    await expect(trigger).toHaveAttribute("data-state", "closed");
    await expect(trigger).toHaveAttribute("aria-haspopup", "dialog");
  });

  test("opens in top layer with positioning attributes", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    await trigger.click();

    const content = page.locator('[data-slot="popover-content"]').first();
    await expect(content).toBeVisible();
    await expect(content).toHaveAttribute("role", "dialog");
    await expect(content).toHaveAttribute("data-state", "open");
    await expect(content).toHaveAttribute("data-side", "bottom");
    await expect(content).toHaveAttribute("data-align", "center");

    // The positioning wrapper carries `popover="auto"` and is in the top layer.
    const inTopLayer = await content.evaluate((el) => {
      const wrapper = el.closest("[data-radix-popper-content-wrapper]");
      return (
        wrapper instanceof HTMLElement &&
        wrapper.matches(":popover-open") &&
        wrapper.getAttribute("popover") === "auto"
      );
    });
    expect(inTopLayer).toBe(true);

    // Content has form fields
    await expect(content.locator("input")).toHaveCount(4);
  });

  test("calling hidePopover() syncs open signal back", async ({ page }) => {
    // CDP-synthesized ESC does not reliably trigger native popover light-
    // dismiss. We exercise the same code path by calling `hidePopover()`
    // on the wrapper directly — this fires the `toggle` event that
    // use_top_layer subscribes to.
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    await trigger.click();

    const content = page.locator('[data-slot="popover-content"]').first();
    await expect(content).toBeVisible();

    await page.evaluate(() => {
      const wrapper = document.querySelector(
        "[data-radix-popper-content-wrapper]",
      ) as HTMLElement;
      wrapper.hidePopover();
    });

    await expect(content).toHaveCount(0);
    await expect(trigger).toHaveAttribute("data-state", "closed");
  });

  test("trigger toggles open and closed", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    const content = page.locator('[data-slot="popover-content"]').first();

    await trigger.click();
    await expect(content).toBeVisible();

    await trigger.click();
    await expect(content).toHaveCount(0);
  });

  test("reopening after browser-initiated close works", async ({ page }) => {
    // If the signal hadn't synced after hidePopover, the next trigger
    // click would set open to `false` (no-op) instead of `true` (reopen).
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    const content = page.locator('[data-slot="popover-content"]').first();
    await trigger.click();
    await expect(content).toBeVisible();

    await page.evaluate(() => {
      const wrapper = document.querySelector(
        "[data-radix-popper-content-wrapper]",
      ) as HTMLElement;
      wrapper.hidePopover();
    });
    await expect(content).toHaveCount(0);

    await trigger.click();
    await expect(content).toBeVisible();
  });
});
