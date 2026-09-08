use crate::*;

pub struct Window<const T: usize, const W: usize> {
    title: Str<T>,
    size: (usize, usize),
    location: (usize, usize),
    shapes: Vect<Shape, W>,
}

pub const WINDOW: Window<0, 0> = Window::<0, 0> {
    title: Str::<MAX>::new(),
    size: (0, 0),
    location: (0, 0),
    shapes: Vect::<Shape, 0>::new(),
};

impl<const T: usize, const W: usize> Window<T, W> {
    pub fn new(title: Str<T>, size: (usize, usize), location: (usize, usize)) -> Self {
        Window {
            title,
            size,
            location,
            shapes: Vect::<Shape, W>::new(),
        }
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

    pub fn add(&mut self, widget: Shape) {
        self.shapes.push(widget);
    }
}
