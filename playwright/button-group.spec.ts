import { test, expect, type Page } from "@playwright/test";

const URL = "http://127.0.0.1:8080/docs/components/button_group";
const TIMEOUT = { timeout: 20 * 60 * 1000 };

async function gotoAndWait(page: Page) {
  await page.goto(URL, TIMEOUT);
  await page
    .locator('[data-slot="button-group"]')
    .first()
    .waitFor({ state: "visible", timeout: 60_000 });
}

test.describe("button_group rendering", () => {
  test("root has data-slot, role=group and horizontal orientation", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const group = page
      .locator('[data-slot="preview"] [data-slot="button-group"]')
      .first();
    await expect(group).toBeVisible();
    await expect(group).toHaveAttribute("role", "group");
    await expect(group).toHaveAttribute("data-orientation", "horizontal");
  });

  test("main demo wires buttons inside the group", async ({ page }) => {
    await gotoAndWait(page);
    const preview = page.locator('[data-slot="preview"]').first();
    await expect(
      preview.getByRole("button", { name: "Archive" }),
    ).toBeVisible();
    await expect(preview.getByRole("button", { name: "Report" })).toBeVisible();
    await expect(preview.getByRole("button", { name: "Snooze" })).toBeVisible();
  });
});

test.describe("button_group variants", () => {
  test("orientation variant is vertical", async ({ page }) => {
    await gotoAndWait(page);
    const group = page.locator(
      '#orientation [data-slot="button-group"]',
    );
    await expect(group.first()).toHaveAttribute(
      "data-orientation",
      "vertical",
    );
  });

  test("separator variant renders a decorative divider", async ({ page }) => {
    await gotoAndWait(page);
    const sep = page
      .locator('#separator [data-slot="button-group-separator"]')
      .first();
    await expect(sep).toBeAttached();
    await expect(sep).toHaveAttribute("role", "none");
  });

  test("split variant pairs a button and an icon button via a separator", async ({
    page,
  }) => {
    await gotoAndWait(page);
    const group = page.locator('#split [data-slot="button-group"]').first();
    await expect(
      group.locator('[data-slot="button-group-separator"]'),
    ).toBeAttached();
    await expect(group.getByRole("button")).toHaveCount(2);
  });
});
