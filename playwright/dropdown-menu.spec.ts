import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/dropdown_menu", { timeout: 20 * 60 * 1000 });

  // Trigger
  const trigger = page.locator('[data-slot="dropdown-menu-trigger"]').first();
  await expect(trigger).toBeVisible();
  await expect(trigger).toHaveAttribute("data-state", "closed");
  await expect(trigger).toHaveAttribute("aria-haspopup", "menu");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");

  // Open menu
  await trigger.click();
  await expect(trigger).toHaveAttribute("data-state", "open");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");

  // Content
  const content = page.locator('[data-slot="dropdown-menu-content"]');
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("role", "menu");
  await expect(content).toHaveAttribute("data-state", "open");
  await expect(content).toHaveAttribute("aria-orientation", "vertical");

  // Items have correct role
  const items = content.locator('[data-slot="dropdown-menu-item"]');
  const count = await items.count();
  expect(count).toBeGreaterThan(0);
  await expect(items.first()).toHaveAttribute("role", "menuitem");

  // Separator
  const separator = content.locator('[data-slot="dropdown-menu-separator"]');
  await expect(separator.first()).toHaveAttribute("role", "separator");

  // Label
  const label = content.locator('[data-slot="dropdown-menu-label"]');
  await expect(label.first()).toBeVisible();

  // Group
  const group = content.locator('[data-slot="dropdown-menu-group"]');
  await expect(group.first()).toHaveAttribute("role", "group");

  // Shortcut
  const shortcut = content.locator('[data-slot="dropdown-menu-shortcut"]');
  await expect(shortcut.first()).toBeVisible();

  // Escape closes menu
  await page.keyboard.press("Escape");
  await expect(content).toHaveCount(0);
  await expect(trigger).toHaveAttribute("data-state", "closed");

  // Reopen and toggle
  await trigger.click();
  await expect(content).toBeVisible();
  await trigger.click();
  await expect(content).toHaveCount(0);
});
