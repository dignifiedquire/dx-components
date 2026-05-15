import { test, expect } from '@playwright/test';

const URL = 'http://127.0.0.1:8080/docs/components/button';
const TIMEOUT = { timeout: 20 * 60 * 1000 };

test('main demo mirrors shadcn button-demo (outline text + icon)', async ({ page }) => {
  await page.goto(URL, TIMEOUT);

  // First preview block = main demo: an outline text button + an
  // outline icon-only button (aria-label "Submit"), per button-demo.tsx.
  const preview = page.locator('[data-slot="preview"]').first();

  const textBtn = preview.getByRole('button', { name: 'Button' });
  await expect(textBtn).toBeVisible();
  await expect(textBtn).toHaveAttribute('data-slot', 'button');
  await expect(textBtn).toHaveAttribute('data-variant', 'outline');
  await expect(textBtn).toHaveAttribute('data-size', 'default');

  const iconBtn = preview.getByRole('button', { name: 'Submit' });
  await expect(iconBtn).toBeVisible();
  await expect(iconBtn).toHaveAttribute('data-variant', 'outline');
  await expect(iconBtn).toHaveAttribute('data-size', 'icon');

  // Clickable and focusable
  await textBtn.click();
  await expect(textBtn).toBeVisible();
  await textBtn.focus();
  await expect(textBtn).toBeFocused();
});

test('sizes variant exposes the radix-flavor size scale', async ({ page }) => {
  await page.goto(URL, TIMEOUT);
  await page.locator('[data-slot="preview"]').first().waitFor({ state: 'visible', timeout: 60_000 });

  await expect(
    page.getByRole('button', { name: 'Extra Small', exact: true }),
  ).toHaveAttribute('data-size', 'xs');
  await expect(
    page.getByRole('button', { name: 'Small', exact: true }),
  ).toHaveAttribute('data-size', 'sm');
  await expect(
    page.getByRole('button', { name: 'Large', exact: true }),
  ).toHaveAttribute('data-size', 'lg');
});
