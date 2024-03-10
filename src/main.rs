#![allow(unused)]

mod error;
mod sorts;

pub type Result<T> = core::result::Result<T, error::Error>;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;
use std::thread::sleep;


fn main() -> Result<()> {
    println!("Hello, world!");
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("sort viwer", 800, 600)
        .position(0, 0)
        .opengl()
        .build()?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,

                _ => println!("{:?}", event),
            }
        }
    }

    sleep(Duration::from_millis(60));
    Ok(())
}
