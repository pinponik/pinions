use pinions::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
struct Counter {
    count: isize,
}

impl App<0, 0> for Counter {
    fn new(&mut self, ctx: &mut Ctx<0, 0>) {
        // ctx.add_window(Window::new(
        //     "Counter",
        //     0,
        //     "Counter".to_pstr(),
        //     (200, 100),
        //     (0, 0),
        // ));
    }

    fn view(&mut self, ctx: &mut Ctx<0, 0>) {
        let win = ctx.window(0);
        //win.add()
    }
}

fn main() {}
