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

  fn draw_beehive(framebuffer: &mut FrameBuffer, x: isize, y: isize) {
    let positions = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
    for &(dx, dy) in &positions {
        framebuffer.point(x + dx, y + dy);
    }
  }

  fn draw_blinker(framebuffer: &mut FrameBuffer, x: isize, y: isize) {
    let positions = [(0, 0), (1, 0), (2, 0)];
    for &(dx, dy) in &positions {
        framebuffer.point(x + dx, y + dy);
    }
  }

  fn draw_loaf(framebuffer: &mut FrameBuffer, x: isize, y: isize) {
    let positions = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];
    for &(dx, dy) in &positions {
        framebuffer.point(x + dx, y + dy);
    }
  }

  fn draw_boat(framebuffer: &mut FrameBuffer, x: isize, y: isize) {
    let positions = [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];
    for &(dx, dy) in &positions {
        framebuffer.point(x + dx, y + dy);
    }
  }

  fn draw_tub(framebuffer: &mut FrameBuffer, x: isize, y: isize) {
    let positions = [(1, 0), (0, 1), (2, 1), (1, 2)];
    for &(dx, dy) in &positions {
        framebuffer.point(x + dx, y + dy);
    }
  }

  fn draw_pulsar(framebuffer: &mut FrameBuffer, x: isize, y: isize) {
    let positions = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
  
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12)
    ];
    for &(dx, dy) in &positions {
        framebuffer.point(x + dx, y + dy);
    }
  }

  fn draw_penta_decathlon(framebuffer: &mut FrameBuffer, x: isize, y: isize) {
    let positions = [
        (1, 0), (2, 0),
        (0, 1), (3, 1),
        (0, 2), (3, 2),
        (1, 3), (2, 3),
        (1, 4), (2, 4),
        (1, 5), (2, 5),
        (0, 6), (3, 6),
        (0, 7), (3, 7),
        (1, 8), (2, 8)
    ];
    for &(dx, dy) in &positions {
        framebuffer.point(x + dx, y + dy);
    }
  }

  fn draw_hwss(framebuffer: &mut FrameBuffer, x: isize, y: isize) {
    let positions = [
        (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
        (0, 1), (5, 1),
        (5, 2),
        (0, 3), (4, 3)
    ];
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
