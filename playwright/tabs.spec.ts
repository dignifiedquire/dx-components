import { test, expect } from "@playwright/test";

test("data slots and classes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/tabs", { timeout: 20 * 60 * 1000 });

  const tabs = page.locator('[data-slot="tabs"]').first();
  await expect(tabs).toBeVisible();

  // Root data-slot and classes
  await expect(tabs).toHaveAttribute("data-slot", "tabs");
  await expect(tabs).toHaveAttribute("data-orientation", "horizontal");
  const tabsClass = await tabs.getAttribute("class");
  expect(tabsClass).toContain("flex");
  expect(tabsClass).toContain("gap-2");

  // TabsList data-slot and classes
  const list = tabs.locator('[data-slot="tabs-list"]');
  await expect(list).toBeVisible();
  await expect(list).toHaveAttribute("role", "tablist");
  const listClass = await list.getAttribute("class");
  expect(listClass).toContain("inline-flex");
  expect(listClass).toContain("rounded-lg");
  expect(listClass).toContain("bg-muted");

  // TabsTrigger data-slot and classes
  const triggers = tabs.locator('[data-slot="tabs-trigger"]');
  await expect(triggers).toHaveCount(2);
  const triggerClass = await triggers.first().getAttribute("class");
  expect(triggerClass).toContain("text-sm");
  expect(triggerClass).toContain("font-medium");
  expect(triggerClass).toContain("rounded-md");

  // TabsContent data-slot and classes
  const contents = tabs.locator('[data-slot="tabs-content"]');
  await expect(contents.first()).toBeVisible();
  const contentClass = await contents.first().getAttribute("class");
  expect(contentClass).toContain("outline-none");
});

test("tab activation and content switching", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/tabs", { timeout: 20 * 60 * 1000 });

  const tabs = page.locator('[data-slot="tabs"]').first();
  const accountTab = tabs.getByRole("tab", { name: "Account" });
  const passwordTab = tabs.getByRole("tab", { name: "Password" });

  // Account tab is active by default
  await expect(accountTab).toHaveAttribute("data-state", "active");
  await expect(accountTab).toHaveAttribute("aria-selected", "true");
  await expect(passwordTab).toHaveAttribute("data-state", "inactive");

  // Active content shows Account
  const activeContent = tabs.locator('[data-slot="tabs-content"][data-state="active"]');
  await expect(activeContent).toContainText("Account");

  // Click Password tab
  await passwordTab.click();
  await expect(passwordTab).toHaveAttribute("data-state", "active");
  await expect(accountTab).toHaveAttribute("data-state", "inactive");
  await expect(activeContent).toContainText("Password");

  // Click back
  await accountTab.click();
  await expect(accountTab).toHaveAttribute("data-state", "active");
});

test("keyboard navigation", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/tabs", { timeout: 20 * 60 * 1000 });

  const tabs = page.locator('[data-slot="tabs"]').first();
  const accountTab = tabs.getByRole("tab", { name: "Account" });
  const passwordTab = tabs.getByRole("tab", { name: "Password" });

  await accountTab.click();

  // ArrowRight moves focus to Password
  await page.keyboard.press("ArrowRight");
  await expect(passwordTab).toBeFocused();

  // ArrowRight loops back to Account
  await page.keyboard.press("ArrowRight");
  await expect(accountTab).toBeFocused();

  // ArrowLeft goes to Password
  await page.keyboard.press("ArrowLeft");
  await expect(passwordTab).toBeFocused();
});

test("accessibility attributes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/tabs", { timeout: 20 * 60 * 1000 });

  const tabs = page.locator('[data-slot="tabs"]').first();

  // TabsList has role="tablist"
  const list = tabs.locator('[data-slot="tabs-list"]');
  await expect(list).toHaveAttribute("role", "tablist");

  // Triggers have role="tab"
  const triggers = tabs.locator('[data-slot="tabs-trigger"]');
  const firstTrigger = triggers.first();
  await expect(firstTrigger).toHaveAttribute("role", "tab");

  // Active trigger has aria-selected="true"
  await expect(firstTrigger).toHaveAttribute("aria-selected", "true");

  // Content has role="tabpanel"
  const content = tabs.locator('[data-slot="tabs-content"][data-state="active"]');
  await expect(content).toHaveAttribute("role", "tabpanel");

  // aria-controls linkage: trigger references content id
  const ariaControls = await firstTrigger.getAttribute("aria-controls");
  expect(ariaControls).toBeTruthy();
  const panel = tabs.locator(`#${ariaControls}`);
  await expect(panel).toHaveAttribute("role", "tabpanel");
});
