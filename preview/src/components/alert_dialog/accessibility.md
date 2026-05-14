Adheres to the [Alert and Message Dialogs WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/alertdialog). The content renders as a native `<dialog>` element with `role="alertdialog"` and `aria-modal="true"`, opened via `showModal()` — focus trap, inert siblings, and ESC handling are provided by the browser.

### Keyboard Interactions

| Key | Description |
| --- | --- |
| `Space` | Opens / closes the dialog when focus is on `AlertDialogTrigger`. |
| `Enter` | Opens / closes the dialog when focus is on `AlertDialogTrigger`. |
| `Tab` | Moves focus to the next focusable element inside the dialog (focus is trapped). |
| `Shift + Tab` | Moves focus to the previous focusable element inside the dialog. |
| `Esc` | Closes the dialog and moves focus back to `AlertDialogTrigger`. |
