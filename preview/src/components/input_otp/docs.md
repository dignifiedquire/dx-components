Accessible one-time password component with copy paste functionality.

## Usage

```rust
use dioxus::prelude::*;
use dioxus_components::input_otp::*;

rsx! {
    InputOTP { max_length: 6,
        InputOTPGroup {
            InputOTPSlot { index: 0 }
            InputOTPSlot { index: 1 }
            InputOTPSlot { index: 2 }
        }
        InputOTPSeparator {}
        InputOTPGroup {
            InputOTPSlot { index: 3 }
            InputOTPSlot { index: 4 }
            InputOTPSlot { index: 5 }
        }
    }
};
```

## Props

### InputOTP

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `max_length` | `usize` | required | Number of input slots |
| `value` | `String` | `""` | Current OTP value |
| `on_change` | `Callback<String>` | - | Called when value changes |
| `on_complete` | `Callback<String>` | - | Called when all slots filled |
| `pattern` | `Option<String>` | `None` | Regex to constrain input characters |
| `disabled` | `bool` | `false` | Whether the input is disabled |

### InputOTPSlot

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `index` | `usize` | required | Slot position (0-based) |
