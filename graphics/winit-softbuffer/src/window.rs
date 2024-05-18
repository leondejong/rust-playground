use std::cell::RefCell;
use std::num::NonZeroU32;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use softbuffer::Surface;

use crate::state::State;

pub const RESIZE: bool = true;
pub const WIDTH: f64 = 576.0;
pub const HEIGHT: f64 = 512.0;
pub const TITLE: &str = "Winit Softbuffer";

#[derive(Default)]
struct App {
    state: State,
    window: Option<Rc<Window>>,
    surface: Option<Rc<RefCell<Surface<Rc<Window>, Rc<Window>>>>>,
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
                        .with_resizable(RESIZE)
                        .with_inner_size(Size::Logical(LogicalSize::new(WIDTH, HEIGHT))),
                )
                .unwrap(),
        );

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        self.window = Some(window);
        self.surface = Some(Rc::new(RefCell::new(surface)));
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.clone().unwrap();
                let surface = self.surface.clone().unwrap();
                let mut surface = surface.borrow_mut();

                let scale = window.scale_factor();

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();

                self.state.render(&mut buffer, scale, width, height);

                buffer.present().unwrap();

                // self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
