import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/dropdown_menu", { timeout: 20 * 60 * 1000 });
  // Wait for WASM hydration before interacting.
  await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });

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

// ---------------------------------------------------------------------------
// Global Escape hygiene
// ---------------------------------------------------------------------------

test("a closed menu does not cancel Escape for the rest of the page", async ({
  page,
}) => {
  // The document-level Escape listener cancels the key's default action in
  // JavaScript, before the Rust callback decides anything. It used to be
  // installed whenever the content component was mounted — which is always,
  // since the mount gate lives inside it — so any page containing a menu
  // silently swallowed every Escape, including the one that closes a native
  // `<dialog>`. The listener is now gated on the menu being open.
  await page.goto("http://127.0.0.1:8080/docs/components/dropdown_menu", {
    timeout: 20 * 60 * 1000,
  });
  await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });

  const escapeWasCancelled = () =>
    page.evaluate(async () => {
      let prevented: boolean | null = null;
      const handler = (event: KeyboardEvent) => {
        if (event.key === "Escape") prevented = event.defaultPrevented;
      };
      document.addEventListener("keydown", handler, false);
      document.dispatchEvent(
        new KeyboardEvent("keydown", { key: "Escape", cancelable: true, bubbles: true }),
      );
      await new Promise((resolve) => setTimeout(resolve, 30));
      document.removeEventListener("keydown", handler, false);
      return prevented;
    });

  expect(await escapeWasCancelled()).toBe(false);

  const trigger = page.locator('[data-slot="dropdown-menu-trigger"]').first();
  const content = page.locator('[data-slot="dropdown-menu-content"]').first();
  await trigger.click();
  await expect(content).toBeVisible();

  // While open the menu does consume Escape — that part is intended.
  expect(await escapeWasCancelled()).toBe(true);
  await page.keyboard.press("Escape");
  await expect(content).toHaveCount(0);

  // ...and it stops consuming it again once closed.
  expect(await escapeWasCancelled()).toBe(false);
});
