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

// ---------------------------------------------------------------------------
// Submenu pointer behaviour
//
// Menus follow the pointer: hovering an item focuses it, so hover and keyboard
// navigation share one highlighted state, and hovering a sub-trigger opens its
// submenu. The subtle part is the grace area — a pointer travelling diagonally
// from the sub-trigger into the submenu passes over the items in between, and
// those must not steal focus and close the submenu on the way.
// ---------------------------------------------------------------------------

test.describe("dropdown menu: submenu pointer behaviour", () => {
  // Scope to the first demo. The navbar's mobile-nav Sheet is always in the
  // DOM and contains its own dropdown trigger, so a bare `.first()` can resolve
  // to a hidden trigger whose menu positions off-screen — which is exactly what
  // happens on WebKit.
  const demo = (page: import("@playwright/test").Page) =>
    page.locator('[data-slot="preview"]').first();

  /**
   * Hover an element until an assertion about the result holds.
   *
   * Every step here re-reads the element's box, because floating-ui places
   * content asynchronously and can re-measure afterwards: a box read once and
   * reused points at where the menu *was*. WebKit places roughly three times
   * slower than Chromium, and slower still under parallel load, which is
   * exactly when a stale coordinate turns into a flaky test. A menu that never
   * responds still fails, on the outer timeout.
   */
  async function hoverUntil(
    page: import("@playwright/test").Page,
    target: import("@playwright/test").Locator,
    check: () => Promise<void>,
  ) {
    await expect(async () => {
      const box = await target.boundingBox();
      expect(box, "target should have a box").not.toBeNull();
      expect(box!.top ?? box!.y, "target should be on screen").toBeGreaterThan(0);
      // Two moves: the second is what produces a pointermove over the target.
      await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
      await page.mouse.move(box!.x + box!.width / 2 + 1, box!.y + box!.height / 2);
      await check();
    }).toPass({ timeout: 20_000 });
  }

  async function openMenu(page: import("@playwright/test").Page) {
    await page.goto("http://127.0.0.1:8080/docs/components/dropdown_menu", {
      timeout: 20 * 60 * 1000,
    });
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    await demo(page).locator('[data-slot="dropdown-menu-trigger"]').first().click();
    const content = page.locator('[data-slot="dropdown-menu-content"]').first();
    await expect(content).toBeVisible();
    await expect
      .poll(async () => content.evaluate((el) => el.getBoundingClientRect().top), {
        timeout: 15_000,
      })
      .toBeGreaterThan(0);
  }

  test("hovering an item highlights it", async ({ page }) => {
    await openMenu(page);
    const item = demo(page).locator('[data-slot="dropdown-menu-item"]').first();
    await hoverUntil(page, item, async () => {
      await expect(item).toHaveAttribute("data-highlighted", "", { timeout: 1000 });
    });
  });

  test("hovering the sub-trigger opens the submenu", async ({ page }) => {
    await openMenu(page);
    const subTrigger = demo(page).locator('[data-slot="dropdown-menu-sub-trigger"]').first();
    const subContent = page.locator('[data-slot="dropdown-menu-sub-content"]').first();
    // Upstream opens after a 100ms dwell rather than on entry.
    await hoverUntil(page, subTrigger, async () => {
      await expect(subContent).toBeVisible({ timeout: 2000 });
    });
  });

  test("moving to another item closes the submenu", async ({ page }) => {
    // This is what items focusing on pointer-move buys: focus leaves the
    // submenu, its layer sees a focus-outside, and it closes. Without it a
    // hover-opened submenu stayed open forever.
    await openMenu(page);
    const subTrigger = demo(page).locator('[data-slot="dropdown-menu-sub-trigger"]').first();
    const subContent = page.locator('[data-slot="dropdown-menu-sub-content"]').first();
    await hoverUntil(page, subTrigger, async () => {
      await expect(subContent).toBeVisible({ timeout: 2000 });
    });

    const other = demo(page).locator('[data-slot="dropdown-menu-item"]').first();
    await hoverUntil(page, other, async () => {
      await expect(subContent).toHaveCount(0, { timeout: 2000 });
    });
  });

  test("a diagonal move into the submenu does not close it", async ({ page }) => {
    // The grace area: a polygon from the pointer to the submenu's edges, gated
    // on the pointer travelling towards it. Crossing the item below the
    // sub-trigger on the way must not close the submenu.
    await openMenu(page);
    const subTrigger = demo(page).locator('[data-slot="dropdown-menu-sub-trigger"]').first();
    const subContent = page.locator('[data-slot="dropdown-menu-sub-content"]').first();

    // The whole sweep retries, re-opening and re-measuring each time: the menu
    // can still be settling into place when the first attempt starts, and a
    // sweep aimed at stale coordinates proves nothing either way.
    await expect(async () => {
      await hoverUntil(page, subTrigger, async () => {
        await expect(subContent).toBeVisible({ timeout: 2000 });
      });
      await expect
        .poll(async () => subContent.evaluate((el) => el.getBoundingClientRect().top), {
          timeout: 5000,
        })
        .toBeGreaterThan(0);

      const trigger = (await subTrigger.boundingBox())!;
      const sub = (await subContent.boundingBox())!;
      const fromX = trigger.x + trigger.width / 2;
      const fromY = trigger.y + trigger.height / 2;
      const toX = sub.x + sub.width / 2;
      const toY = sub.y + sub.height - 8;
      for (let step = 1; step <= 8; step++) {
        await page.mouse.move(
          fromX + ((toX - fromX) * step) / 8,
          fromY + ((toY - fromY) * step) / 8,
        );
        await page.waitForTimeout(20);
      }

      await expect(subContent).toBeVisible({ timeout: 1000 });
    }).toPass({ timeout: 30_000 });
  });
});
