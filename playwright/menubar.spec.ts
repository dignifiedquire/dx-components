import { test, expect } from "@playwright/test";

test("pointer navigation", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=menubar&", { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const menubar = page.locator('[data-slot="menubar"]');
  await expect(menubar).toBeVisible();

  const menubarClass = await menubar.getAttribute('class');
  expect(menubarClass).toContain('flex');
  expect(menubarClass).toContain('rounded-md');
  expect(menubarClass).toContain('border');
  expect(menubarClass).toContain('bg-background');
  expect(menubarClass).toContain('shadow-xs');

  const fileMenu = page.locator('[data-slot="menubar-menu"]').first();
  const fileTrigger = fileMenu.locator('[data-slot="menubar-trigger"]');
  await expect(fileTrigger).toBeVisible();

  const triggerClass = await fileTrigger.getAttribute('class');
  expect(triggerClass).toContain('rounded-sm');
  expect(triggerClass).toContain('text-sm');
  expect(triggerClass).toContain('font-medium');

  await fileTrigger.click();
  const fileContent = fileMenu.locator('[data-slot="menubar-content"]');
  await expect(fileContent).toHaveAttribute("data-state", "open");

  const contentClass = await fileContent.getAttribute('class');
  expect(contentClass).toContain('z-50');
  expect(contentClass).toContain('rounded-md');
  expect(contentClass).toContain('border');
  expect(contentClass).toContain('bg-popover');
  expect(contentClass).toContain('shadow-md');

  // Hover over Edit menu to switch
  const editMenu = page.locator('[data-slot="menubar-menu"]').nth(1);
  const editTrigger = editMenu.locator('[data-slot="menubar-trigger"]');
  await editTrigger.hover();
  const editContent = editMenu.locator('[data-slot="menubar-content"]');
  await expect(editContent).toHaveAttribute("data-state", "open");
  await expect(fileContent).toHaveCount(0);

  // Assert items have data-slot and classes
  const items = editContent.locator('[data-slot="menubar-item"]');
  await expect(items).toHaveCount(3);

  const itemClass = await items.first().getAttribute('class');
  expect(itemClass).toContain('flex');
  expect(itemClass).toContain('cursor-default');
  expect(itemClass).toContain('rounded-sm');
  expect(itemClass).toContain('text-sm');

  // Click Cut item
  const cutItem = editContent.getByRole("menuitem", { name: "Cut" });
  await cutItem.click();
  await expect(fileContent).toHaveCount(0);
});

test("keyboard navigation", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=menubar&", { timeout: 20 * 60 * 1000 });

  const menubar = page.locator('[data-slot="menubar"]');
  await menubar.focus();

  const fileMenu = page.locator('[data-slot="menubar-menu"]').first();
  const fileTrigger = fileMenu.locator('[data-slot="menubar-trigger"]');

  // Go right with the keyboard
  await page.keyboard.press("ArrowRight");
  const editMenu = page.locator('[data-slot="menubar-menu"]').nth(1);
  const editTrigger = editMenu.locator('[data-slot="menubar-trigger"]');
  await expect(editTrigger).toBeFocused();

  // Go left with the keyboard
  await page.keyboard.press("ArrowLeft");
  await expect(fileTrigger).toBeFocused();

  // Open the File menu
  await page.keyboard.press("ArrowDown");
  const fileContent = fileMenu.locator('[data-slot="menubar-content"]');
  await expect(fileContent).toHaveAttribute("data-state", "open");

  // Assert the new item is focused
  const newItem = fileContent.getByRole("menuitem", { name: "New" });
  await expect(newItem).toBeFocused();

  // Select New item
  await page.keyboard.press("Enter");
  await expect(fileContent).toHaveCount(0);
});
