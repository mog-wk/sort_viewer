#![allow(unused)]

mod error;
mod sorts;
mod utils;

pub type Result<T> = core::result::Result<T, error::Error>;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{ Rect, Point };

use std::time::Duration;
use std::thread::sleep;


fn main() -> Result<()> {
    println!("Hello, world!");
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window_width = 800_i32;
    let window_height = 600_i32;

    let window = video_subsystem
        .window("sort viwer", window_width as u32, window_height as u32)
        .position(0, 0)
        .opengl()
        .build()?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // dev test
    //let mut test_arr: [u32; 6] = [4, 3, 2, 5, 1, 6];
    let mut test_arr: [u32; 4] = [4, 3, 100, 40];

    // preprocess input

    utils::array::get_limits(&test_arr);

    // border configs
    let h_margin: i32 = 80;
    let v_margin: i32 = 40;
    let h_padding = (window_width - 2*h_margin) / test_arr.len() as i32;
    let v_scale = (window_height - 2*v_margin) / 6;

    println!("{} {} {} {}", h_margin, v_margin, h_padding, v_scale);

    // draw canvas border
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.draw_line(
        Point::new(h_margin, v_margin),
        Point::new(window_width - h_margin, v_margin),
    );
    canvas.draw_line(
        Point::new(window_width - h_margin, v_margin),
        Point::new(window_width - h_margin, window_height - v_margin),
    );
    canvas.draw_line(
        Point::new(window_width - h_margin, window_height - v_margin),
        Point::new(h_margin, window_height - v_margin),
    );
    canvas.draw_line(
        Point::new(h_margin, window_height - v_margin),
        Point::new(h_margin, v_margin),
    );
    canvas.present();


    canvas.set_draw_color(Color::RGB(127, 255, 127));

    let mut event_pump = sdl_context.event_pump()?;
    let mut paused = false;
    let mut step = 0_u32;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                Event::KeyDown { keycode: Some(Keycode::P), .. } => paused = !paused,
                Event::KeyDown { keycode: Some(Keycode::S), .. } => step += 1,
                _ => (),
            }
        }

        if paused || step == 0 {
            continue;
        }
        if step > 0 {
            step -= 1;
        }
        for i in 0..test_arr.len() {
            let e = test_arr[i];
            let i = i as i32;

            let x = h_margin + i * h_padding as i32;
            let y = v_margin + (e) as i32;
            let w = h_padding as u32 - 32;
            let h = (window_height - v_margin * 2) as u32 - 12 - e;
            println!("{} {} {} {}", x, y, w, h);
            canvas.fill_rect(Rect::new(x, y, w, h));
        }
        canvas.fill_rect(Rect::new(12 ,12, 36, 36));
        canvas.present();

        sleep(Duration::from_millis(60));
    }
    Ok(())
}
