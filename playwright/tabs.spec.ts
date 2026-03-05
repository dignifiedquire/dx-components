import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=tabs&", { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const tabs = page.locator('[data-slot="tabs"]');
  await expect(tabs).toBeVisible();
  await expect(tabs).toHaveAttribute('data-orientation', 'horizontal');

  const tabsClass = await tabs.getAttribute('class');
  expect(tabsClass).toContain('flex');
  expect(tabsClass).toContain('gap-2');

  const list = tabs.locator('[data-slot="tabs-list"]');
  await expect(list).toBeVisible();
  const listClass = await list.getAttribute('class');
  expect(listClass).toContain('inline-flex');
  expect(listClass).toContain('rounded-lg');
  expect(listClass).toContain('bg-muted');

  const triggers = tabs.locator('[data-slot="tabs-trigger"]');
  await expect(triggers).toHaveCount(3);
  const triggerClass = await triggers.first().getAttribute('class');
  expect(triggerClass).toContain('rounded-md');
  expect(triggerClass).toContain('text-sm');

  const activeContent = tabs.locator('[data-slot="tabs-content"][data-state="active"]');

  const tab1Button = page.getByRole("tab", { name: "Tab 1" });
  const tab2Button = page.getByRole("tab", { name: "Tab 2" });
  const tab3Button = page.getByRole("tab", { name: "Tab 3" });

  // Clicking the right arrow should focus the next tab trigger
  await tab1Button.click();
  await page.keyboard.press("ArrowRight");
  await expect(tab2Button).toBeFocused();

  // Clicking enter should activate the focused tab
  await page.keyboard.press("Enter");
  await expect(activeContent).toContainText("Tab 2 Content");

  // Clicking right twice more should bring us back to the first tab
  await page.keyboard.press("ArrowRight");
  await expect(tab3Button).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(tab1Button).toBeFocused();

  // Clicking each tab should activate it
  await tab3Button.click();
  await expect(activeContent).toContainText("Tab 3 Content");
  await tab2Button.click();
  await expect(activeContent).toContainText("Tab 2 Content");
  await tab1Button.click();
  await expect(activeContent).toContainText("Tab 1 Content");
});
