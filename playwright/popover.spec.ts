import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/popover";

test.describe("popover", () => {
  test("trigger accessibility attributes when closed", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    // Wait for WASM hydration before interacting.
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    await expect(trigger).toBeVisible();
    await expect(trigger).toHaveAttribute("data-state", "closed");
    await expect(trigger).toHaveAttribute("aria-haspopup", "dialog");
  });

  test("opens in top layer with positioning attributes", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    // Wait for WASM hydration before interacting.
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    await trigger.click();

    const content = page.locator('[data-slot="popover-content"]').first();
    await expect(content).toBeVisible();
    await expect(content).toHaveAttribute("role", "dialog");
    await expect(content).toHaveAttribute("data-state", "open");
    await expect(content).toHaveAttribute("data-side", "bottom");
    await expect(content).toHaveAttribute("data-align", "center");

    // The positioning wrapper carries `popover="manual"` and is in the top
    // layer. `manual` rather than `auto`: the browser's light-dismiss would
    // race the trigger's own click handler (whichever ran first decided
    // whether a click re-opened or closed), and it cannot be told to ignore
    // the trigger. Dismissal is DismissableLayer's job, exactly as upstream.
    const inTopLayer = await content.evaluate((el) => {
      const wrapper = el.closest("[data-radix-popper-content-wrapper]");
      return (
        wrapper instanceof HTMLElement &&
        wrapper.matches(":popover-open") &&
        wrapper.getAttribute("popover") === "manual"
      );
    });
    expect(inTopLayer).toBe(true);

    // Content has form fields
    await expect(content.locator("input")).toHaveCount(4);
  });

  test("calling hidePopover() syncs open signal back", async ({ page }) => {
    // CDP-synthesized ESC does not reliably trigger native popover light-
    // dismiss. We exercise the same code path by calling `hidePopover()`
    // on the wrapper directly — this fires the `toggle` event that
    // use_top_layer subscribes to.
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    // Wait for WASM hydration before interacting.
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    await trigger.click();

    const content = page.locator('[data-slot="popover-content"]').first();
    await expect(content).toBeVisible();

    await page.evaluate(() => {
      const wrapper = document.querySelector(
        "[data-radix-popper-content-wrapper]",
      ) as HTMLElement;
      wrapper.hidePopover();
    });

    await expect(content).toHaveCount(0);
    await expect(trigger).toHaveAttribute("data-state", "closed");
  });

  test("trigger toggles open and closed", async ({ page }) => {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    // Wait for WASM hydration before interacting.
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    const content = page.locator('[data-slot="popover-content"]').first();

    await trigger.click();
    await expect(content).toBeVisible();

    await trigger.click();
    await expect(content).toHaveCount(0);
  });

  test("reopening after browser-initiated close works", async ({ page }) => {
    // If the signal hadn't synced after hidePopover, the next trigger
    // click would set open to `false` (no-op) instead of `true` (reopen).
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    // Wait for WASM hydration before interacting.
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    const content = page.locator('[data-slot="popover-content"]').first();
    await trigger.click();
    await expect(content).toBeVisible();

    await page.evaluate(() => {
      const wrapper = document.querySelector(
        "[data-radix-popper-content-wrapper]",
      ) as HTMLElement;
      wrapper.hidePopover();
    });
    await expect(content).toHaveCount(0);

    await trigger.click();
    await expect(content).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Top-layer divergence guarantees
//
// Upstream Radix portals content into `document.body`, which detaches it from
// ancestor CSS. We render in place and reach the top layer via the `popover`
// attribute, so `primitives/src/popper.rs` has to restore by hand what the
// portal would have escaped. These tests pin that contract: they fail if the
// wrapper's user-agent-stylesheet reset is dropped.
// ---------------------------------------------------------------------------

test.describe("popover: top-layer divergence guarantees", () => {
  async function openPopover(page: import("@playwright/test").Page) {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    // Wait for WASM hydration before interacting.
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    const content = page.locator('[data-slot="popover-content"]').first();

    // Opening is retried because `popover="auto"` races the trigger click
    // against the browser's native light-dismiss: whichever lands first
    // decides whether the click opens or re-closes. That race is what the
    // DismissableLayer rework removes (upstream ignores pointer-downs on the
    // trigger; native light-dismiss cannot). It is ~8% on WebKit under load
    // and unrelated to what these tests assert, so retry the open rather than
    // let it mask the CSS-inheritance assertions below. A popover that never
    // opens still fails here, loudly.
    await expect(async () => {
      if (!(await content.isVisible())) {
        await trigger.click();
      }
      await expect(content).toBeVisible({ timeout: 1500 });
    }).toPass({ timeout: 20_000 });

    // Floating-ui parks the content at `translate(0, -200%)` until it has
    // measured and placed it, so a freshly opened popover can sit above the
    // viewport for a frame or two. Hit-testing before that lands measures
    // nothing. Wait for a real on-screen position.
    await expect
      .poll(
        async () => content.evaluate((el) => el.getBoundingClientRect().top),
        { timeout: 10_000 },
      )
      .toBeGreaterThan(0);

    return content;
  }

  /** Is the element the topmost thing at its own centre? */
  const isHitTestable = (content: import("@playwright/test").Locator) =>
    content.evaluate((el) => {
      const r = el.getBoundingClientRect();
      const top = document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2);
      return !!top && (top === el || el.contains(top));
    });

  test("wrapper neutralises the UA [popover] stylesheet", async ({ page }) => {
    const content = await openPopover(page);

    // The browser styles every [popover] element with `background: Canvas`,
    // `color: CanvasText` and `overflow: auto`. On the positioning wrapper that
    // paints an opaque panel behind the content — which shows through the
    // content's rounded corners, most visibly in dark mode — and the
    // `overflow: auto` clips the content's box-shadow. Upstream renders a plain
    // portaled div and never meets these rules.
    const wrapper = await content.evaluate((el) => {
      const w = el.closest("[data-radix-popper-content-wrapper]") as HTMLElement;
      const cs = getComputedStyle(w);
      return {
        background: cs.backgroundColor,
        overflowX: cs.overflowX,
        overflowY: cs.overflowY,
      };
    });
    expect(wrapper.background).toBe("rgba(0, 0, 0, 0)");
    expect(wrapper.overflowX).toBe("visible");
    expect(wrapper.overflowY).toBe("visible");
  });

  test("an ancestor's pointer-events: none does not disable an open popover", async ({
    page,
  }) => {
    // A top-layer element still *inherits* pointer-events, and Radix itself
    // uses `pointer-events: none` on outer content while a layer is open. Its
    // portal escapes that; ours has to pin `pointer-events: auto` instead, or
    // the overlay is painted but dead to input.
    const content = await openPopover(page);
    expect(await isHitTestable(content)).toBe(true);

    await page.evaluate(() => {
      const preview = document.querySelector('[data-slot="preview"]') as HTMLElement;
      preview.style.pointerEvents = "none";
    });

    await expect(content).toBeVisible();
    expect(await isHitTestable(content)).toBe(true);
  });

  test("an ancestor's visibility: hidden does not hide an open popover", async ({
    page,
  }) => {
    // Same inheritance problem as pointer-events: hiding a container would
    // otherwise blank out an overlay that is logically inside it.
    const content = await openPopover(page);

    await page.evaluate(() => {
      const preview = document.querySelector('[data-slot="preview"]') as HTMLElement;
      preview.style.visibility = "hidden";
    });

    const visibility = await content.evaluate(
      (el) => getComputedStyle(el).visibility,
    );
    expect(visibility).toBe("visible");
    expect(await isHitTestable(content)).toBe(true);
  });
});

// ---------------------------------------------------------------------------
// Dismissal semantics
//
// These are the behaviours the DismissableLayer rework restores. Under
// `popover="auto"` the browser owned dismissal: ESC could not be driven from
// CDP at all, and a click on the trigger raced native light-dismiss against
// the trigger's own handler. Now our own listeners own it, upstream-style, and
// the behaviour is directly testable.
// ---------------------------------------------------------------------------

test.describe("popover: dismissal", () => {
  async function open(page: import("@playwright/test").Page) {
    await page.goto(URL, { timeout: 20 * 60 * 1000 });
    await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
    const trigger = page.locator('[data-slot="popover-trigger"]').first();
    const content = page.locator('[data-slot="popover-content"]').first();
    await trigger.click();
    await expect(content).toBeVisible();
    await expect
      .poll(async () => content.evaluate((el) => el.getBoundingClientRect().top), {
        timeout: 10_000,
      })
      .toBeGreaterThan(0);
    return { trigger, content };
  }

  test("Escape closes the popover", async ({ page }) => {
    // A real key press, not a `hidePopover()` stand-in: dismissal now runs
    // through `use_escape_keydown` inside the dismissable layer stack.
    const { content } = await open(page);
    await page.keyboard.press("Escape");
    await expect(content).toHaveCount(0);
  });

  test("clicking outside closes the popover", async ({ page }) => {
    const { content } = await open(page);
    // Click far from both the trigger and the content.
    await page.mouse.click(5, 5);
    await expect(content).toHaveCount(0);
  });

  test("clicking the popover's own padding does NOT close it", async ({ page }) => {
    // The regression guard for making the layer a hook. If DismissableLayer
    // rendered its own div inside PopperContent's styled box, a pointer-down on
    // the box's `p-4` padding would never reach the layer's handler, the
    // "pointer is inside the tree" flag would stay false, and the document
    // listener would dismiss on the popover's own padding.
    const { content } = await open(page);
    const box = await content.boundingBox();
    expect(box).not.toBeNull();
    // Top-centre, 4px in: inside the content's own padding and clear of every
    // child control. Not a corner — the box is `rounded-md`, so a corner pixel
    // can fall outside the painted shape and hit-test straight through.
    await page.mouse.click(box!.x + box!.width / 2, box!.y + 4);
    await expect(content).toBeVisible();
    await expect(content).toHaveAttribute("data-state", "open");
  });

  test("focus returns to the trigger after closing", async ({ page }) => {
    // Upstream restores focus from FocusScope's unmount auto-focus, which is
    // preventable via `on_close_auto_focus`.
    const { trigger, content } = await open(page);
    await page.keyboard.press("Escape");
    await expect(content).toHaveCount(0);
    await expect(trigger).toBeFocused();
  });
});

// ---------------------------------------------------------------------------
// Exit animation
// ---------------------------------------------------------------------------

test("closing plays the exit animation before unmounting", async ({ page }) => {
  // The shadcn content class carries `data-[state=closed]:animate-out
  // fade-out-0 zoom-out-95`, but those never ran before this rework. Two
  // separate causes: the top layer was driven by `open`, so `hidePopover()`
  // set `display: none` in the same frame the state flipped; and Popper's
  // pre-placement `animation: none` inline style could never be cleared,
  // because Dioxus's interpreter re-applies inline properties that a new style
  // string omits. Both are fixed, so the exit animation must now actually run.
  await page.goto(URL, { timeout: 20 * 60 * 1000 });
  await page.locator("body:not(.preload)").waitFor({ timeout: 60_000 });
  const trigger = page.locator('[data-slot="popover-trigger"]').first();
  const content = page.locator('[data-slot="popover-content"]').first();

  await trigger.click();
  await expect(content).toBeVisible();

  // Record the exit animation as it starts rather than sampling after the
  // fact: the window between the state flip and unmount is ~150ms and racing
  // it makes the test flaky, while a missing animation still fails loudly
  // because nothing gets recorded.
  await content.evaluate((el) => {
    (window as Window & { __exitAnim?: unknown }).__exitAnim = null;
    el.addEventListener("animationstart", (event) => {
      (window as Window & { __exitAnim?: unknown }).__exitAnim = {
        name: (event as AnimationEvent).animationName,
        state: el.getAttribute("data-state"),
        display: getComputedStyle(el).display,
      };
    });
  });

  await page.keyboard.press("Escape");
  await expect(content).toHaveCount(0);

  const recorded = (await page.evaluate(
    () => (window as Window & { __exitAnim?: unknown }).__exitAnim,
  )) as { name: string; state: string; display: string } | null;

  expect(recorded, "an exit animation should have started on close").not.toBeNull();
  expect(recorded!.state).toBe("closed");
  expect(recorded!.name).not.toBe("none");
  expect(recorded!.display).not.toBe("none");
});
