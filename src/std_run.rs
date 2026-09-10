use crate::*;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct Application {
    
}

pub fn run<const T: usize, const W: usize, A: App<T, W>>(app: &mut A, flow: Flow) {
    let event_loop = EventLoop::new();
}
