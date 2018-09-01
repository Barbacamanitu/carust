#[macro_use]
extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;

mod renderer;
use renderer::RenderWindow;

pub fn main() {
    let mut render_window = RenderWindow::new();
    while render_window.is_running() {
        render_window.handle_events();

    
    }

}