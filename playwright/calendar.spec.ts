import { test, expect } from "@playwright/test";

test("data slots and classes", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", {
    timeout: 20 * 60 * 1000,
  });

  const calendar = page.locator('[data-slot="calendar"]').nth(0);
  await expect(calendar).toBeVisible({ timeout: 30000 });

  const calendarClass = await calendar.getAttribute("class");
  expect(calendarClass).toContain("flex");
  expect(calendarClass).toContain("rounded-lg");
  expect(calendarClass).toContain("border");
  expect(calendarClass).toContain("bg-background");

  // Assert navigation data-slot
  const nav = calendar.locator('[data-slot="calendar-navigation"]');
  await expect(nav).toBeVisible();

  // Assert nav button data-slots
  await expect(calendar.locator('[data-slot="calendar-nav-prev"]')).toBeVisible();
  await expect(calendar.locator('[data-slot="calendar-nav-next"]')).toBeVisible();

  // Assert grid data-slot
  const grid = calendar.locator('[data-slot="calendar-grid"]');
  await expect(grid).toBeVisible();

  // Assert day data-slots
  const days = calendar.locator('[data-slot="calendar-day"]');
  expect(await days.count()).toBeGreaterThan(0);

  const dayClass = await days.first().getAttribute("class");
  expect(dayClass).toContain("rounded-lg");
  expect(dayClass).toContain("text-sm");
});

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", {
    timeout: 20 * 60 * 1000,
  });
  const calendar = page.locator('[data-slot="calendar"]').nth(0);
  const prevButton = calendar.locator('[data-slot="calendar-nav-prev"]');
  const nextButton = calendar.locator('[data-slot="calendar-nav-next"]');

  await expect(calendar).toBeVisible({ timeout: 30000 });
  const currentMonth = calendar.locator('[data-slot="calendar-select-month"] select');
  let currentMonthText = await currentMonth.inputValue();

  // Click the previous button to go to the previous month
  await prevButton.click();
  let previousMonthText = await currentMonth.inputValue();
  expect(previousMonthText).not.toBe(currentMonthText);

  // Click the next button to go back to the current month
  await nextButton.click();
  await expect(currentMonth).toHaveValue(currentMonthText);

  // Move focus to the calendar with tab
  await page.keyboard.press("Tab");
  const focusedDay = calendar.locator(
    '[data-slot="calendar-day"][data-month="current"]:focus'
  );
  const firstDay = focusedDay.first();
  const day = await firstDay.textContent();
  const dayNumber = parseInt(day || "", 10);

  // Pressing right arrow should move focus to the next day
  await page.keyboard.press("ArrowRight");
  const nextDay = focusedDay.first();
  const nextDayNumber = parseInt((await nextDay.textContent()) || "", 10);
  let current_date = new Date();
  let daysInMonth = new Date(
    current_date.getFullYear(),
    current_date.getMonth() + 1,
    0
  ).getDate();
  if (dayNumber + 1 > daysInMonth) {
    expect(nextDayNumber).toBe(1);
  } else {
    expect(nextDayNumber).toBe(dayNumber + 1);
  }

  // Pressing left arrow should move focus back to the original day
  await page.keyboard.press("ArrowLeft");
  await expect(focusedDay.first()).toContainText(day || "failure");

  // Pressing down arrow should move focus to the next week
  await page.keyboard.press("ArrowDown");
  const nextWeekDay = focusedDay.first();
  const nextWeekDayNumber = parseInt(
    (await nextWeekDay.textContent()) || "",
    10
  );
  if (dayNumber + 7 > daysInMonth) {
    expect(nextWeekDayNumber).toBe(dayNumber + 7 - daysInMonth);
  } else {
    expect(nextWeekDayNumber).toBe(dayNumber + 7);
  }

  // Pressing up arrow should move focus back to the original day
  await page.keyboard.press("ArrowUp");
  await expect(focusedDay.first()).toContainText(day || "failure");
});

test("year navigation by moving 52 weeks with arrow keys", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", {
    timeout: 20 * 60 * 1000,
  });

  const calendar = page.locator('[data-slot="calendar"]').nth(0);
  const monthSelect = calendar.locator('[data-slot="calendar-select-month"] select');
  const yearSelect = calendar.locator('[data-slot="calendar-select-year"] select');

  await expect(calendar).toBeVisible({ timeout: 30000 });

  const initialMonth = await monthSelect.inputValue();
  const initialYear = await yearSelect.inputValue();
  const initialYearNumber = parseInt(initialYear, 10);
  const initialMonthNumber = parseInt(initialMonth, 10);

  const startDate = new Date(initialYearNumber, initialMonthNumber - 1, 1);
  const targetDate = new Date(initialYearNumber + 1, initialMonthNumber - 1, 1);
  const daysDifference = Math.floor(
    (targetDate.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24)
  );
  const weeksToMove = Math.ceil(daysDifference / 7);

  const firstDay = calendar
    .locator('[data-slot="calendar-day"][data-month="current"]')
    .first();
  await firstDay.focus();

  for (let i = 0; i < weeksToMove; i++) {
    await page.keyboard.press("ArrowDown");
  }

  const nextYear = await yearSelect.inputValue();
  const nextYearNumber = parseInt(nextYear, 10);
  expect(nextYearNumber).toBe(initialYearNumber + 1);

  const nextMonth = await monthSelect.inputValue();
  expect(nextMonth).toBe(initialMonth);

  for (let i = 0; i < weeksToMove; i++) {
    await page.keyboard.press("ArrowUp");
  }

  const currentYear = await yearSelect.inputValue();
  expect(currentYear).toBe(initialYear);

  const currentMonth = await monthSelect.inputValue();
  expect(currentMonth).toBe(initialMonth);
});

