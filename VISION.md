```rust
use pinions::prelude::*;

#[derive(Default)]
type Counter = i32;

impl App for Counter {
    fn view(&mut self) {
        Window::new("")
    }
}
