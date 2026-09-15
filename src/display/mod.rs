use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use spin::Mutex;
use static_cell::StaticCell;

use crate::display::colors::Color;

pub mod colors;

pub struct DisplayWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    cursor_x: usize,
    cursor_y: usize,
    default_color: Color,
}

pub static DISPLAY: Mutex<Option<DisplayWriter>> = Mutex::new(None);
pub static FRAME_BUFFER: StaticCell<&mut [u8]> = StaticCell::new();

impl DisplayWriter {
    pub fn new(_frame: &mut FrameBuffer) -> Self {
        let info = (&mut *_frame).info();
        let local_slice: &mut [u8] = (&mut *_frame).buffer_mut();
        let static_slice: &'static mut [u8] =
            unsafe { core::mem::transmute::<&mut [u8], &'static mut [u8]>(local_slice) };
        let buf: &'static mut &'static mut [u8] = { FRAME_BUFFER.init(static_slice) };
        let mut writer = Self {
            framebuffer: buf,
            info,
            cursor_x: 0,
            cursor_y: 0,
            default_color: Color::new(0xff, 0xff, 0xff),
        };
        writer.clear(Color::new(0x00, 0x00, 0x00));
        writer
    }

    pub fn _change_color(&mut self, color: Color) {
        self.default_color = color;
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Option<&Color>) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }
        self.cursor_x = x + 1 % self.info.width;
        self.cursor_y = y + 1 % self.info.height;
        let selected_color: &Color = {
            if color.is_some() {
                color.unwrap()
            } else {
                &self.default_color
            }
        };
        let pixel_offset = (y * self.info.stride + x) * self.info.bytes_per_pixel;
        let (r, g, b) = selected_color.ret();
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.framebuffer[pixel_offset] = r;
                self.framebuffer[pixel_offset + 1] = g;
                self.framebuffer[pixel_offset + 2] = b;
            }
            PixelFormat::Bgr => {
                self.framebuffer[pixel_offset] = b;
                self.framebuffer[pixel_offset + 1] = g;
                self.framebuffer[pixel_offset + 2] = r;
            }
            PixelFormat::U8 => {
                self.framebuffer[pixel_offset] = r / 3 + g / 3 + b / 3;
            }
            PixelFormat::Unknown {
                red_position,
                green_position,
                blue_position,
            } => {
                self.framebuffer[pixel_offset + red_position as usize] = r;
                self.framebuffer[pixel_offset + green_position as usize] = g;
                self.framebuffer[pixel_offset + blue_position as usize] = b;
            }
            _ => {}
        }
    }
    pub fn clear(&mut self, color: Color) {
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                self.write_pixel(x, y, Some(&color));
            }
        }
    }
}

#[macro_export]
macro_rules! pen_init {
    ($framebuffer:expr) => {
        $crate::display::init($framebuffer)
    };
}

#[macro_export]
macro_rules! draw {
    // ($x:expr, $y:expr $(, $color:expr)?) => {
    //     draw!($x, $y, $($color)?)
    // };
    ($x:expr, $y:expr, $color:expr) => {
        $crate::display::draw($x, $y, $color);
    };
    ($x:expr, $y:expr) => {
        $crate::display::draw($x, $y, None);
    };
}

#[macro_export]
macro_rules! pen_color {
    ($color:expr) => {
        $crate::display::_change_color($color)
    };
}

#[doc(hidden)]
pub fn _change_color(color: Color) {
    DISPLAY.lock();
    if let Some(writer) = DISPLAY.lock().as_mut() {
        writer._change_color(color);
    }
}
#[doc(hidden)]
pub fn draw(x: usize, y: usize, color: Option<&Color>) {
    DISPLAY.lock();
    if let Some(writer) = DISPLAY.lock().as_mut() {
        writer.write_pixel(x, y, color);
    }
}

#[doc(hidden)]
pub fn init(mut framebuffer: FrameBuffer) {
    let display: DisplayWriter = DisplayWriter::new(&mut framebuffer);
    *DISPLAY.lock() = Some(display);
}
