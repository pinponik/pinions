use crate::*;

use vello;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct Application<A: App<T, W>, const T: usize, const W: usize> {
    windows: Vect<Option<Window>, W>,
    app: A,
    ctx: Ctx<T, W>,
}

impl<A: App<T, W>, const T: usize, const W: usize> ApplicationHandler for Application<A, T, W> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        for i in 0..W {
            self.windows[i] = Some(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.ctx.should_close = true;
            }
            WindowEvent::RedrawRequested => {
                for window in self.windows.iter_mut() {
                    window.as_ref().unwrap().request_redraw();
                }
            }
            _ => {}
        }
    }
}

pub fn run<const T: usize, const W: usize, A: App<T, W>>(app: A) {
    let mut appl = Application {
        windows: Vect::new(),
        app: app,
        ctx: Ctx::new(),
    };
    appl.app.run(&mut appl.ctx);
    let event_loop = EventLoop::new().unwrap();
    if let Flow::Wait = appl.ctx.flow {
        event_loop.set_control_flow(ControlFlow::Wait);
    } else {
        event_loop.set_control_flow(ControlFlow::Poll);
    }
    event_loop.run_app(&mut appl);
}
