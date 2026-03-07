import { test, expect } from '@playwright/test';

test('basic slider structure and keyboard navigation', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/block?name=slider&variant=main&', { timeout: 20 * 60 * 1000 });

  // Root: data-slot, rendered as span, data-orientation
  const slider = page.locator('[data-slot="slider"]');
  await expect(slider).toBeVisible();
  await expect(slider).toHaveAttribute('data-orientation', 'horizontal');
  // Root should not have role="group" (Radix doesn't)
  expect(await slider.getAttribute('role')).toBeNull();

  // Styled layer classes
  const sliderClass = await slider.getAttribute('class');
  expect(sliderClass).toContain('relative');
  expect(sliderClass).toContain('flex');
  expect(sliderClass).toContain('touch-none');

  // Track
  const track = slider.locator('[data-slot="slider-track"]');
  await expect(track).toBeVisible();
  const trackClass = await track.getAttribute('class');
  expect(trackClass).toContain('rounded-full');
  expect(trackClass).toContain('bg-muted');

  // Range
  const range = slider.locator('[data-slot="slider-range"]');
  await expect(range).toBeVisible();
  const rangeClass = await range.getAttribute('class');
  expect(rangeClass).toContain('bg-primary');

  // Thumb: role=slider, aria attributes
  const thumb = slider.locator('[data-slot="slider-thumb"]');
  await expect(thumb).toHaveAttribute('role', 'slider');
  const thumbClass = await thumb.getAttribute('class');
  expect(thumbClass).toContain('rounded-full');
  expect(thumbClass).toContain('border-primary');

  // Initial value should be 50
  await expect(thumb).toHaveAttribute('aria-valuenow', '50');
  await expect(thumb).toHaveAttribute('aria-valuemin', '0');
  await expect(thumb).toHaveAttribute('aria-valuemax', '100');

  // Should not be disabled
  expect(await slider.getAttribute('data-disabled')).toBeNull();
  expect(await thumb.getAttribute('data-disabled')).toBeNull();

  // Keyboard navigation
  await thumb.focus();

  // Shift+ArrowRight: increases by 10
  await page.keyboard.press('Shift+ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');

  await page.keyboard.press('Shift+ArrowRight');
  await expect(thumb).toHaveAttribute('aria-valuenow', '70');

  // Shift+ArrowLeft: decreases by 10
  await page.keyboard.press('Shift+ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '60');

  // ArrowLeft: decreases by step (1)
  await page.keyboard.press('ArrowLeft');
  await expect(thumb).toHaveAttribute('aria-valuenow', '59');

  // ArrowRight: increases by step (1)
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
