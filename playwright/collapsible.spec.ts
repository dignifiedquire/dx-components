import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/block?name=collapsible&variant=main&", { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const collapsible = page.locator('[data-slot="collapsible"]');
  await expect(collapsible).toBeVisible();

  const trigger = page.locator('[data-slot="collapsible-trigger"]');
  await expect(trigger).toBeVisible();

  const content = page.locator('[data-slot="collapsible-content"]');
  await expect(content).toHaveAttribute("data-open", "false");

  // Click trigger to open
  await trigger.click();
  await expect(content).toHaveAttribute("data-open", "true");
  await expect(collapsible).toHaveAttribute("data-open", "true");

  // Click trigger to close
  await trigger.click();
  await expect(content).toHaveAttribute("data-open", "false");
  await expect(collapsible).toHaveAttribute("data-open", "false");
});
