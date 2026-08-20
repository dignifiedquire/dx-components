import { test, expect } from "@playwright/test";

test("hover card opens on hover/focus and closes on leave/blur", async ({
  page,
}) => {
  await page.goto("http://127.0.0.1:8080/docs/components/hover_card", {
    timeout: 20 * 60 * 1000,
  });

  // Wait for WASM hydration before interacting. `app_layout.rs` removes the
  // `preload` class from <body> after the first render, so this clears only
  // once the app has rendered and its listeners are attached. A hover fired
  // before that lands on a dead element and is never replayed — the card
  // simply never opens (WebKit is slow enough to hit this every run).
  await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });

  const trigger = page.locator('[data-slot="hover-card-trigger"]');
  await expect(trigger).toBeVisible();
  await expect(trigger).toHaveAttribute("data-state", "closed");

  const content = page.locator('[data-slot="hover-card-content"]');

  // Start the pointer off the trigger so the hover below is a real move onto it.
  await page.mouse.move(0, 0);
  await trigger.hover();
  await expect(content).toBeVisible({ timeout: 10_000 });
  await expect(content).toHaveAttribute("data-state", "open");
  await expect(content).toHaveAttribute("data-side", "bottom");
  await expect(content).toHaveAttribute("data-align", "center");

  // Moving the mouse away hides content
  await page.mouse.move(0, 0);
  await expect(content).toHaveCount(0);

  // Focus shows content — confirm focus actually landed on the anchor first,
  // so a failure here points at the open path rather than at focus itself.
  await trigger.focus();
  await expect(trigger).toBeFocused();
  await expect(content).toBeVisible({ timeout: 10_000 });

  // Blur hides content
  await trigger.blur();
  await expect(content).toHaveCount(0);
});
