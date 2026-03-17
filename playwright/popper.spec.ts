import { test, expect } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/popper";
const TIMEOUT = { timeout: 20 * 60 * 1000 };
const EXPECT_TIMEOUT = { timeout: 15_000 };

/** Navigate, wait for WASM hydration, and wait for popper positioning. */
async function gotoAndWait(page: import("@playwright/test").Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-testid="popper-demos"]')
    .waitFor({ state: "visible", timeout: 60_000 });
  // Wait for at least one popper wrapper to be positioned (no -200% fallback)
  await page.waitForFunction(
    () => {
      const wrappers = document.querySelectorAll(
        "[data-radix-popper-content-wrapper]",
      );
      if (wrappers.length === 0) return false;
      return Array.from(wrappers).some((w) => {
        const style = w.getAttribute("style") || "";
        return style.includes("translate(") && !style.includes("-200%");
      });
    },
    { timeout: 15_000 },
  );
}

// ---------------------------------------------------------------------------
// Upstream: popper.stories.tsx — "Styled"
// ---------------------------------------------------------------------------

test.describe("Popper: styled", () => {
  test("renders anchor and floating content", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    await expect(section.locator("button")).toBeVisible();
    const content = section.locator("[data-side]");
    await expect(content).toBeVisible(EXPECT_TIMEOUT);
  });

  test("wrapper has data-radix-popper-content-wrapper", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const wrapper = section.locator("[data-radix-popper-content-wrapper]");
    await expect(wrapper).toBeAttached(EXPECT_TIMEOUT);
  });

  test("has data-side and data-align attributes", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const content = section.locator("[data-side]");
    // Flip middleware may change the actual side based on available viewport space
    const side = await content.getAttribute("data-side");
    expect(["top", "right", "bottom", "left"]).toContain(side);
    const align = await content.getAttribute("data-align");
    expect(["start", "center", "end"]).toContain(align);
  });

  test("wrapper has fixed positioning", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const wrapper = section.locator("[data-radix-popper-content-wrapper]");
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("position: fixed");
  });
});

// ---------------------------------------------------------------------------
// Upstream: popper.stories.tsx — sides
// ---------------------------------------------------------------------------

test.describe("Popper: sides", () => {
  test("side=top sets data-side to top", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="side-top"]');
    const content = section.locator("[data-side]");
    await expect(content).toHaveAttribute("data-side", "top", EXPECT_TIMEOUT);
  });

  test("side=right sets data-side to right", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="side-right"]');
    const content = section.locator("[data-side]");
    await expect(content).toHaveAttribute(
      "data-side",
      "right",
      EXPECT_TIMEOUT,
    );
  });

  test("side=left sets data-side to left", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="side-left"]');
    const content = section.locator("[data-side]");
    await expect(content).toHaveAttribute("data-side", "left", EXPECT_TIMEOUT);
  });
});

// ---------------------------------------------------------------------------
// Upstream: popper.stories.tsx — alignments
// ---------------------------------------------------------------------------

test.describe("Popper: alignments", () => {
  test("align=start sets data-align to start", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="align-start"]');
    const content = section.locator("[data-side]");
    await expect(content).toHaveAttribute(
      "data-align",
      "start",
      EXPECT_TIMEOUT,
    );
  });

  test("align=end sets data-align to end", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="align-end"]');
    const content = section.locator("[data-side]");
    await expect(content).toHaveAttribute("data-align", "end", EXPECT_TIMEOUT);
  });
});

// ---------------------------------------------------------------------------
// Upstream: popper.stories.tsx — "WithCustomArrow"
// ---------------------------------------------------------------------------

test.describe("Popper: with arrow", () => {
  test("arrow has data-slot=popper-arrow", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="with-arrow"]');
    const arrow = section.locator('[data-slot="popper-arrow"]');
    await expect(arrow).toBeAttached(EXPECT_TIMEOUT);
  });

  test("arrow contains an SVG element", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="with-arrow"]');
    const svg = section.locator('[data-slot="popper-arrow"] svg');
    await expect(svg).toBeAttached(EXPECT_TIMEOUT);
  });

  test("arrow SVG has correct dimensions", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="with-arrow"]');
    const svg = section.locator('[data-slot="popper-arrow"] svg');
    await expect(svg).toHaveAttribute("width", "20");
    await expect(svg).toHaveAttribute("height", "10");
  });
});

// ---------------------------------------------------------------------------
// CSS variables on wrapper
// ---------------------------------------------------------------------------

test.describe("Popper: CSS variables", () => {
  test("wrapper has --radix-popper-transform-origin", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const wrapper = section.locator("[data-radix-popper-content-wrapper]");
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("--radix-popper-transform-origin");
  });

  test("wrapper has --radix-popper-available-width", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const wrapper = section.locator("[data-radix-popper-content-wrapper]");
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("--radix-popper-available-width");
  });

  test("wrapper has --radix-popper-available-height", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const wrapper = section.locator("[data-radix-popper-content-wrapper]");
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("--radix-popper-available-height");
  });

  test("wrapper has --radix-popper-anchor-width", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const wrapper = section.locator("[data-radix-popper-content-wrapper]");
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("--radix-popper-anchor-width");
  });

  test("wrapper has --radix-popper-anchor-height", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="styled"]');
    const wrapper = section.locator("[data-radix-popper-content-wrapper]");
    const style = await wrapper.getAttribute("style");
    expect(style).toContain("--radix-popper-anchor-height");
  });
});

// ---------------------------------------------------------------------------
// Upstream: popper.stories.tsx — "WithPortal"
// ---------------------------------------------------------------------------

test.describe("Popper: with portal", () => {
  test("portal anchor is visible in section", async ({ page }) => {
    await gotoAndWait(page);
    const section = page.locator('[data-testid="with-portal"]');
    await expect(section.locator("button")).toBeVisible();
  });

  test("portal content renders", async ({ page }) => {
    await gotoAndWait(page);
    const inner = page.locator('[data-testid="portal-content-inner"]');
    await expect(inner).toBeAttached(EXPECT_TIMEOUT);
  });
});
