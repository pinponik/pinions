use crate::*;

use vello;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct Application<A: App<T, W>, const T: usize, const W: usize> {
    window: Option<Window>,
    app: A,
    ctx: Ctx<T, W>,
}

impl<A: App<T, W>, const T: usize, const W: usize> ApplicationHandler for Application<A, T, W> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.ctx.should_close = true;
            }
            WindowEvent::RedrawRequested => {}
            _ => {}
        }
    }
}

pub fn run<const T: usize, const W: usize, A: App<T, W>>(app: &mut A, flow: Flow) {
    let event_loop = EventLoop::new();
}
