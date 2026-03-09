# Phase 3: shadcn Styled Layers

Create `components/src/{component}.rs` for each shadcn/ui v4 component, matching their Tailwind classes exactly. Each imports from `dioxus_primitives` and adds shadcn classes via `tw_merge!` / `TwVariant` / `TwClass`.

**Source**: `../shadcn-ui/apps/v4/registry/new-york-v4/ui/` (56 component files)

---

## Status overview

| Category | Done | Remaining | Total |
|---|---|---|---|
| 3a. Radix-backed styled | 15 | 8 | 23 |
| 3b. HTML-only styled | 10 | 0 | 10 |
| 3c. Dioxus-only | 4 | 0 | 4 |
| 3d. HTML-only (new) | 8 | 0 | 8 |
| 3e. Composite | 0 | 4 | 4 |
| 3f. Deferred | — | 7 | 7 |
| **Total** | **37** | **19** | **56** |

---

## 3a. shadcn components backed by Radix primitives

### DONE (8)

| shadcn Component | File | Notes |
|---|---|---|
| **Accordion** | `accordion.rs` | TwClass + chevron icon |
| **Button** | `button.rs` | TwVariant/TwClass, 6 variants × 5 sizes |
| **Calendar** | `calendar.rs` + `calendar.css` | CSS data-slot selectors |
| **Collapsible** | `collapsible.rs` | Thin passthrough |
| **RadioGroup** | `radio_group.rs` | Composed indicator + CircleIcon |
| **Tabs** | `tabs.rs` | TabsListVariant (Default/Line) |
| **Toggle** | via `toggle_group.rs` | ToggleVariant/ToggleSize context |
| **ToggleGroup** | `toggle_group.rs` | Context-based variant/size pass-down |

### DONE — Phase 3a styled layers (7)

| shadcn Component | File | Notes |
|---|---|---|
| **AspectRatio** | `aspect_ratio.rs` | Thin passthrough |
| **Avatar** | `avatar.rs` | AvatarSize enum, +3 HTML-only (Badge, Group, GroupCount) |
| **Checkbox** | `checkbox.rs` | Composes indicator + CheckIcon internally |
| **Label** | `label.rs` | Disabled/peer-disabled classes |
| **Progress** | `progress.rs` | Composes indicator + translateX transform |
| **Separator** | `separator.rs` | Defaults Horizontal + decorative:true |
| **Switch** | `switch.rs` | SwitchSize enum, composes thumb internally |

### BLOCKED — primitive needs rewrite first (Phase 2d-f) (8)

| shadcn Component | Blocks On | shadcn Key Classes |
|---|---|---|
| **Dialog** | Phase 2d | Overlay: `fixed inset-0 z-50 bg-black/50` + animate-in/out, Content: `fixed top-1/2 left-1/2 z-50 ...` + Header/Footer/Title/Description/Close |
| **AlertDialog** | Phase 2d | Same as Dialog + `role="alertdialog"`, Cancel/Action buttons |
| **Popover** | Phase 2d | Content: `z-50 w-72 origin-(--radix-popover-content-transform-origin) rounded-md border bg-popover p-4 text-popover-foreground shadow-md` + slide-in/out animations |
| **Tooltip** | Phase 2d | Content: `z-50 w-fit origin-(--radix-tooltip-content-transform-origin) animate-in rounded-md bg-foreground px-3 py-1.5 text-xs text-balance text-background` |
| **HoverCard** | Phase 2d | Content: `z-50 w-64 origin-(--radix-hover-card-content-transform-origin) rounded-md border bg-popover p-4 text-popover-foreground shadow-md` |
| **Sheet** | Phase 2d (Dialog) | Side variants (top/right/bottom/left) with slide animations, uses Dialog primitive re-styled |
| **DropdownMenu** | Phase 2e | Content: `z-50 max-h-(--radix-dropdown-menu-content-available-height) min-w-[8rem] ...` + ~12 sub-component classes (Item, CheckboxItem, RadioItem, Label, Separator, Shortcut, SubTrigger, SubContent) |
| **ContextMenu** | Phase 2e | Same menu classes as DropdownMenu |
| **Menubar** | Phase 2e | Root: `flex h-9 items-center gap-1 rounded-md border bg-background p-1 shadow-xs` + menu sub-component classes |
| **Select** | Phase 2f | Trigger: `flex w-fit items-center justify-between gap-2 rounded-md border ...` + Content/Item/Group/Label/Separator classes |
| **Slider** | Phase 2f | Root: `relative flex w-full touch-none items-center select-none`, Track: `relative grow overflow-hidden rounded-full bg-muted`, Range: `absolute bg-primary`, Thumb: `block size-4 shrink-0 rounded-full border bg-background shadow-sm ...` |
| **ScrollArea** | Phase 2f | Root: `relative`, Viewport: `size-full rounded-[inherit]`, ScrollBar: `flex touch-none p-px transition-colors select-none` + orientation variants |
| **Toast/Sonner** | Phase 2f | Custom Toaster with variant icons (success/info/warning/error/loading) |

---

## 3b. shadcn components — styled HTML only (no Radix primitive) — ALL DONE ✓

