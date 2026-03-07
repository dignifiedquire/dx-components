import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/docs/components/select", {
        timeout: 20 * 60 * 1000,
    });

    // Scope to the preview container
    const preview = page.locator('[data-slot="preview"]').first();
    await expect(preview).toBeVisible();

    // Trigger
    const selectTrigger = preview.locator('[data-slot="select-trigger"]');
    await expect(selectTrigger).toBeVisible();
    await expect(selectTrigger).toHaveAttribute("role", "combobox");
    await expect(selectTrigger).toHaveAttribute("data-state", "closed");
    await expect(selectTrigger).toHaveAttribute("aria-expanded", "false");
    await expect(selectTrigger).toHaveAttribute("aria-autocomplete", "none");

    // Value shows placeholder
    const selectValue = preview.locator('[data-slot="select-value"]');
    await expect(selectValue).toBeVisible();
    await expect(selectValue).toHaveAttribute("data-placeholder", "");

    // Open menu
    await selectTrigger.click();
    await expect(selectTrigger).toHaveAttribute("data-state", "open");
    await expect(selectTrigger).toHaveAttribute("aria-expanded", "true");

    // Content
    const content = page.locator('[data-slot="select-content"]');
    await expect(content).toBeVisible();
    await expect(content).toHaveAttribute("role", "listbox");
    await expect(content).toHaveAttribute("data-state", "open");

    // Items have correct role
    const items = content.locator('[data-slot="select-item"]');
    const count = await items.count();
    expect(count).toBeGreaterThan(0);
    await expect(items.first()).toHaveAttribute("role", "option");

    // Group
    const group = content.locator('[data-slot="select-group"]');
    await expect(group.first()).toHaveAttribute("role", "group");

    // Label
    const label = content.locator('[data-slot="select-label"]');
    await expect(label.first()).toBeVisible();

    // Assert the menu is focused
    await expect(content).toBeFocused();
    await page.keyboard.press("ArrowDown");
    const firstItem = items.first();
    await expect(firstItem).toBeFocused();
    await expect(firstItem).toHaveAttribute("data-highlighted", "");

    // Arrow down moves focus
    await page.keyboard.press("ArrowDown");
    const secondItem = items.nth(1);
    await expect(secondItem).toBeFocused();

    // Arrow up moves back
    await page.keyboard.press("ArrowUp");
    await expect(firstItem).toBeFocused();

    // Enter selects
    await page.keyboard.press("Enter");
    await expect(content).toHaveCount(0);

    // Reopen and check typeahead
    await selectTrigger.click();
    await page.keyboard.type("Ban");
    await expect(secondItem).toBeFocused();

    // Escape closes
    await page.keyboard.press("Escape");
    await expect(content).toHaveCount(0);

    // Reopen and click to select
    await selectTrigger.click();
    await expect(content).toBeVisible();
    const bananaItem = content.getByRole("option", { name: "banana" });
    await bananaItem.click();
    await expect(content).toHaveCount(0);
});

test("tabbing out closes", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/docs/components/select", {
        timeout: 20 * 60 * 1000,
    });

    const preview = page.locator('[data-slot="preview"]').first();
    const selectTrigger = preview.locator('[data-slot="select-trigger"]');
    await selectTrigger.click();

    const content = page.locator('[data-slot="select-content"]');
    await expect(content).toBeVisible();
    await expect(content).toBeFocused();

    await page.keyboard.press("Tab");
    await expect(content).toHaveCount(0);
});

test("arrow keys from trigger", async ({ page }) => {
    await page.goto("http://127.0.0.1:8080/docs/components/select", {
        timeout: 20 * 60 * 1000,
    });

    const preview = page.locator('[data-slot="preview"]').first();
    const selectTrigger = preview.locator('[data-slot="select-trigger"]');
    await selectTrigger.focus();

    // Down arrow opens and focuses first item
    await page.keyboard.press("ArrowDown");
    const content = page.locator('[data-slot="select-content"]');
    const items = content.locator('[data-slot="select-item"]');
    await expect(items.first()).toBeFocused();

    await page.keyboard.press("Escape");

    // Up arrow opens and focuses last item
    await selectTrigger.focus();
    await page.keyboard.press("ArrowUp");
    const lastItem = items.last();
    await expect(lastItem).toBeFocused();
});
