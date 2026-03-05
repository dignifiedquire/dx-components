import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/block?name=popover&variant=main&", { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const popover = page.locator('[data-slot="popover"]');
  await expect(popover).toBeVisible();

  const trigger = popover.locator('[data-slot="popover-trigger"]');
  await expect(trigger).toBeVisible();
  await expect(trigger).toHaveText("Show Popover");

  // Open the popover
  await trigger.click();

  const content = page.locator('[data-slot="popover-content"]');
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("data-state", "open");
  await expect(content).toHaveAttribute("data-side", "bottom");

  const contentClass = await content.getAttribute("class");
  expect(contentClass).toContain("z-50");
  expect(contentClass).toContain("rounded-md");
  expect(contentClass).toContain("border");
  expect(contentClass).toContain("bg-popover");
  expect(contentClass).toContain("shadow-md");

  // Focus trap: first focusable element should be focused
  const confirm = page.getByRole("button", { name: "Confirm" });
  const cancel = page.getByRole("button", { name: "Cancel" });
  await expect(confirm).toBeFocused();

  // Tab cycles within popover
  await page.keyboard.press("Tab");
  await expect(cancel).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(confirm).toBeFocused();

  // Enter on confirm should close popover and show deleted message
  await page.keyboard.press("Enter");
  await expect(page.locator("#component-preview-frame")).toContainText("Item deleted!");

  // Open again and test escape
  await trigger.click();
  await page.keyboard.press("Escape");
});
