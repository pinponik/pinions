use crate::*;

pub trait App<const T: usize, const W: usize> {
    fn view(&mut self, ctx: &mut Ctx<T, W>) -> ();
    fn new(&mut self, ctx: &mut Ctx<T, W>) -> ();
}
