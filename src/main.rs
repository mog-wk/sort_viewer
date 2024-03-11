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
    let mut test_arr: [u32; 6] = [4, 3, 2, 5, 1, 6];
    //let mut test_arr: [u32; 4] = [4, 3, 100, 40];

    // preprocess input

    let limits = utils::array::get_limits(&test_arr);
    //let path = spath.gen_path(&test_arr, crate::sorts::insertion::sort_path);
    
    println!("{:?}", test_arr);

    // border configs
    let h_margin: i32 = 80;
    let v_margin: i32 = 40;
    let h_padding = (window_width - 2 * h_margin) / test_arr.len() as i32;
    let inner_padding = 32;
    let box_height: i32 = window_height - 2 * v_margin;
    let box_unit: i32 = box_height / limits.1 as i32;

    // draw canvas border
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.draw_line(
        Point::new(h_margin, v_margin),
        Point::new(window_width - h_margin - inner_padding, v_margin),
    );
    canvas.draw_line(
        Point::new(window_width - h_margin - inner_padding, v_margin),
        Point::new(window_width - h_margin - inner_padding, window_height - v_margin),
    );
    canvas.draw_line(
        Point::new(window_width - h_margin - inner_padding, window_height - v_margin),
        Point::new(h_margin, window_height - v_margin),
    );
    canvas.draw_line(
        Point::new(h_margin, window_height - v_margin),
        Point::new(h_margin, v_margin),
    );
    canvas.present();


    let mut event_pump = sdl_context.event_pump()?;
    let mut paused = false;
    let mut step = 0_u32;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } | Event::Quit {..}  => break 'run,
                Event::KeyDown { keycode: Some(Keycode::P), .. } => paused = !paused,
                Event::KeyDown { keycode: Some(Keycode::S), .. } => step += 1,
                _ => (),
            }
        }

        // time control
        if paused || step == 0 {
            continue;
        } else {
            step -= 1;
        }

        // sort step

        let changed_indexes = sorts::insertion::sort(&mut canvas, &mut test_arr);

        println!("{:?}", test_arr);

        // render bars
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(127, 255, 127));
        for i in 0..test_arr.len() {
            let e = test_arr[i];
            let i = i as i32;

            let x: i32 = h_margin + i * h_padding as i32;
            let y: i32 = box_height + v_margin - (e as i32 * box_unit);
            let w: u32 = (h_padding - inner_padding) as u32;
            let h: u32 = (window_height - v_margin - y) as u32;

            //println!("{}: {} {} {} {}", e, x, y, w, h);
            canvas.fill_rect(Rect::new(x, y, w, h));
        }
        canvas.fill_rect(Rect::new(12 ,12, 36, 36));
        canvas.present();

        sleep(Duration::from_millis(60));
    }
    Ok(())
}
