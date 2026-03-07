import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/menubar", { timeout: 20 * 60 * 1000 });

  // Scope to the preview container
  const preview = page.locator('[data-slot="preview"]').first();
  await expect(preview).toBeVisible();

  // Menubar root
  const menubar = preview.locator('[data-slot="menubar"]');
  await expect(menubar).toBeVisible();
  await expect(menubar).toHaveAttribute("role", "menubar");

  // Triggers
  const triggers = menubar.locator('[data-slot="menubar-trigger"]');
  const triggerCount = await triggers.count();
  expect(triggerCount).toBeGreaterThan(0);

  const fileTrigger = triggers.first();
  await expect(fileTrigger).toHaveAttribute("role", "menuitem");
  await expect(fileTrigger).toHaveAttribute("aria-haspopup", "menu");
  await expect(fileTrigger).toHaveAttribute("data-state", "closed");

  // Click File trigger to open
  await fileTrigger.click();
  await expect(fileTrigger).toHaveAttribute("data-state", "open");
  await expect(fileTrigger).toHaveAttribute("aria-expanded", "true");

  // Content
  const content = page.locator('[data-slot="menubar-content"]');
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("role", "menu");
  await expect(content).toHaveAttribute("data-state", "open");
  await expect(content).toHaveAttribute("aria-orientation", "vertical");

  // Items have correct role
  const items = content.locator('[data-slot="menubar-item"]');
  const itemCount = await items.count();
  expect(itemCount).toBeGreaterThan(0);
  await expect(items.first()).toHaveAttribute("role", "menuitem");

  // Separator
  const separator = content.locator('[data-slot="menubar-separator"]');
  await expect(separator.first()).toHaveAttribute("role", "separator");

  // Group
  const group = content.locator('[data-slot="menubar-group"]');
  await expect(group.first()).toHaveAttribute("role", "group");

  // Shortcut
  const shortcut = content.locator('[data-slot="menubar-shortcut"]');
  await expect(shortcut.first()).toBeVisible();

  // Escape closes menu
  await page.keyboard.press("Escape");
  await expect(content).toHaveCount(0);
  await expect(fileTrigger).toHaveAttribute("data-state", "closed");

  // Reopen File, hover Edit to switch
  await fileTrigger.click();
  await expect(content).toBeVisible();

  const editTrigger = triggers.nth(1);
  await editTrigger.hover();
  await expect(editTrigger).toHaveAttribute("data-state", "open");

  // Escape to close
  await page.keyboard.press("Escape");
  await expect(content).toHaveCount(0);
});
