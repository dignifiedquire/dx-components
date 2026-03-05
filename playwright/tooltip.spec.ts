import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=tooltip&", { timeout: 20 * 60 * 1000 });

  const tooltipRoot = page.locator('[data-slot="tooltip"]');
  await expect(tooltipRoot).toBeVisible();

  const trigger = page.locator('[data-slot="tooltip-trigger"]');
  await expect(trigger).toBeVisible();

  const tooltip = page.getByRole("tooltip");

  // tabbing to the trigger element should show the tooltip
  await page.locator("#component-preview-frame").focus();
  await page.keyboard.press("Tab");
  await expect(tooltip).toBeVisible();

  // Tooltip content has correct data-slot and classes
  const content = page.locator('[data-slot="tooltip-content"]');
  await expect(content).toBeVisible();
  const contentClass = await content.getAttribute('class');
  expect(contentClass).toContain('rounded-md');
  expect(contentClass).toContain('text-xs');
  expect(contentClass).toContain('z-50');

  // tabbing out of the trigger element should hide the tooltip
  await page.keyboard.press("Tab");
  await expect(tooltip).toHaveCount(0);

  // hovering over the trigger element should show the tooltip
  await trigger.hover();
  await expect(tooltip).toBeVisible();

  // moving the mouse away from the trigger element should hide the tooltip
  await page.mouse.move(0, 0);
  await expect(tooltip).toHaveCount(0);
});
