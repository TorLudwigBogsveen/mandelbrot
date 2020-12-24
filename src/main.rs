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
use engine_gui::{comps::{Slider, TextBox}, gui::{GUI, TextAlign}};
use engine_renderer::{color::*, font::Font, graphics::Graphics, renderer, shader::Shader};

fn main() {
    let mut win = engine_core::window::Window::new(600, 400, "Graphics").unwrap();
    win.make_current();

    renderer::init_gl(&mut win);

    let mut xoff: f64 = -2.5;
    let mut yoff: f64 = -1.0;
    let mut width: f64 = 3.5;
    let mut height: f64 = 2.0;
    let mut zoom: f64 = 1.0;

    let mut gfx = Graphics::new(&mut win);
    gfx.set_shape_shader(Shader::from_file("res/mandelbrot.glsl"));

    let mut gui = GUI::new(&mut win);

    let mut input = Input::new(&mut win);

    //gfx.set_font(Font::new("res/UbuntuMono-Regular.ttf", 60));
    gui.graphics.set_font(Font::new("res/UbuntuMono-Regular.ttf", 80));
    gui.style.text_align = TextAlign::Center;

    let mut fps = 0;
    let mut fps_count = 0;
    let mut time = std::time::SystemTime::now();

    while !win.should_close() {
        fps_count += 1;

        if time.elapsed().unwrap() >= std::time::Duration::from_secs(1) {
            time = std::time::SystemTime::now();
            fps = fps_count;
            fps_count = 0;
        }

        if input.key_down(Key::R) {
            gfx.set_shape_shader(Shader::from_file("res/mandelbrot.glsl"));
        }

        gfx.clear(WHITE);

        let shader = gfx.shape_shader();
        shader.bind();
        shader.upload_from_name_1d("u_xoff", xoff);
        shader.upload_from_name_1d("u_yoff", yoff);
        shader.upload_from_name_1d("u_width", width / zoom);
        shader.upload_from_name_1d("u_height", height / zoom);
        shader.upload_from_name_1f("u_framewidth", win.get_width() as f32);
        shader.upload_from_name_1f("u_frameheight", win.get_height() as f32);

        //xoff += xoff.abs() * 0.01;
        //yoff += yoff.abs() * 0.01;
        //zoom += 0.001;

        gfx.fill_rect(-1.0, -1.0, 2.0, 2.0);

        //gfx.set_color(RED);
        //gfx.draw_string("fgh", 0.0, 0.0);

        gfx.update();
        gfx.flush();


        gui.graphics.set_translation(0.0, 0.0);
        gui.graphics.set_scale(1.0, 1.0);
        gui.graphics.draw_string(&fps.to_string(), -1.0, 1.0 - (gui.graphics.font().height() as f32 / win.get_height() as f32)  / 1.5);

        //gfx.draw_string("fgh", 0.0, 0.0);

        gui.graphics.set_translation(-1.0, -1.0);
        gui.graphics.set_scale(2.0 / gfx.frame_width() as f32, 2.0 / gfx.frame_height() as f32);
        

        if input.mouse_scroll_y() != 0.0 {
            //println!("{}", input.mouse_x());
            let x = input.mouse_x() as f64 / 2.0 + 0.5;
            let y = -input.mouse_y() as f64 / 2.0 + 0.5;

            //println!("{}", x * (width / zoom) + xoff);
            //println!("{}", y * (height / zoom) + yoff);

            //println!("zoom: {}", zoom);

            let w1 = width / (zoom);
            let w2 = width / (zoom * 1.05f64.powf(input.mouse_scroll_y() as f64));

            let h1 = height / (zoom);
            let h2 = height / (zoom * 1.05f64.powf(input.mouse_scroll_y() as f64));

            xoff = x * (w1 - w2) + xoff;
            yoff = y * (h1 - h2) + yoff;

            //(input.mouse_x() / 2.0) * (width / zoom) = (input.mouse_x() / 2.0) * (width / zoom * 1.05f32.powf(input.mouse_scroll_y())) + c

            //yoff = lerp(yoff, y, 0.1);
            zoom *= 1.05f64.powf(input.mouse_scroll_y() as f64);
        }
        
        //xoff = x_slider.val * 10.0 - 5.0;
        //yoff = y_slider.val * 10.0 - 5.0;
        //zoom = 1.0 / zoom_slider.val;
        

        gui.update();
        
        win.poll_events();
        win.swap_buffers();

        input.update();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

fn key_check(text_box: &mut TextBox) {
    for k in &text_box.keys  {
            
        if *k == Key::Backspace {
            text_box.text.pop();
        }

        if *k == Key::Slash { //minus
            text_box.text.push('-');
        }

        println!("{}", *k as u16);
        
        let c = *k as u8 as char;

        if (c >= '0' && c <= '9') || c == '.' {
            text_box.text.push(c);
        }

        println!("d{}", c);
    }
    text_box.keys.clear();
}