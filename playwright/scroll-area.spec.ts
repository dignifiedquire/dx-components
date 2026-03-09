import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/scroll_area', { timeout: 20 * 60 * 1000 });

  // Scope to first preview block
  const preview = page.locator('[data-slot="preview"]').first();

  // data-slot assertions
  const scrollArea = preview.locator('[data-slot="scroll-area"]').first();
  await expect(scrollArea).toBeVisible();

  const scrollClass = await scrollArea.getAttribute('class');
  expect(scrollClass).toContain('relative');

  // Assert scroll direction attribute
  await expect(scrollArea).toHaveAttribute('data-scroll-direction', 'vertical');

  // Assert content is rendered
  await expect(preview.getByText('Scrollable content item 1', { exact: true })).toBeVisible();
});
