import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/docs/components/alert_dialog", { timeout: 20 * 60 * 1000 });

  // Trigger button
  const trigger = page.locator('[data-slot="alert-dialog-trigger"]');
  await expect(trigger).toBeVisible();
  await expect(trigger).toHaveAttribute("data-state", "closed");
  await expect(trigger).toHaveAttribute("aria-haspopup", "dialog");

  // Open the alert dialog
  await trigger.click();

  // Overlay
  const overlay = page.locator('[data-slot="alert-dialog-overlay"]');
  await expect(overlay).toBeVisible();
  await expect(overlay).toHaveAttribute("data-state", "open");

  // Content
  const content = page.locator('[data-slot="alert-dialog-content"]');
  await expect(content).toBeVisible();
  await expect(content).toHaveAttribute("role", "alertdialog");
  await expect(content).toHaveAttribute("aria-modal", "true");
  await expect(content).toHaveAttribute("data-state", "open");

  // Title and description
  const title = content.locator('[data-slot="alert-dialog-title"]');
  await expect(title).toHaveText("Delete item");

  const description = content.locator('[data-slot="alert-dialog-description"]');
  await expect(description).toContainText("Are you sure");

  // Footer with action buttons
  const footer = content.locator('[data-slot="alert-dialog-footer"]');
  await expect(footer).toBeVisible();

  // Cancel and Action buttons
  const cancelButton = content.locator('[data-slot="alert-dialog-cancel"]');
  await expect(cancelButton).toBeVisible();
  await expect(cancelButton).toHaveText("Cancel");

  const actionButton = content.locator('[data-slot="alert-dialog-action"]');
  await expect(actionButton).toBeVisible();
  await expect(actionButton).toHaveText("Delete");

  // Focus should be trapped within dialog
  await page.keyboard.press("Tab");
  const focusedInDialog = await page.evaluate(() => {
    const content = document.querySelector('[data-slot="alert-dialog-content"]');
    return content?.contains(document.activeElement);
  });
  expect(focusedInDialog).toBe(true);

  // Overlay click should NOT close (unlike regular dialog)
  await overlay.click({ position: { x: 5, y: 5 }, force: true });
  await expect(content).toBeVisible();

  // Escape should close
  await page.keyboard.press("Escape");
  await expect(overlay).toHaveCount(0);

  // Reopen and test cancel button
  await trigger.click();
  await expect(overlay).toBeVisible();
  await cancelButton.click();
  await expect(overlay).toHaveCount(0);

  // Reopen and test action button
  await trigger.click();
  await expect(overlay).toBeVisible();
  await actionButton.click();
  await expect(overlay).toHaveCount(0);
});
