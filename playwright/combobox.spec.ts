import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/combobox";

test.describe("combobox", () => {
  test("input renders with ARIA attributes", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const input = page.locator('[data-slot="combobox-input"]').first();
    await expect(input).toBeVisible();
    await expect(input).toHaveAttribute("role", "combobox");
    await expect(input).toHaveAttribute("aria-expanded", "false");
    await expect(input).toHaveAttribute("aria-autocomplete", "list");
  });

  test("opens in top layer positioned below the input", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const input = page.locator('[data-slot="combobox-input"]').first();
    await input.focus();

    const content = page.locator('[data-slot="combobox-content"]').first();
    await expect(content).toBeVisible();
    await expect(content).toHaveAttribute("data-state", "open");

    // The Popper wrapper carries `popover="manual"` and is in the top layer.
    const status = await content.evaluate((el) => {
      const wrapper = el.closest("[data-radix-popper-content-wrapper]");
      return {
        popoverAttr:
          wrapper instanceof HTMLElement ? wrapper.getAttribute("popover") : null,
        popoverOpen:
          wrapper instanceof HTMLElement
            ? wrapper.matches(":popover-open")
            : false,
      };
    });
    expect(status.popoverAttr).toBe("manual");
    expect(status.popoverOpen).toBe(true);

    // Dropdown is anchored below the input (floating-ui positioning).
    // Floating-ui computes position asynchronously, so wait until the
    // wrapper has a non-zero transform applied before measuring.
    await page
      .locator("[data-radix-popper-content-wrapper]")
      .first()
      .evaluate((el) =>
        new Promise<void>((resolve) => {
          const check = () => {
            if ((el as HTMLElement).style.transform.includes("translate")) {
              resolve();
            } else {
              requestAnimationFrame(check);
            }
          };
          check();
        }),
      );
    const positioning = await page.evaluate(() => {
      const input = document.querySelector(
        '[data-slot="combobox-input"]',
      ) as HTMLElement;
      const content = document.querySelector(
        '[data-slot="combobox-content"]',
      ) as HTMLElement;
      const i = input.getBoundingClientRect();
      const c = content.getBoundingClientRect();
      return { inputBottom: i.bottom, contentTop: c.top };
    });
    // Allow a small offset (Popper may add side_offset)
    expect(Math.abs(positioning.contentTop - positioning.inputBottom)).toBeLessThan(
      8,
    );
  });

  test("typing filters items", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const input = page.locator('[data-slot="combobox-input"]').first();
    await input.focus();
    await input.fill("re");

    const items = page.locator('[data-slot="combobox-item"]');
    await expect(items).toHaveCount(1);
    await expect(items.first()).toContainText("Remix");
  });

  test("selecting an item closes and unmounts content", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const input = page.locator('[data-slot="combobox-input"]').first();
    await input.focus();

    const content = page.locator('[data-slot="combobox-content"]').first();
    await expect(content).toBeVisible();

    await page
      .locator('[data-slot="combobox-item"]')
      .filter({ hasText: "Astro" })
      .first()
      .click();

    await expect(content).toHaveCount(0);
    await expect(input).toHaveValue("Astro");
  });
});
