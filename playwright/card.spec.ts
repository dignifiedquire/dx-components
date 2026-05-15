import { test, expect } from '@playwright/test';

test('card renders with correct data-slot attributes and structure', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/card', { timeout: 20 * 60 * 1000 });

  // Scope to the first preview block (main demo = the login card) — the
  // page renders several card previews (size / image / rtl).
  const preview = page.locator('[data-slot="preview"]').first();

  // Card root — radix-flavor classes (no border / shadow-sm; uses a ring).
  const card = preview.locator('[data-slot="card"]').first();
  await expect(card).toBeVisible();
  await expect(card).toHaveClass(/rounded-xl/);
  await expect(card).toHaveClass(/ring-1/);
  await expect(card).toHaveClass(/ring-foreground\/10/);

  // Header / title / description / action
  await expect(preview.locator('[data-slot="card-header"]').first()).toBeVisible();
  const title = preview.locator('[data-slot="card-title"]').first();
  await expect(title).toBeVisible();
  await expect(title).toContainText('Login to your account');

  const description = preview.locator('[data-slot="card-description"]').first();
  await expect(description).toBeVisible();
  await expect(description).toHaveClass(/text-muted-foreground/);

  await expect(preview.locator('[data-slot="card-action"]').first()).toBeVisible();
  await expect(preview.locator('[data-slot="card-content"]').first()).toBeVisible();
  await expect(preview.locator('[data-slot="card-footer"]').first()).toBeVisible();

  // The login form has two inputs.
  await expect(preview.locator('[data-slot="input"]')).toHaveCount(2);
});
