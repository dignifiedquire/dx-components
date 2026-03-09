import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/popover", { timeout: 20 * 60 * 1000 });

  // Trigger
  const trigger = page.locator('[data-slot="popover-trigger"]');
  await expect(trigger).toBeVisible();
  await expect(trigger).toHaveAttribute("data-state", "closed");
  await expect(trigger).toHaveAttribute("aria-haspopup", "dialog");

  // Open
  await trigger.click();

  // Content
  const content = page.locator('[data-slot="popover-content"]');
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("role", "dialog");
  await expect(content).toHaveAttribute("data-state", "open");
  await expect(content).toHaveAttribute("data-side", "bottom");
  await expect(content).toHaveAttribute("data-align", "center");

  // Content has form fields
  await expect(content.locator("input")).toHaveCount(4);

  // Escape closes
  await page.keyboard.press("Escape");
  await expect(content).toHaveCount(0);

  // Reopen and test trigger toggle
  await trigger.click();
  await expect(content).toBeVisible();
  await trigger.click();
  await expect(content).toHaveCount(0);
});
