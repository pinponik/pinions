use crate::*;

trait App {
    fn view(&mut self, ctx: &mut Ctx) -> ();
}
