import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/block?name=accordion&variant=main&", { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const accordion = page.locator('[data-slot="accordion"]');
  await expect(accordion).toBeVisible();

  const items = accordion.locator('[data-slot="accordion-item"]');
  await expect(items).toHaveCount(4);

  const firstItem = items.first();
  const firstItemClass = await firstItem.getAttribute("class");
  expect(firstItemClass).toContain("border-b");

  // Click first item trigger
  const firstTrigger = firstItem.locator('[data-slot="accordion-trigger"]');
  await firstTrigger.click();
  await expect(firstItem).toHaveAttribute("data-open", "true");

  const triggerClass = await firstTrigger.getAttribute("class");
  expect(triggerClass).toContain("flex");
  expect(triggerClass).toContain("text-sm");
  expect(triggerClass).toContain("font-medium");

  // Click second item - first should close (single mode)
  const secondItem = items.nth(1);
  const secondTrigger = secondItem.locator('[data-slot="accordion-trigger"]');
  await secondTrigger.click();
  await expect(secondItem).toHaveAttribute("data-open", "true");
  await expect(firstItem).toHaveAttribute("data-open", "false");
});
