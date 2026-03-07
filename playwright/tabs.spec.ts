import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/tabs", { timeout: 20 * 60 * 1000 });

  // Find the first tabs component (the demo, not installation tabs)
  const tabs = page.locator('[data-slot="tabs"]').first();
  await expect(tabs).toBeVisible();
  await expect(tabs).toHaveAttribute('data-orientation', 'horizontal');

  const list = tabs.locator('[data-slot="tabs-list"]');
  await expect(list).toBeVisible();
  await expect(list).toHaveAttribute('role', 'tablist');

  const triggers = tabs.locator('[data-slot="tabs-trigger"]');
  await expect(triggers).toHaveCount(2);

  const accountTab = tabs.getByRole("tab", { name: "Account" });
  const passwordTab = tabs.getByRole("tab", { name: "Password" });

  // Account tab is selected by default
  await expect(accountTab).toHaveAttribute('data-state', 'active');
  await expect(accountTab).toHaveAttribute('aria-selected', 'true');
  await expect(passwordTab).toHaveAttribute('data-state', 'inactive');

  // Account content is visible
  const activeContent = tabs.locator('[data-slot="tabs-content"][data-state="active"]');
  await expect(activeContent).toContainText("Account");

  // Clicking Password tab should activate it
  await passwordTab.click();
  await expect(passwordTab).toHaveAttribute('data-state', 'active');
  await expect(accountTab).toHaveAttribute('data-state', 'inactive');
  await expect(activeContent).toContainText("Password");

  // Clicking Account tab should switch back
  await accountTab.click();
  await expect(accountTab).toHaveAttribute('data-state', 'active');
  await expect(activeContent).toContainText("Account");

  // Keyboard navigation: ArrowRight should focus Password tab
  await accountTab.click();
  await page.keyboard.press("ArrowRight");
  await expect(passwordTab).toBeFocused();

  // ArrowRight again should loop back to Account (loop=true by default)
  await page.keyboard.press("ArrowRight");
  await expect(accountTab).toBeFocused();

  // ArrowLeft should go to Password
  await page.keyboard.press("ArrowLeft");
  await expect(passwordTab).toBeFocused();
});
