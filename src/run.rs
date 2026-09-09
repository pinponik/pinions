use crate::*;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub fn run<const T: usize, const W: usize, A: App<T, W>>(app: &mut A, flow: Flow) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
}
