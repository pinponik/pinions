use crate::*;

pub struct Ctx<const T: usize, const W: usize> {
    drawing: bool,
    windows: Vect<Window<T, W>, MAX>,
}

impl<const T: usize, const W: usize> Ctx<T, W> {
    //fn add(&mut self, win: usize, widget: /* type */) {}
    pub fn window(&mut self, win: usize) -> &mut Window<T, W> {
        &mut self.windows[win]
    }
}
