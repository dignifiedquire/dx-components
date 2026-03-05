import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=toolbar&", { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const toolbar = page.locator('[data-slot="toolbar"]');
  await expect(toolbar).toBeVisible();

  const toolbarClass = await toolbar.getAttribute('class');
  expect(toolbarClass).toContain('flex');
  expect(toolbarClass).toContain('rounded-md');
  expect(toolbarClass).toContain('border');
  expect(toolbarClass).toContain('bg-background');

  // Assert buttons have data-slot and classes
  const buttons = toolbar.locator('[data-slot="toolbar-button"]');
  await expect(buttons).toHaveCount(6);

  const btnClass = await buttons.first().getAttribute('class');
  expect(btnClass).toContain('inline-flex');
  expect(btnClass).toContain('rounded-md');
  expect(btnClass).toContain('text-sm');
  expect(btnClass).toContain('font-medium');

  // Assert separator
  const separator = toolbar.locator('[data-slot="toolbar-separator"]');
  await expect(separator).toBeVisible();

  const sepClass = await separator.getAttribute('class');
  expect(sepClass).toContain('bg-border');

  // Keyboard navigation
  let bold = page.getByRole("button", { name: "Bold" });
  let italic = page.getByRole("button", { name: "Italic" });
  let underline = page.getByRole("button", { name: "Underline" });
  let alignLeft = page.getByRole("button", { name: "Align Left" });
  let alignCenter = page.getByRole("button", { name: "Align Center" });
  let alignRight = page.getByRole("button", { name: "Align Right" });

  await page.locator("#component-preview-frame").focus();
  await page.keyboard.press("Tab");
  await expect(bold).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(italic).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(underline).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(alignLeft).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(alignCenter).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(alignRight).toBeFocused();
});
