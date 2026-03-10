import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/dialog", { timeout: 20 * 60 * 1000 });

  // Trigger button
  const trigger = page.locator('[data-slot="dialog-trigger"]');
  await expect(trigger).toBeVisible();
  await expect(trigger).toHaveAttribute("data-state", "closed");
  await expect(trigger).toHaveAttribute("aria-haspopup", "dialog");

  // Open the dialog
  await trigger.click();

  // Overlay
  const overlay = page.locator('[data-slot="dialog-overlay"]');
  await expect(overlay).toBeVisible();
  await expect(overlay).toHaveAttribute("data-state", "open");

  // Content
  const content = page.locator('[data-slot="dialog-content"]');
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("role", "dialog");
  await expect(content).toHaveAttribute("aria-modal", "true");
  await expect(content).toHaveAttribute("data-state", "open");

  // Title and description
  const title = content.locator('[data-slot="dialog-title"]');
  await expect(title).toHaveText("Edit profile");

  const description = content.locator('[data-slot="dialog-description"]');
  await expect(description).toContainText("Make changes to your profile");

  // Close button (X icon in top-right corner)
  const closeButton = content.locator('[data-slot="dialog-close"]').first();
  await expect(closeButton).toBeVisible();

  // Tab should keep focus within dialog (focus trap)
  await page.keyboard.press("Tab");
  // Focus should stay within the dialog content
  const focusedInDialog = await page.evaluate(() => {
    const content = document.querySelector('[data-slot="dialog-content"]');
    return content?.contains(document.activeElement);
  });
  expect(focusedInDialog).toBe(true);

  // Escape should close the dialog
  await page.keyboard.press("Escape");
  await expect(overlay).toHaveCount(0);

  // Reopen and test close button
  await trigger.click();
  await expect(overlay).toBeVisible();
  await closeButton.click();
  await expect(overlay).toHaveCount(0);

  // Reopen and test overlay click to close
  await trigger.click();
  await expect(overlay).toBeVisible();
  await overlay.click({ position: { x: 5, y: 5 } });
  await expect(overlay).toHaveCount(0);
});
