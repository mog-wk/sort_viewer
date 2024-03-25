
use sdl2::rect::{ Rect, Point };
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

// sorter box render
// sdl2 uses i32 for position and u32 for dimentons(width/height)
#[derive(Debug)]
pub struct SorterBox {
    margin: (i32, i32), // space to canvas border
    dimentions: (i32, i32), // dimentions
    h_padding: i32, // space between border and sort bars
    inner_padding: i32,// padding between sort bars 
}

impl SorterBox {
    pub fn new(
    margin: (i32, i32),
    dimentions: (i32, i32),
    h_padding: i32,
    inner_padding: i32, 
    ) -> Self {
        Self { margin, dimentions, h_padding, inner_padding, }
    }

    pub fn margin(&self) -> (i32, i32) {
        self.margin
    }

    pub fn h_margin(&self) -> i32 {
        self.margin.0
    }

    pub fn v_margin(&self) -> i32 {
        self.margin.1
    }

    pub fn h_dim(&self) -> i32 {
        self.dimentions.0
    }
    pub fn v_dim(&self) -> i32 {
        self.dimentions.1
    }

    pub fn render_border(&self, canvas: &mut Canvas<Window>, window_dim: (i32, i32)) {

        // draw canvas border
        let a = Point::new(self.margin.0, self.margin.1);
        let b = Point::new(window_dim.0 - self.margin.0 - self.inner_padding, self.margin.1);
        let c = Point::new(window_dim.0 - self.margin.0 - self.inner_padding, window_dim.1 - self.margin.1);
        let d = Point::new(self.margin.0, window_dim.1 - self.margin.1);


        canvas.set_draw_color(Color::RGB(255, 255, 255));

        canvas.draw_line(a, b);
        canvas.draw_line(b, c);
        canvas.draw_line(c, d);
        canvas.draw_line(d, a);
    }
    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        arr: &mut [u32],
        box_unit: i32,
        window_height: i32,
        target_index: (usize, usize),
    ) {
        let w: i32 = self.h_padding - self.inner_padding;
        // render bars
        for i in 0..arr.len() {
            if i == target_index.0 || i == target_index.1 {
                canvas.set_draw_color(Color::RGB(255, 127, 0));
            } else {
                canvas.set_draw_color(Color::RGB(127, 255, 127));
            }

            let e = arr[i];
            let i = i as i32;

            let x: i32 = self.margin.0 + i * w;
            let y: i32 = self.dimentions.1 + self.margin.1 - (e as i32 * box_unit);
            let h: i32 = (window_height - self.margin.1 - y);

            println!("{} {} {} {}", x, y, w, h);

            canvas.fill_rect(Rect::new(x, y, w as u32, h as u32));
            canvas.set_draw_color(Color::RGB(127, 0, 0));
            canvas.draw_line(
                Point::new(x+w-1, y),
                Point::new(x+w-1, y+h),
            );
            canvas.draw_line(
                Point::new(x+w-2, y),
                Point::new(x+w-2, y+h),
            );
        }
        // debug square
        canvas.fill_rect(Rect::new(12 ,12, 36, 36));
    }
}

//box_unit: i32,
