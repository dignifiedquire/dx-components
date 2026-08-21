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

  // Reopen, then close by clicking the trigger again.
  await trigger.click();
  await expect(content).toBeVisible();

  // A raw mouse click rather than `trigger.click()`. The menu is modal, so the
  // dismissable layer sets `pointer-events: none` outside itself — the trigger
  // included, exactly as upstream does. The user's click therefore falls
  // through to the document, where the layer's outside-pointerdown handler
  // dismisses it; the menu closes and the trigger never sees the event.
  // Playwright's `click()` refuses to drive an element that will not receive
  // the event, so it would wait forever on behaviour that is correct.
  const triggerBox = await trigger.boundingBox();
  expect(triggerBox).not.toBeNull();
  await page.mouse.click(
    triggerBox!.x + triggerBox!.width / 2,
    triggerBox!.y + triggerBox!.height / 2,
  );
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

// ---------------------------------------------------------------------------
// Dismissable layer behaviour
//
// The menu family used to dismiss through three ad-hoc mechanisms — a document
// Escape listener, an outside-click helper and a focusout probe — none of which
// could take part in the shared layer stack or disable outside pointer events.
// These pin what the layer brought.
// ---------------------------------------------------------------------------

test.describe("dropdown menu: layer semantics", () => {
  async function open(page: import("@playwright/test").Page) {
    await page.goto("http://127.0.0.1:8080/docs/components/dropdown_menu", {
      timeout: 20 * 60 * 1000,
    });
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    const trigger = page.locator('[data-slot="dropdown-menu-trigger"]').first();
    const content = page.locator('[data-slot="dropdown-menu-content"]').first();
    await trigger.click();
    await expect(content).toBeVisible();
    return { trigger, content };
  }

  test("a modal menu makes the rest of the page inert while open", async ({ page }) => {
    // Upstream's `disableOutsidePointerEvents`: the layer sets
    // `pointer-events: none` on the body and `auto` on itself, so the first
    // click outside dismisses the menu instead of also activating whatever it
    // landed on. Without it a single click both closed the menu and pressed the
    // button underneath.
    const { content } = await open(page);
    expect(await page.evaluate(() => document.body.style.pointerEvents)).toBe("none");

    await page.keyboard.press("Escape");
    await expect(content).toHaveCount(0);
    expect(await page.evaluate(() => document.body.style.pointerEvents)).toBe("");
  });

  test("clicking outside closes the menu", async ({ page }) => {
    const { content } = await open(page);
    await page.mouse.click(5, 5);
    await expect(content).toHaveCount(0);
  });

  test("closing plays the exit animation", async ({ page }) => {
    // The top layer follows Presence's animation-aware `present` rather than
    // `open`, so `hidePopover()` no longer sets `display: none` in the frame
    // the state flips and `data-[state=closed]:animate-out` can run.
    const { content } = await open(page);
    await content.evaluate((el) => {
      (window as Window & { __exit?: unknown }).__exit = null;
      el.addEventListener("animationstart", (event) => {
        (window as Window & { __exit?: unknown }).__exit = {
          name: (event as AnimationEvent).animationName,
          state: el.getAttribute("data-state"),
        };
      });
    });

    await page.keyboard.press("Escape");
    await expect(content).toHaveCount(0);

    const recorded = (await page.evaluate(
      () => (window as Window & { __exit?: unknown }).__exit,
    )) as { name: string; state: string } | null;
    expect(recorded, "an exit animation should have started on close").not.toBeNull();
    expect(recorded!.state).toBe("closed");
    expect(recorded!.name).not.toBe("none");
  });
});
