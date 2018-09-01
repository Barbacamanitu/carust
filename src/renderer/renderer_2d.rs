extern crate gfx_window_glutin;
extern crate glutin;
extern crate gfx;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;

use renderer;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;


const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
// Put this code above your main function
gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4];4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub struct Renderer2D {
    window: renderer::RenderWindow
}

impl Renderer2D {
    pub fn render(&self, data: &Vec<u8>) {
        println!("Rendering!!!");
    }

    pub fn new() -> Renderer2D {
        Renderer2D {
            window: renderer::RenderWindow::new()
        }
    }
}

