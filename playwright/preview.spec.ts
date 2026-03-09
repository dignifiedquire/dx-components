import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test.describe("homepage", () => {
  test("should not have any automatically detectable accessibility issues", async ({
    page,
  }) => {
    await page.goto("http://127.0.0.1:8080/", { timeout: 20 * 60 * 1000 });

    // Wait for the page to fully load
    await page.getByRole("heading", { name: "Build your component library" }).waitFor({ state: "visible", timeout: 60_000 });

    const accessibilityScanResults = await new AxeBuilder({ page })
      .disableRules("color-contrast")
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});


test.describe("details", () => {
  test("should not have any automatically detectable accessibility issues", async ({
    page,
  }) => {
    await page.goto("http://127.0.0.1:8080/docs/components/separator", { timeout: 20 * 60 * 1000 });

    // Wait for the page to fully load
    await page.locator('[data-slot="preview"]').first().waitFor({ state: "visible", timeout: 60_000 });

    // Scope a11y scan to just the component preview area
    const accessibilityScanResults = await new AxeBuilder({ page })
      .include('[data-slot="preview"]')
      .disableRules(["color-contrast"])
      .analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
});
