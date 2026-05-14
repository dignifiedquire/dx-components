### Announce

Renders children inline and mirrors them into a visually-hidden ARIA live region at `document.body` for screen readers.

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `type` | `AnnounceType` | `Polite` | Announcement urgency. `Polite` waits for the current screen-reader utterance to finish; `Assertive` interrupts; `Off` disables announcement (children still render visually). |
| `role` | `Option<RegionRole>` | derived from `type` | ARIA role for the underlying live region (`Status`, `Alert`, `Log`, `Marquee`, `Timer`). When `None`, the role is picked automatically based on `type`. |
| `aria_atomic` | `Option<bool>` | `false` | When `true`, the entire region is re-announced on any change. When `false`, only the changed nodes are. |
| `aria_relevant` | `Option<String>` | — | Which types of changes are announced (e.g. `"additions text"`). See the ARIA `aria-relevant` attribute. |
| `region_identifier` | `Option<String>` | — | Optional unique key. Without it, all `Announce` instances share one polite + one assertive region. Pass an id to create a dedicated region for that identifier — useful when concurrent announcers would otherwise overwrite each other. |
| `class` | `Option<String>` | — | Forwarded to the inline element. |
| `attributes` | `Vec<Attribute>` | — | Spread onto the inline element. |
| `children` | `Element` | — | Content to render inline and mirror into the live region. |

### AnnounceType

```rust
pub enum AnnounceType {
    Polite,    // Default. Waits for an idle moment in screen-reader output.
    Assertive, // Interrupts the screen reader immediately.
    Off,       // Children render but are not announced.
}
```

### RegionRole

```rust
pub enum RegionRole {
    Status,  // Default for `Polite`.
    Alert,   // Default for `Assertive`.
    Log,     // For chat logs and similar streaming content.
    Marquee, // Continuously updating, less important content.
    Timer,   // Time-related announcements.
}
```
