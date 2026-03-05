import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=main&', { timeout: 20 * 60 * 1000 });

  // data-slot assertions
  const slider = page.locator('[data-slot="slider"]');
  await expect(slider).toBeVisible();

  const sliderClass = await slider.getAttribute('class');
  expect(sliderClass).toContain('relative');
  expect(sliderClass).toContain('flex');
  expect(sliderClass).toContain('touch-none');

  const track = slider.locator('[data-slot="slider-track"]');
  await expect(track).toBeVisible();
  const trackClass = await track.getAttribute('class');
  expect(trackClass).toContain('rounded-full');
  expect(trackClass).toContain('bg-muted');

  const range = slider.locator('[data-slot="slider-range"]');
  await expect(range).toBeVisible();
  const rangeClass = await range.getAttribute('class');
  expect(rangeClass).toContain('bg-primary');

  const thumb = slider.locator('[data-slot="slider-thumb"]');
  const thumbClass = await thumb.getAttribute('class');
  expect(thumbClass).toContain('rounded-full');
  expect(thumbClass).toContain('border-primary');
  // The initial aria-valuenow should be 50
  await expect(thumb).toHaveAttribute('aria-valuenow', '50');
  await thumb.focus();
  // The aria-valuenow should be 60 after pressing Shift+ArrowRight
  await page.keyboard.press('Shift+ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
  await page.keyboard.press('Shift+ArrowRight');
  // The aria-valuenow should be 70 after pressing Shift+ArrowRight again
  await expect(thumb).toHaveAttribute('aria-valuenow', '70');
  // Pressing Shift+ArrowLeft should decrease the value by 10
  await page.keyboard.press('Shift+ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
  // Pressing ArrowLeft should decrease the value by 1
  await page.keyboard.press('ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '59');
  // Pressing ArrowRight should increase the value by 1
  await page.keyboard.press('ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');
});

test('dynamic min/max', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=dynamic_range&', { timeout: 20 * 60 * 1000 });
  const slider = page.locator('[data-slot="slider"]');
  const thumb = slider.locator('[data-slot="slider-thumb"]');

  // Initial state: percentage mode (0-100)
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '100');

  // Switch to absolute number mode
  await page.getByRole('switch', { name: 'Percentage' }).click();

  // Should now be absolute mode (0-1000)
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '1000');

  // Click back to percentage mode
  await page.getByRole('switch', { name: 'Percentage' }).click();

  // Should be back to percentage mode (0-100)
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '100');
});

