use std::cell::RefCell;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use pixels::{Pixels, SurfaceTexture};

use crate::state::State;

pub const WIDTH: f64 = 576.0;
pub const HEIGHT: f64 = 512.0;
pub const TITLE: &str = "Winit Pixels";

#[derive(Default)]
struct App {
    state: State,
    window: Option<Rc<Window>>,
    pixels: Option<Rc<RefCell<Pixels>>>,
}

pub fn run(state: State) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    // event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    app.state = state;
    event_loop.run_app(&mut app).unwrap();
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(TITLE)
                        .with_resizable(false)
                        .with_inner_size(Size::Logical(LogicalSize::new(WIDTH, HEIGHT))),
                )
                .unwrap(),
        );

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
        };

        self.window = Some(window.clone());
        self.pixels = Some(Rc::new(RefCell::new(pixels)));

        window.request_redraw();
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                let _ = self.window.take();
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.clone().unwrap();
                let pixels = self.pixels.clone().unwrap();
                let mut pixels = pixels.borrow_mut();

                let scale = window.scale_factor();

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                pixels.resize_surface(width, height).unwrap();

                let mut buffer = pixels.frame_mut();

                self.state.render(&mut buffer, scale, width, height);

                let _ = pixels.render();

                // window.request_redraw();
            }
            _ => (),
        }
    }
}
