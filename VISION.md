```rust
use pinions::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
type Counter = i32;

impl App for Counter {
    fn view(&mut self, ctx: &mut Context) {
        ctx.add(LABEL.label(format!("Count: {}", self)));
        ctx.add(BUTTON.label(|_| "Increment").on_click(|&mut ctx| {
            *self += 1;
            ctx.request_redraw();
        }));
    }
}
fn main() {
    let mut app = Counter::default();
    let _ = pinions::run(&mut app, "Counter Example", 320, 240);
}
```
