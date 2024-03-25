#![allow(unused)]

mod error;
mod sorts;
mod utils;
mod rendering;

pub type Result<T> = core::result::Result<T, error::Error>;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{ Rect, Point };

use std::time::Duration;
use std::thread::sleep;

use rand::prelude::SliceRandom;

use rendering::sorter::SorterBox;

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


    let mut rng = rand::thread_rng();

    // dev test let mut test_arr: [u32; 6] = [4, 3, 2, 5, 1, 6]; let mut test_arr: [u32; 4] = [4,
    // 3, 100, 40];
    let mut test_arr: [u32; 71] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
        14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
        37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
        60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
        //72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
        //83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94,
        //95, 96, 97, 98, 99, 100,
    ];
    test_arr.shuffle(&mut rng);

    // preprocess input

    let limits = utils::array::get_limits(&test_arr);
    //let path = spath.gen_path(&test_arr, crate::sorts::insertion::sort_path);
    
    println!("{:?}", test_arr);

    // border configs
    let h_margin: i32 = 4;
    let v_margin: i32 = 40;
    let inner_padding = 2;
    let h_padding = (window_width - 2 * h_margin) / test_arr.len() as i32 ;
    let box_height: i32 = window_height - 2 * v_margin;

    let sort_box = SorterBox::new(
        (80, 40),
        (0, box_height),
        h_padding,
        inner_padding,
    );
    let box_unit: i32 = box_height / limits.1 as i32;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();


    let mut event_pump = sdl_context.event_pump()?;
    let mut paused = true;
    let mut step = 1_u32;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } | Event::Quit {..}  => break 'run,
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    paused = !paused;
                    println!("{} {}", paused, step);
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => step += 1,
                _ => (),
            }
        }

        // time control
        if paused {
            continue;
        } else {
            step = step.checked_sub(1).unwrap_or(0);
        }

        // sort step
        let changed_indexes = sorts::insertion::sort(&mut canvas, &mut test_arr);

        println!("{:?}", test_arr);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        sort_box.render_border(&mut canvas, (window_width, window_height));

        // render bars
        sort_box.render(&mut canvas, &mut test_arr, box_unit, window_height);

        canvas.fill_rect(Rect::new(12 ,12, 36, 36));
        canvas.present();

        if !paused {
            sleep(Duration::from_millis(1));
        }

        sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }
    Ok(())
}