test("shift + arrow keys navigation", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", {
    timeout: 20 * 60 * 1000,
  });

  const calendar = page.locator('[data-slot="calendar"]').nth(0);
  const monthSelect = calendar.locator('[data-slot="calendar-select-month"] select');
  const yearSelect = calendar.locator('[data-slot="calendar-select-year"] select');

  await expect(calendar).toBeVisible({ timeout: 30000 });

  const initialMonth = await monthSelect.inputValue();
  const initialYear = await yearSelect.inputValue();
  const initialYearNumber = parseInt(initialYear, 10);
  const initialMonthNumber = parseInt(initialMonth, 10);

  const firstDay = calendar
    .locator('[data-slot="calendar-day"][data-month="current"]')
    .first();
  await firstDay.focus();

  await page.keyboard.press("Shift+ArrowDown");

  let currentMonth = await monthSelect.inputValue();
  let currentYear = await yearSelect.inputValue();
  let expectedMonth = initialMonthNumber === 12 ? 1 : initialMonthNumber + 1;
  let expectedYear =
    initialMonthNumber === 12 ? initialYearNumber + 1 : initialYearNumber;

  expect(parseInt(currentMonth, 10)).toBe(expectedMonth);
  expect(parseInt(currentYear, 10)).toBe(expectedYear);

  await page.keyboard.press("Shift+ArrowUp");

  currentMonth = await monthSelect.inputValue();
  currentYear = await yearSelect.inputValue();
  expect(currentMonth).toBe(initialMonth);
  expect(currentYear).toBe(initialYear);
});

async function testArrowKeyNavigation(
  page: any,
  arrowKey: "ArrowRight" | "ArrowLeft",
  startPosition: "first" | "last",
  expectedOrder: "ascending" | "descending"
) {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", {
    timeout: 20 * 60 * 1000,
  });

  const calendar = page.locator('[data-slot="calendar"]').nth(0);
  const monthSelect = calendar.locator('[data-slot="calendar-select-month"] select');
  const yearSelect = calendar.locator('[data-slot="calendar-select-year"] select');

  await expect(calendar).toBeVisible({ timeout: 30000 });

  const currentMonthValue = await monthSelect.inputValue();
  const currentYearValue = await yearSelect.inputValue();
  const monthNumber = parseInt(currentMonthValue, 10);
  const yearNumber = parseInt(currentYearValue, 10);

  const daysInMonth = new Date(yearNumber, monthNumber, 0).getDate();

  const startDay = calendar
    .locator('[data-slot="calendar-day"][data-month="current"]')
    [startPosition]();
  await startDay.focus();

  const focusedDay = calendar.locator(
    '[data-slot="calendar-day"][data-month="current"]:focus'
  );

  const daysVisited = [];

  let dayText = await focusedDay.first().textContent();
  let dayNumber = parseInt(dayText || "", 10);
  daysVisited.push(dayNumber);

  for (let i = 1; i < daysInMonth; i++) {
    await page.keyboard.press(arrowKey);

    dayText = await focusedDay.first().textContent();
    dayNumber = parseInt(dayText || "", 10);
    daysVisited.push(dayNumber);
  }

  expect(daysVisited.length).toBe(daysInMonth);

  const sortedDays = [...daysVisited].sort((a, b) => a - b);
  const expectedDays = Array.from({ length: daysInMonth }, (_, i) => i + 1);

  expect(sortedDays).toEqual(expectedDays);

  if (expectedOrder === "ascending") {
    expect(daysVisited).toEqual(expectedDays);
  } else {
    const expectedReverseDays = Array.from(
      { length: daysInMonth },
      (_, i) => daysInMonth - i
    );
    expect(daysVisited).toEqual(expectedReverseDays);
  }
}

test("right arrow key navigates through all days of the month", async ({
  page,
}) => {
  await testArrowKeyNavigation(page, "ArrowRight", "first", "ascending");
});

test("left arrow key navigates through all days of the month in reverse", async ({
  page,
}) => {
  await testArrowKeyNavigation(page, "ArrowLeft", "last", "descending");
});
