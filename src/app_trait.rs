use crate::*;

pub trait App<const T: usize, const W: usize> {
    fn view(&mut self, ctx: &mut Ctx<T, W>) -> ();
    fn run(&mut self, ctx: &mut Ctx<T, W>) -> ();
}
