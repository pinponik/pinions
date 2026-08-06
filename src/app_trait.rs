use crate::*;

pub trait App {
    fn view(&mut self, ctx: &mut Ctx) -> ();
}
