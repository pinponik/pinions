use crate::*;

pub struct Window<const T: usize> {
    title: Str<T>,
    size: (usize, usize),
    location: (usize, usize),
    close: bool,
    pub should_close: bool,
}

pub const WINDOW: Window<0> = Window::<0> {
    title: Str::<0>::new(),
    size: (0, 0),
    location: (0, 0),
    close: false,
    should_close: false,
};

impl<const T: usize> Window<T> {
    pub fn new(title: Str<T>, size: (usize, usize), location: (usize, usize)) -> Self {
        Window {
            title,
            size,
            location,
            close: false,
            should_close: false,
        }
    }

    pub fn close(&mut self) {
        self.close = true;
    }

    pub fn unclose(&mut self) {
        self.close = false;
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn title(&mut self, title: Str<T>) {
        self.title = title;
    }

    pub fn get_title(&self) -> &Str<T> {
        &self.title
    }

    pub fn size(&mut self, size: (usize, usize)) {
        self.size = size;
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size
    }

    pub fn location(&mut self, location: (usize, usize)) {
        self.location = location;
    }

    pub fn get_location(&self) -> (usize, usize) {
        self.location
    }
}
