# Phase 3: shadcn Styled Layers — COMPLETE

> **Updated 2026-03-12**: All styled layers are done. The formerly "blocked" and "deferred" items
> have all been implemented. Only Chart remains (blocked on charting library — tracked in Phase A
> of `unified-completion-plan.md`).

All `components/src/{component}.rs` files exist for every shadcn/ui v4 component (except Chart).
Each imports from `dioxus_primitives` and adds shadcn classes via `tw_merge!` / `TwVariant` / `TwClass`.

**Source**: `../shadcn-ui/apps/v4/registry/new-york-v4/ui/` (56 component files)

---

## Final Status

| Category | Done | Total |
|---|---|---|
| 3a. Radix-backed styled | 23 | 23 |
| 3b. HTML-only styled | 10 | 10 |
| 3c. Dioxus-only | 4 | 4 |
| 3d. HTML-only (new) | 8 | 8 |
| 3e. Composite | 4 | 4 |
| 3f. Former "deferred" | 6 | 7 |
| **Total** | **55** | **56** |

The one remaining component (Chart) is tracked separately as it requires a charting/visualization library that doesn't exist for Dioxus yet.

---

## 3a. shadcn components backed by Radix primitives — ALL DONE

| shadcn Component | File | Notes |
|---|---|---|
| Accordion | `accordion.rs` | TwClass + chevron icon |
| AlertDialog | `alert_dialog.rs` | Overlay + Content + Action/Cancel |
| AspectRatio | `aspect_ratio.rs` | Thin passthrough |
| Avatar | `avatar.rs` | AvatarSize enum |
| Button | `button.rs` | TwVariant/TwClass, 6 variants x 5 sizes |
| Calendar | `calendar.rs` + `calendar.css` | CSS data-slot selectors |
| Checkbox | `checkbox.rs` | Composes indicator + CheckIcon |
| Collapsible | `collapsible.rs` | Thin passthrough |
| ContextMenu | `context_menu.rs` | Menu sub-component classes |
| Dialog | `dialog.rs` | Overlay + animate-in/out |
| DropdownMenu | `dropdown_menu.rs` | ~12 sub-component classes |
| HoverCard | `hover_card.rs` | Content positioning |
| Label | `label.rs` | Disabled/peer-disabled classes |
| Menubar | `menubar.rs` | Menu sub-component classes |
| NavigationMenu | `navigation_menu.rs` | Viewport animation |
| Popover | `popover.rs` | Slide-in/out animations |
| Progress | `progress.rs` | translateX transform |
| RadioGroup | `radio_group.rs` | Composed indicator + CircleIcon |
| ScrollArea | `scroll_area.rs` | Orientation variants |
| Select | `select.rs` | Trigger + Content/Item classes |
| Separator | `separator.rs` | Defaults Horizontal + decorative |
| Slider | `slider.rs` | Track/Range/Thumb classes |
| Switch | `switch.rs` | SwitchSize enum |
| Tabs | `tabs.rs` | TabsListVariant (Default/Line) |
| Toast | `toast.rs` | Toaster with variant icons |
| Toggle | via `toggle_group.rs` | ToggleVariant/ToggleSize context |
| ToggleGroup | `toggle_group.rs` | Context-based variant/size |
| Tooltip | `tooltip.rs` | z-50 positioning |

## 3b. HTML-only styled — ALL DONE

Alert, Badge, Card, Input, Kbd, Skeleton, Spinner, Table, Textarea

## 3c. Dioxus-only styled — ALL DONE

DatePicker, DragAndDropList, Navbar, Toolbar

## 3d. HTML-only new — ALL DONE

Breadcrumb, Pagination, NativeSelect, Empty, Item, ButtonGroup, InputGroup, Field

## 3e. Composite — ALL DONE

Form, Sheet, Sidebar, NavigationMenu

## 3f. Former "deferred" — DONE (except Chart)

Carousel, Command, Combobox, Drawer, InputOTP, Resizable — all implemented as Dioxus-native equivalents.

Chart — still deferred (needs charting library).
