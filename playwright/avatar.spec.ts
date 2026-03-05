import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=avatar&", { timeout: 20 * 60 * 1000 });

  // data-slot attribute on avatar root
  const avatar = page.locator('[data-slot="avatar"]').first();
  await expect(avatar).toBeVisible();
  await expect(avatar).toHaveAttribute('data-slot', 'avatar');

  // Key Tailwind classes from shadcn
  const classAttr = await avatar.getAttribute('class');
  expect(classAttr).toContain('flex');
  expect(classAttr).toContain('overflow-hidden');
  expect(classAttr).toContain('rounded-full');

  // Avatar image has data-slot
  const image = avatar.locator('[data-slot="avatar-image"]');
  await expect(image).toBeVisible();
  await expect(image).toHaveAttribute("src", "https://avatars.githubusercontent.com/u/66571940?s=96&v=4");

  const imageClass = await image.getAttribute('class');
  expect(imageClass).toContain('aspect-square');

  // Error state avatar shows fallback with data-slot
  const errorAvatar = page.locator('[data-slot="avatar"]').nth(2);
  await expect(errorAvatar).toContainText("JK");

  const fallback = errorAvatar.locator('[data-slot="avatar-fallback"]');
  await expect(fallback).toBeVisible();

  const fallbackClass = await fallback.getAttribute('class');
  expect(fallbackClass).toContain('bg-muted');
});
