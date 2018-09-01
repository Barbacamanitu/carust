
extern crate gfx_window_glutin;
extern crate glutin;
extern crate gfx;
extern crate gfx_device_gl;

use gfx::traits::FactoryExt;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlContext, GlRequest,WindowEvent,Event,ControlFlow};
use self::gfx_device_gl::{Device,Factory};
use glutin::Api::OpenGl;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

mod renderer_2d;

pub struct RenderWindow {
    pub window: glutin::GlWindow,
    pub device: Device,
    pub factory: Factory,
    pub color_view: gfx::handle::RenderTargetView<gfx_device_gl::Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Srgb)>,
    pub depth_view: gfx::handle::DepthStencilView<gfx_device_gl::Resources, (gfx::format::D24_S8, gfx::format::Unorm)>,
    pub events_loop: glutin::EventsLoop,
    is_running: bool,
}

struct Dimensions<T> {
    width: T,
    height: T,
}

impl RenderWindow {

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn handle_events(&mut self) {
        let mut running = self.is_running;
        self.events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested |
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => running = false,
                    _ => {}
                }
            }
        });
        self.is_running = running;
    }


    pub fn new() -> RenderWindow {
        let events_loop = glutin::EventsLoop::new();
        let windowbuilder = glutin::WindowBuilder::new()
        .with_title("Render Window".to_string())
        .with_dimensions(glutin::dpi::LogicalSize::new(512.0, 512.0));
        let contextbuilder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl,(3,2)))
        .with_vsync(true);
        let (window, device, factory, color_view, depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(windowbuilder, contextbuilder, &events_loop);
        RenderWindow {
            window: window,
            device: device,
            factory: factory,
            color_view: color_view,
            depth_view: depth_view,
            events_loop: events_loop,
            is_running: true
        }
    }

}