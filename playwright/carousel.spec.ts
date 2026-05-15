import { test, expect, type Locator, type Page } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/carousel";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

/** Navigate and wait for WASM hydration (at least one carousel rendered). */
async function gotoAndWait(page: Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-slot="carousel"]')
    .first()
    .waitFor({ state: "visible", timeout: 60_000 });
  // The page hydrates several carousels; give the WASM bundle a beat so the
  // first interaction doesn't race the click-handler wiring (Firefox/WebKit).
  await page.waitForLoadState("networkidle");
}

/** The first/main carousel inside the hero preview. */
function mainCarousel(page: Page) {
  return page.locator('[data-slot="preview"] [data-slot="carousel"]').first();
}

/** A variant carousel by its example section id (e.g. "multiple"). */
function variantCarousel(page: Page, id: string) {
  return page.locator(`#${id} [data-slot="carousel"]`).first();
}

/** Read the signed translate percentage off a carousel-content element. */
async function translatePct(content: Locator): Promise<number> {
  const style = (await content.getAttribute("style")) ?? "";
  const m = style.match(/translate[XY]\((-?[\d.]+)%\)/);
  return m ? parseFloat(m[1]) : NaN;
}

/** Click a button until it's disabled (caps iterations so a bug can't hang). */
async function clickUntilDisabled(btn: Locator, cap = 12) {
  for (let i = 0; i < cap; i++) {
    if (!(await btn.isEnabled())) return;
    await btn.click();
  }
}

// ---------------------------------------------------------------------------
// Boundary behaviour — single-visible (main: total 5, slides_per_view 1)
// ---------------------------------------------------------------------------

test.describe("carousel: single-visible boundaries", () => {
  test("prev is disabled at the start", async ({ page }) => {
    await gotoAndWait(page);
    const car = mainCarousel(page);
    await expect(car.locator('[data-slot="carousel-previous"]')).toBeDisabled();
    await expect(car.locator('[data-slot="carousel-next"]')).toBeEnabled();
  });

  test("next disables at the end and the track does not overscroll", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const car = mainCarousel(page);
    const next = car.locator('[data-slot="carousel-next"]');
    const content = car.locator('[data-slot="carousel-content"]');

    await clickUntilDisabled(next);

    await expect(next).toBeDisabled();
    await expect(car.locator('[data-slot="carousel-previous"]')).toBeEnabled();
    // 5 slides, 1 per view, -100% per step → clamps at index 4 (-400%) and
    // never overscrolls beyond.
    await expect.poll(() => translatePct(content)).toBeCloseTo(-400, 1);
  });
});

// ---------------------------------------------------------------------------
// Boundary behaviour — multi-visible (multiple: total 5, slides_per_view 3)
// ---------------------------------------------------------------------------

test.describe("carousel: multi-visible boundaries", () => {
  test("next stops at the last full group (no scrolling into empty space)", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const car = variantCarousel(page, "multiple");
    const next = car.locator('[data-slot="carousel-next"]');
    const content = car.locator('[data-slot="carousel-content"]');

    await clickUntilDisabled(next);

    // 5 slides, 3 per view → max index 2, step = 100/3 % → clamps at
    // ≈ -66.667%. The bug this guards against was scrolling past -100%.
    await expect(next).toBeDisabled();
    await expect.poll(() => translatePct(content)).toBeCloseTo(-66.667, 1);
  });

  test("prev returns to the start and disables", async ({ page }) => {
    await gotoAndWait(page);
    const car = variantCarousel(page, "multiple");
    const next = car.locator('[data-slot="carousel-next"]');
    const prev = car.locator('[data-slot="carousel-previous"]');
    const content = car.locator('[data-slot="carousel-content"]');

    await next.click();
    await expect(prev).toBeEnabled();
    await clickUntilDisabled(prev);

    await expect(prev).toBeDisabled();
    await expect.poll(() => translatePct(content)).toBeCloseTo(0, 1);
  });
});

// ---------------------------------------------------------------------------
// Disabled state is visible (button gets opacity-50 at the boundary)
// ---------------------------------------------------------------------------

test.describe("carousel: disabled state is visible", () => {
  test("disabled prev button is dimmed", async ({ page }) => {
    await gotoAndWait(page);
    const prev = mainCarousel(page).locator('[data-slot="carousel-previous"]');
    await expect(prev).toBeDisabled();
    // disabled:opacity-50 → computed opacity 0.5 once the transition settles.
    await expect
      .poll(async () => prev.evaluate((el) => getComputedStyle(el).opacity))
      .toBe("0.5");
  });
});

// ---------------------------------------------------------------------------
// Viewport must NOT be a scroll container — `overflow: hidden` only hides
// the scrollbar, the element stays trackpad/touch/programmatically
// scrollable, which bypasses the transform boundary entirely. The viewport
// must use `overflow: clip` so a swipe can't scroll past the last slide.
// ---------------------------------------------------------------------------

test.describe("carousel: viewport is not scrollable", () => {
  test("setting scrollLeft on the viewport has no effect", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const viewport = mainCarousel(page).locator(
      '[data-slot="carousel-viewport"]',
    );
    const scrolled = await viewport.evaluate((el) => {
      el.scrollLeft = 9999;
      return el.scrollLeft;
    });
    expect(scrolled).toBe(0);
  });

  test("viewport overflow is clip (not hidden)", async ({ page }) => {
    await gotoAndWait(page);
    const viewport = mainCarousel(page).locator(
      '[data-slot="carousel-viewport"]',
    );
    const overflowX = await viewport.evaluate(
      (el) => getComputedStyle(el).overflowX,
    );
    expect(overflowX).toBe("clip");
  });
});

// ---------------------------------------------------------------------------
// Vertical orientation — same multi-visible boundary maths as horizontal,
// but the track is translated on Y. Guards the regression where the vertical
// carousel scrolled past the last full group into empty space.
// ---------------------------------------------------------------------------

test.describe("carousel: vertical orientation", () => {
  test("next stops at the last full group and clamps translateY", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const car = variantCarousel(page, "orientation");
    const next = car.locator('[data-slot="carousel-next"]');
    const content = car.locator('[data-slot="carousel-content"]');

    await clickUntilDisabled(next);

    // 5 slides, 2 per view → max index 3, step = 100/2 % → clamps at -150%.
    await expect(next).toBeDisabled();
    await expect.poll(() => translatePct(content)).toBeCloseTo(-150, 1);
  });

  test("prev returns to the start and disables", async ({ page }) => {
    await gotoAndWait(page);
    const car = variantCarousel(page, "orientation");
    const next = car.locator('[data-slot="carousel-next"]');
    const prev = car.locator('[data-slot="carousel-previous"]');
    const content = car.locator('[data-slot="carousel-content"]');

    await next.click();
    await expect(prev).toBeEnabled();
    await clickUntilDisabled(prev);

    await expect(prev).toBeDisabled();
    await expect.poll(() => translatePct(content)).toBeCloseTo(0, 1);
  });

  test("viewport is not scrollable on the Y axis", async ({ page }) => {
    await gotoAndWait(page);
    const viewport = variantCarousel(page, "orientation").locator(
      '[data-slot="carousel-viewport"]',
    );
    const scrolled = await viewport.evaluate((el) => {
      el.scrollTop = 9999;
      return el.scrollTop;
    });
    expect(scrolled).toBe(0);
  });
});
