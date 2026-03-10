A collection of links for navigating websites.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::navigation_menu::*;

rsx! {
    NavigationMenu {
        NavigationMenuList {
            NavigationMenuItem {
                NavigationMenuTrigger { "Getting Started" }
                NavigationMenuContent {
                    NavigationMenuLink { href: "/docs", "Documentation" }
                }
            }
        }
    }
};
```

## Props

### NavigationMenu

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | `None` | Controlled active item |
| `default_value` | `String` | `""` | Default active item |
| `on_value_change` | `Callback<String>` | - | Called when active item changes |
| `orientation` | `NavigationMenuOrientation` | `Horizontal` | Layout direction |
| `viewport` | `bool` | `true` | Whether to render a viewport container |

### NavigationMenuLink

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `href` | `Option<String>` | `None` | Link destination |
| `active` | `bool` | `false` | Whether the link is currently active |
