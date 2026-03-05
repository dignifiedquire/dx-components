import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/block?name=hover_card&variant=main&", { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const hoverCard = page.locator('[data-slot="hover-card"]');
  await expect(hoverCard).toBeVisible();

  const trigger = hoverCard.locator('[data-slot="hover-card-trigger"]');
  await expect(trigger).toBeVisible();

  // tabbing to the trigger element should show the content
  await page.locator("#component-preview-frame").focus();
  await page.keyboard.press("Tab");
  const content = page.locator('[data-slot="hover-card-content"]');
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("data-state", "open");
  await expect(content).toHaveAttribute("data-side", "bottom");

  const contentClass = await content.getAttribute("class");
  expect(contentClass).toContain("z-50");
  expect(contentClass).toContain("rounded-md");
  expect(contentClass).toContain("border");
  expect(contentClass).toContain("bg-popover");
  expect(contentClass).toContain("shadow-md");

  // tabbing out of the trigger element should hide the content
  await page.keyboard.press("Tab");
  await expect(content).toHaveCount(0);

  // hovering over the trigger element should show the content
  await trigger.hover();
  await expect(content).toBeVisible();

  // moving the mouse away should hide it
  await page.mouse.move(0, 0);
  await expect(content).toHaveCount(0);
});
