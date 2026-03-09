import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/hover_card", { timeout: 20 * 60 * 1000 });

  // Trigger is an anchor element with correct attributes
  const trigger = page.locator('[data-slot="hover-card-trigger"]');
  await expect(trigger).toBeVisible();
  await expect(trigger).toHaveAttribute("data-state", "closed");

  const content = page.locator('[data-slot="hover-card-content"]');

  // Hovering shows content
  await trigger.hover();
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("data-state", "open");
  await expect(content).toHaveAttribute("data-side", "bottom");
  await expect(content).toHaveAttribute("data-align", "center");

  // Moving mouse away hides content
  await page.mouse.move(0, 0);
  await expect(content).toHaveCount(0);

  // Focus shows content
  await trigger.focus();
  await expect(content).toBeVisible();

  // Blur hides content
  await trigger.blur();
  await expect(content).toHaveCount(0);
});
