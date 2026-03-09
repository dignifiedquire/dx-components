import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/docs/components/scroll_area', { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const scrollArea = page.locator('[data-slot="scroll-area"]');
  await expect(scrollArea).toBeVisible();

  const scrollClass = await scrollArea.getAttribute('class');
  expect(scrollClass).toContain('relative');

  // Assert scroll direction attribute
  await expect(scrollArea).toHaveAttribute('data-scroll-direction', 'vertical');

  // Assert content is rendered
  await expect(page.getByText('Scrollable content item 1')).toBeVisible();
});