| shadcn Component | File | Notes |
|---|---|---|
| **Alert** | `alert.rs` | TwVariant/TwClass, 2 variants (Default/Destructive) |
| **Badge** | `badge.rs` | TwVariant/TwClass, 6 variants, data-variant |
| **Card** | `card.rs` | 7 sub-components (Card/Header/Title/Description/Action/Content/Footer) |
| **Input** | `input.rs` | Explicit event handler props (oninput/onchange/onfocus/onblur) |
| **Kbd** | `kbd.rs` | Kbd + KbdGroup |
| **Skeleton** | `skeleton.rs` | animate-pulse div |
| **Spinner** | `spinner.rs` | Inline Lucide loader-circle SVG, animate-spin |
| **Table** | `table.rs` | 8 sub-components (Table/Header/Body/Footer/Row/Head/Cell/Caption) |
| **Textarea** | `textarea.rs` | Explicit event handler props |
| **Button** | `button.rs` | Moved to 3a (Radix-backed) |

---

## 3c. Dioxus-only styled components (no shadcn equivalent) — ALL DONE ✓

| Component | File | Notes |
|---|---|---|
| **DatePicker** | `date_picker.rs` + `date_picker.css` | CSS data-slot selectors |
| **DragAndDropList** | `drag_and_drop_list.rs` + `drag_and_drop_list.css` | CSS data-slot selectors |
| **Navbar** | `navbar.rs` + `navbar.css` | CSS data-slot selectors |
| **Toolbar** | `toolbar.rs` | Thin passthrough with minimal styling |

---

## 3d. shadcn HTML-only components — NEW — ALL DONE ✓

| shadcn Component | File | Sub-components | Notes |
|---|---|---|---|
| **Breadcrumb** | `breadcrumb.rs` | 7 (Breadcrumb, List, Item, Link, Page, Separator, Ellipsis) | ChevronRight + Ellipsis icons |
| **Pagination** | `pagination.rs` | 7 (Pagination, Content, Item, Link, Previous, Next, Ellipsis) | Button variant classes, ChevronLeft/Right icons |
| **NativeSelect** | `native_select.rs` | 3 (NativeSelect, Option, OptGroup) | NativeSelectSize enum, ChevronDown icon |
| **Empty** | `empty.rs` | 6 (Empty, Header, Media, Title, Description, Content) | EmptyMediaVariant (Default/Icon) |
| **Item** | `item.rs` | 10 (Group, Separator, Item, Media, Content, Title, Description, Actions, Header, Footer) | ItemVariant, ItemSize, ItemMediaVariant enums |
| **ButtonGroup** | `button_group.rs` | 3 (ButtonGroup, Text, Separator) | Orientation enum (Horizontal/Vertical) |
| **InputGroup** | `input_group.rs` | 5 (InputGroup, Addon, Text, Input, Textarea) | InputGroupAddonAlign enum |
| **Field** | `field.rs` | 10 (FieldSet, Legend, Group, Field, Content, Label, Title, Description, Error, Separator) | FieldOrientation, FieldLegendVariant enums |

---

## 3e. Composite components (depend on multiple other components)

| shadcn Component | Dependencies | Notes |
|---|---|---|
| **Form** | Label + Field + validation | React Hook Form integration — needs Dioxus form equivalent |
| **Sheet** | Dialog (re-styled) | Side variants (top/right/bottom/left), blocked on Dialog |
| **Sidebar** | Sheet + Tooltip + Button + Separator + Input + Skeleton | Complex layout, many dependencies |
| **NavigationMenu** | New primitive needed | Radix has NavigationMenu primitive — we don't have it yet |

---

## 3f. Deferred (external JS library dependencies)

| shadcn Component | External Dependency | Decision |
|---|---|---|
| Carousel | embla-carousel | Needs Dioxus-native impl |
| Chart | recharts | Needs Dioxus-native impl |
| Command | cmdk | Needs Dioxus-native impl |
| Combobox | @base-ui/react | Could build with Select + Input |
| Drawer | vaul | Could approximate with Dialog + gestures |
| InputOTP | input-otp | Could build custom |
| Resizable | react-resizable-panels | Needs Dioxus-native impl |

---

## Execution order

```
DONE: Button, Collapsible, Calendar, DatePicker, Navbar, DragAndDropList (commits 399cc7f, ca22a9f)
DONE: Accordion (components/src/accordion.rs)
DONE: Tabs, RadioGroup, ToggleGroup, Toolbar (commit c98851d)
DONE: HTML-only: Alert, Badge, Card, Input, Kbd, Skeleton, Spinner, Table, Textarea (commit afbebe3)
DONE: Radix-backed: AspectRatio, Avatar, Checkbox, Label, Progress, Separator, Switch (commit 2c7954a)
DONE: HTML-only new: Breadcrumb, Pagination, NativeSelect, Empty, Item, ButtonGroup, InputGroup, Field
1. Phase 2d primitives → Dialog, AlertDialog, Popover, Tooltip, HoverCard styled layers  ← NEXT
4. Phase 2e primitives → DropdownMenu, ContextMenu, Menubar styled layers
5. Phase 2f primitives → Select, Slider, ScrollArea, Toast/Sonner styled layers
6. Composites: Sheet (Dialog re-styled), Form, Sidebar
7. NavigationMenu (new primitive + styled layer)
```

---

## Per-component deliverables

1. `components/src/{component}.rs` — shadcn classes via `tw_merge!` / `TwVariant` / `TwClass`
2. `components/tests/{component}.rs` — SSR class/structure verification tests
3. `preview/src/components/{component}/component.rs` — switch import to `dioxus_components`
4. `playwright/{component}.spec.ts` — behavior + accessibility + class tests

---

## Verification (per component)

```bash
cargo test -p dioxus-components --test {component}
cargo build -p preview
cd playwright && npx playwright test {component}.spec.ts --project=chromium
```
