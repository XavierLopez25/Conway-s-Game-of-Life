use crate::colors::Color;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub background_color: Color,
    pub foreground_color: Color,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        let background_color = Color::from_hex(0x000000); // Default to black
        let foreground_color = Color::from_hex(0xFFFFFF); // Default to white
        let buffer = vec![background_color.to_hex(); width * height];
        FrameBuffer {
            width,
            height,
            buffer,
            background_color,
            foreground_color,
        }
    }

    pub fn clear(&mut self) {
        let color_hex = self.background_color.to_hex();
        self.buffer.fill(color_hex);
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }
        let index = y as usize * self.width + x as usize;
        self.buffer[index] = self.foreground_color.to_hex();
    }

    pub fn get_pixel(&self, x: isize, y: isize) -> Option<Color> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        let index = y as usize * self.width + x as usize;
        Some(Color::from_hex(self.buffer[index]))
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = Color::from_hex(color);
    }

    pub fn set_foreground_color(&mut self, color: u32) {
        self.foreground_color = Color::from_hex(color);
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        let index = y * self.width + x;
        self.buffer[index] == self.foreground_color.to_hex()
    }

    pub fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];

        directions.iter().filter_map(|&(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                if self.is_alive(nx as usize, ny as usize) {
                    Some(1)
                } else {
                    None
                }
            } else {
                None
            }
        }).sum()
    }

    pub fn update_game_of_life(&mut self) {
        let mut new_buffer = self.buffer.clone();  // Clona el buffer actual

        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.is_alive(x, y);
                let neighbors = self.live_neighbor_count(x, y);
                let idx = y * self.width + x;
                new_buffer[idx] = match (alive, neighbors) {
                    (true, 2) | (true, 3) => self.foreground_color.to_hex(),
                    (false, 3) => self.foreground_color.to_hex(),
                    _ => self.background_color.to_hex(),
                };
            }
        }

        self.buffer = new_buffer;  // Actualiza el buffer
    }

}
