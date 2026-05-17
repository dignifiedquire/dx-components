import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/alert_dialog";

test.describe("alert dialog", () => {
  test("trigger accessibility attributes when closed", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page
      .locator('[data-slot="alert-dialog-trigger"]')
      .first();
    await expect(trigger).toBeVisible();
    await expect(trigger).toHaveAttribute("data-state", "closed");
    await expect(trigger).toHaveAttribute("aria-haspopup", "dialog");
  });

  test("opens with role=alertdialog and aria attributes", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page
      .locator('[data-slot="alert-dialog-trigger"]')
      .first();
    await trigger.click();

    const content = page
      .locator('[data-slot="alert-dialog-content"]')
      .first();
    await expect(content).toBeVisible();
    await expect(content).toHaveAttribute("role", "alertdialog");
    await expect(content).toHaveAttribute("aria-modal", "true");
    await expect(content).toHaveAttribute("data-state", "open");

    const isDialog = await content.evaluate(
      (el) => el instanceof HTMLDialogElement && el.open,
    );
    expect(isDialog).toBe(true);

    // Overlay mounts via Presence
    const overlay = page
      .locator('[data-slot="alert-dialog-overlay"]')
      .first();
    await expect(overlay).toBeVisible();
    await expect(overlay).toHaveAttribute("data-state", "open");

    // Title and description
    await expect(
      content.locator('[data-slot="alert-dialog-title"]'),
    ).toHaveText("Are you absolutely sure?");
    await expect(
      content.locator('[data-slot="alert-dialog-description"]'),
    ).toContainText("This action cannot be undone");

    // Footer + buttons
    const footer = content.locator('[data-slot="alert-dialog-footer"]');
    await expect(footer).toBeVisible();
    const cancelBtn = content.locator('[data-slot="alert-dialog-cancel"]');
    const actionBtn = content.locator('[data-slot="alert-dialog-action"]');
    await expect(cancelBtn).toHaveText("Cancel");
    await expect(actionBtn).toHaveText("Continue");

    // Native focus trap
    const focusedInDialog = await page.evaluate(() => {
      const c = document.querySelector('[data-slot="alert-dialog-content"]');
      return c?.contains(document.activeElement);
    });
    expect(focusedInDialog).toBe(true);
  });

  test("overlay click does NOT close (alert dialog semantics)", async ({
    page,
  }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.locator('[data-slot="alert-dialog-trigger"]').first().click();

    const content = page
      .locator('[data-slot="alert-dialog-content"]')
      .first();
    const overlay = page
      .locator('[data-slot="alert-dialog-overlay"]')
      .first();
    await expect(overlay).toBeVisible();

    // Force a click in the corner of the overlay — should NOT dismiss
    await overlay.dispatchEvent("pointerdown", { button: 0 });
    await expect(content).toHaveAttribute("data-state", "open");

    const stillOpen = await content.evaluate(
      (el) => el instanceof HTMLDialogElement && el.open,
    );
    expect(stillOpen).toBe(true);
  });

  test("cancel button closes", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.locator('[data-slot="alert-dialog-trigger"]').first().click();
    const content = page
      .locator('[data-slot="alert-dialog-content"]')
      .first();
    await expect(content).toHaveAttribute("data-state", "open");

    await content.locator('[data-slot="alert-dialog-cancel"]').click();

    await expect(content).toHaveAttribute("data-state", "closed");
    await expect(
      page.locator('[data-slot="alert-dialog-overlay"]'),
    ).toHaveCount(0);
  });

  test("action button closes", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.locator('[data-slot="alert-dialog-trigger"]').first().click();
    const content = page
      .locator('[data-slot="alert-dialog-content"]')
      .first();
    await expect(content).toHaveAttribute("data-state", "open");

    await content.locator('[data-slot="alert-dialog-action"]').click();

    await expect(content).toHaveAttribute("data-state", "closed");
    await expect(
      page.locator('[data-slot="alert-dialog-overlay"]'),
    ).toHaveCount(0);
  });

  test("browser-initiated close() syncs open state back", async ({ page }) => {
    // Native `<dialog>` close (e.g. real ESC) fires a `close` event that
    // use_top_layer subscribes to. We exercise that path via direct
    // .close() since CDP-synthesized ESC does not reliably trigger the
    // native cancel/close flow.
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page
      .locator('[data-slot="alert-dialog-trigger"]')
      .first();
    await trigger.click();
    const content = page
      .locator('[data-slot="alert-dialog-content"]')
      .first();
    await expect(content).toHaveAttribute("data-state", "open");

    await content.evaluate((el) => (el as HTMLDialogElement).close());

    await expect(content).toHaveAttribute("data-state", "closed");

    // Re-open via trigger — proves the signal synced back
    await trigger.click();
    await expect(content).toHaveAttribute("data-state", "open");
  });
});
