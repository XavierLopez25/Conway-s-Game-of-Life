use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::Duration;
mod framebuffer;
mod colors;
use framebuffer::FrameBuffer;

fn render(framebuffer: &mut FrameBuffer) {
    framebuffer.update_game_of_life();
}

fn draw_block(framebuffer: &mut FrameBuffer, x: isize, y: isize) {
    let positions = [(0, 0), (1, 0), (0, 1), (1, 1)];
    for &(dx, dy) in &positions {
        framebuffer.point(x + dx, y + dy);
    }
  }
  
fn main() {
    let window_width = 1200;
    let window_height = 800;
    let framebuffer_width = 120;
    let framebuffer_height = 80;
    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_foreground_color(0xCDE8E5);
    framebuffer.set_background_color(0x4D869C);

    let mut window = Window::new(
        "Rust Graphics - Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();


    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        render(&mut framebuffer);

        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
              .unwrap();

        std::thread::sleep(frame_delay);
    }
}
