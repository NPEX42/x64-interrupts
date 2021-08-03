# x86 UART Serial Logger

----

## Examples

```rust
use x86_interrupts as interrupts;

pub fn main() {
    interrupts::initialize().expect("Failed To Init Interrupts");
    interrupts::set_irq_handler(1, on_keypress); // IRQ #1: PS2 Keyboard.
}

pub fn on_keypress() {
    println!("Key Pressed!");
}
```
