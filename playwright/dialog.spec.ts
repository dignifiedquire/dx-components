import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/dialog";

test.describe("dialog", () => {
  test("trigger has accessibility attributes when closed", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });

    const trigger = page.locator('[data-slot="dialog-trigger"]').first();
    await expect(trigger).toBeVisible();
    await expect(trigger).toHaveAttribute("data-state", "closed");
    await expect(trigger).toHaveAttribute("aria-haspopup", "dialog");
  });

  test("opens via show_modal with focus trap and aria attributes", async ({
    page,
  }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page.locator('[data-slot="dialog-trigger"]').first();
    await trigger.click();

    // Content is now a native <dialog> element with open attribute
    const content = page.locator('[data-slot="dialog-content"]').first();
    await expect(content).toBeVisible();
    await expect(content).toHaveAttribute("role", "dialog");
    await expect(content).toHaveAttribute("aria-modal", "true");
    await expect(content).toHaveAttribute("data-state", "open");
    const isDialog = await content.evaluate(
      (el) => el instanceof HTMLDialogElement && el.open,
    );
    expect(isDialog).toBe(true);

    // Overlay mounts (Presence-wrapped, only present when open)
    const overlay = page.locator('[data-slot="dialog-overlay"]').first();
    await expect(overlay).toBeVisible();
    await expect(overlay).toHaveAttribute("data-state", "open");

    // Title and description set by their IDs
    await expect(content.locator('[data-slot="dialog-title"]')).toHaveText(
      "Edit profile",
    );
    await expect(
      content.locator('[data-slot="dialog-description"]'),
    ).toContainText("Make changes to your profile");

    // Native focus trap moves focus into the dialog on showModal()
    const focusedInDialog = await page.evaluate(() => {
      const c = document.querySelector('[data-slot="dialog-content"]');
      return c?.contains(document.activeElement);
    });
    expect(focusedInDialog).toBe(true);

    // Tab stays inside the dialog (native focus trap)
    await page.keyboard.press("Tab");
    const stillInDialog = await page.evaluate(() => {
      const c = document.querySelector('[data-slot="dialog-content"]');
      return c?.contains(document.activeElement);
    });
    expect(stillInDialog).toBe(true);
  });

  test("close button closes dialog and unmounts overlay", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page.locator('[data-slot="dialog-trigger"]').first();
    await trigger.click();

    const content = page.locator('[data-slot="dialog-content"]').first();
    const overlay = page.locator('[data-slot="dialog-overlay"]').first();
    await expect(overlay).toBeVisible();

    const closeButton = content.locator('[data-slot="dialog-close"]').first();
    await closeButton.click();

    // Dialog content stays in DOM (always-mounted) but is closed
    await expect(content).toHaveAttribute("data-state", "closed");
    const dialogClosed = await content.evaluate(
      (el) => el instanceof HTMLDialogElement && !el.open,
    );
    expect(dialogClosed).toBe(true);

    // Overlay unmounts via Presence so it no longer blocks pointer events
    await expect(overlay).toHaveCount(0);
  });

  test("overlay click closes dialog", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page.locator('[data-slot="dialog-trigger"]').first();
    await trigger.click();

    const overlay = page.locator('[data-slot="dialog-overlay"]').first();
    await expect(overlay).toBeVisible();

    // Click in the corner of the overlay (away from the centered content)
    await overlay.dispatchEvent("pointerdown", { button: 0 });

    const content = page.locator('[data-slot="dialog-content"]').first();
    await expect(content).toHaveAttribute("data-state", "closed");
    await expect(overlay).toHaveCount(0);
  });

  test("browser-initiated close() syncs open state back", async ({ page }) => {
    // <dialog>.close() (fired natively on ESC) emits a `close` event that
    // use_top_layer subscribes to, syncing the controlled open signal back
    // to false. We exercise that path via direct .close() since CDP-
    // synthesized ESC does not reliably trigger the native cancel/close
    // flow.
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    const trigger = page.locator('[data-slot="dialog-trigger"]').first();
    await trigger.click();

    const content = page.locator('[data-slot="dialog-content"]').first();
    await expect(content).toHaveAttribute("data-state", "open");

    await content.evaluate((el) => (el as HTMLDialogElement).close());

    await expect(content).toHaveAttribute("data-state", "closed");
    await expect(page.locator('[data-slot="dialog-overlay"]')).toHaveCount(0);

    // Re-opening must work — if the signal hadn't synced, the trigger
    // would set open to false (no-op) instead of true.
    await trigger.click();
    await expect(content).toHaveAttribute("data-state", "open");
  });
});
