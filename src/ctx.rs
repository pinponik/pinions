use crate::*;

pub struct Ctx<const T: usize, const W: usize> {
    drawing: bool,
    windows: Vect<Window<T, W>, MAX>,
    pub close: bool,
    pub should_close: bool,
    pub flow: Flow,
}

impl<const T: usize, const W: usize> Ctx<T, W> {
    pub fn new() -> Self {
        Self {
            drawing: true,
            windows: Vect::new(),
            close: false,
            should_close: false,
            flow: Flow::default(),
        }
    }

    /// Starts a new frame; call this at the beginning of each frame.
    pub fn begin_frame(&mut self) {
        self.drawing = true;
        // Optionally reset any per-frame state here.
    }

    /// Ends the current frame; call this after submitting all draw calls.
    pub fn end_frame(&mut self) {
        self.drawing = false;
    }

    /// Returns a mutable reference to a window by index.
    pub fn window(&mut self, win: usize) -> &mut Window<T, W> {
        &mut self.windows[win]
    }

    pub fn close(&mut self) {
        self.close = true;
    }

    pub fn add_window(&mut self, window: Window<T, W>) {
        self.windows.push(window);
    }
}
