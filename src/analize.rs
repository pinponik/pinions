use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Instant;

use vello::{
    kurbo::Stroke, peniko::Fill, AaConfig, AaSupport, Affine, Circle, Color, RenderParams,
    Renderer, RendererOptions, Scene,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App { state: None };
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct App {
    state: Option<State>,
}

struct State {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    renderer: Renderer,
    scene: Scene,
    start: Instant,
}

impl ApplicationHandler for App {
    // Na desktopie wywoływane raz, przy starcie aplikacji.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Vello + winit"))
                .unwrap(),
        );
        match pollster::block_on(State::new(window)) {
            Ok(state) => {
                state.window.request_redraw();
                self.state = Some(state);
            }
            Err(e) => {
                eprintln!("Inicjalizacja nie powiodła się: {e}");
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = self.state.as_mut() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    state.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                state.draw();
                // Ciągła pętla renderowania (dla animacji).
                // W aplikacji sterowanej zdarzeniami usuń tę linię i wywołuj
                // request_redraw() tylko wtedy, gdy scena faktycznie się zmieni.
                state.window.request_redraw();
            }
            _ => {}
        }
    }
}

impl State {
    async fn new(window: Arc<Window>) -> Result<Self, Box<dyn std::error::Error>> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        // Arc<Window> daje nam Surface<'static> — surface może mieszkać w strukturze State.
        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await?;

        let surface_config = wgpu::SurfaceConfiguration {
            // STORAGE_BINDING i COPY_DST: vello renderuje przez compute shader,
            // a render_to_surface robi wewnętrzny blit na teksturę powierzchni.
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            format: surface
                .get_capabilities(&adapter)
                .formats
                .first()
                .copied()
                .unwrap(),
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        // Jednorazowa, kosztowna inicjalizacja — trzymaj Renderer przez całe
        // życie aplikacji, nie twórz go w każdej klatce.
        let renderer = Renderer::new(
            &device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: AaSupport::all(),
                num_init_threads: NonZeroUsize::new(1),
            },
        )?;

        Ok(Self {
            window,
            device,
            queue,
            surface,
            surface_config,
            renderer,
            scene: Scene::new(),
            start: Instant::now(),
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    fn draw(&mut self) {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                // Utracona/przeniesiona powierzchnia: przekonfiguruj i pomiń klatkę.
                let (w, h) = (self.surface_config.width, self.surface_config.height);
                self.resize(w, h);
                return;
            }
        };

        let t = self.start.elapsed().as_secs_f32();

        // Scene budujemy od nowa w każdej klatce — to tani wzorzec w Vello.
        self.scene.reset();
        let cx = 320.0 + 180.0 * (t * 2.0).cos();
        let cy = 240.0 + 120.0 * (t * 2.0).sin();
        let ball = Circle::new((cx, cy), 60.0);

        self.scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgba8(242, 140, 168, 255),
            None,
            &ball,
        );
        self.scene.stroke(
            &Stroke::new(4.0),
            Affine::IDENTITY,
            Color::from_rgba8(137, 220, 235, 255),
            None,
            &ball,
        );

        if let Err(e) = self.renderer.render_to_surface(
            &self.device,
            &self.queue,
            &self.scene,
            &frame,
            &RenderParams {
                base_color: Color::from_rgba8(30, 30, 46, 255),
                width: self.surface_config.width,
                height: self.surface_config.height,
                antialiasing_method: AaConfig::Msaa16,
            },
        ) {
            eprintln!("Błąd renderowania: {e}");
        }

        frame.present();
    }
}
