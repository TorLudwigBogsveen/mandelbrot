/*
*   Copyright (c) 2020 Ludwig Bogsveen
*   All rights reserved.

*   Permission is hereby granted, free of charge, to any person obtaining a copy
*   of this software and associated documentation files (the "Software"), to deal
*   in the Software without restriction, including without limitation the rights
*   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
*   copies of the Software, and to permit persons to whom the Software is
*   furnished to do so, subject to the following conditions:

*   The above copyright notice and this permission notice shall be included in all
*   copies or substantial portions of the Software.

*   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
*   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
*   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
*   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
*   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
*   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
*   SOFTWARE.
*/

use engine_core::input::{Input, Key};
use engine_gui::{comps::TextBox, gui::GUI};
use engine_renderer::{color::*, graphics::Graphics, renderer, shader::Shader};

fn main() {
    let mut win = engine_core::window::Window::new(600, 400, "Graphics").unwrap();
    win.make_current();
    renderer::init_gl(&mut win);

    let mut xoff: f32 = -2.5;
    let mut yoff: f32 = -1.0;
    let width = 3.5;
    let height = 2.0;
    let mut zoom = 1.0;

    let mut gfx = Graphics::new(&mut win);
    gfx.set_shape_shader(Shader::from_file("res/mandelbrot.glsl"));

    let mut gui = GUI::new(&mut win);

    let mut input = Input::new(&mut win);
    
    let mut x_box = TextBox {
        x: 0.0,
        y: 0.0, 
        width: 128.0, 
        height: 64.0,
        text: String::from("0.0"),
        selected: false,
        keys: Vec::new(),
    };

    while !win.should_close() {
        gfx.clear(WHITE);

        let shader = gfx.shape_shader();
        shader.bind();
        shader.upload_from_name_1f("u_xoff", xoff);
        shader.upload_from_name_1f("u_yoff", yoff);
        shader.upload_from_name_1f("u_width", width / zoom);
        shader.upload_from_name_1f("u_height", height / zoom);
        shader.upload_from_name_1f("u_framewidth", win.get_width() as f32);
        shader.upload_from_name_1f("u_frameheight", win.get_height() as f32);

        //xoff += xoff.abs() * 0.01;
        //yoff += yoff.abs() * 0.01;
        //zoom += 0.001;

        gfx.fill_rect(-1.0, -1.0, 2.0, 2.0);

        gfx.update();
        gfx.flush();


        gui.graphics.set_translation(-1.0, -1.0);
        gui.graphics.set_scale(2.0 / gfx.frame_width() as f32, 2.0 / gfx.frame_height() as f32);

        gui.text_box(&mut x_box);

        for k in &x_box.keys  {
            let c = *k as u8 as char;

            if (c >= '0' && c <= '9') || c == '.' {
                
            } 
            
            println!("d{}", c);
        }

        gui.update();
        
        win.poll_events();
        win.swap_buffers();

        input.update();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}