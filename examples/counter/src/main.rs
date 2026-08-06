#![no_std]
#![no_main]
use pinions::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
struct Counter {
    count: isize,
}

impl App for Counter {
    fn view(&mut self, ctx: &mut Ctx) {
        //ctx.add(0, LABEL);
    }
}

fn main() {}
