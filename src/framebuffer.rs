use crate::color::Color;
use std::io::{self, Write};

pub struct Framebuffer {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::new(0, 0, 0).to_hex_u32(); // Negro por defecto
        let current_color = Color::new(255, 255, 255).to_hex_u32(); // Blanco por defecto
        let buffer = vec![background_color; width * height];
        Framebuffer {
            buffer,
            width,
            height,
            background_color,
            current_color,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.current_color;
        }
    }

    pub fn get_pixel(&self, x: isize, y: isize) -> Option<u32> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = (y as usize) * self.width + (x as usize);
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color.to_hex_u32();
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color.to_hex_u32();
    }

    pub fn render_buffer(&self, filename: &str) -> io::Result<()> {
        let mut file = std::fs::File::create(filename)?;

        // BMP file header
        let file_header_size = 14;
        let info_header_size = 40;
        let pixel_data_offset = file_header_size + info_header_size;
        let row_size = (3 * self.width + 3) & !3;
        let pixel_data_size = row_size * self.height;
        let file_size = pixel_data_offset + pixel_data_size;

        file.write_all(&[
            0x42, 0x4D, // Signature "BM"
            (file_size & 0xFF) as u8,
            ((file_size >> 8) & 0xFF) as u8,
            ((file_size >> 16) & 0xFF) as u8,
            ((file_size >> 24) & 0xFF) as u8,
            0x00, 0x00, // Reserved
            0x00, 0x00, // Reserved
            (pixel_data_offset & 0xFF) as u8,
            ((pixel_data_offset >> 8) & 0xFF) as u8,
            ((pixel_data_offset >> 16) & 0xFF) as u8,
            ((pixel_data_offset >> 24) & 0xFF) as u8,
        ])?;

        // DIB header
        file.write_all(&[
            (info_header_size & 0xFF) as u8,
            ((info_header_size >> 8) & 0xFF) as u8,
            ((info_header_size >> 16) & 0xFF) as u8,
            ((info_header_size >> 24) & 0xFF) as u8,
            (self.width & 0xFF) as u8,
            ((self.width >> 8) & 0xFF) as u8,
            ((self.width >> 16) & 0xFF) as u8,
            ((self.width >> 24) & 0xFF) as u8,
            (self.height & 0xFF) as u8,
            ((self.height >> 8) & 0xFF) as u8,
            ((self.height >> 16) & 0xFF) as u8,
            ((self.height >> 24) & 0xFF) as u8,
            0x01, 0x00, // Planes
            0x18, 0x00, // Bits per pixel (24 bits)
            0x00, 0x00, 0x00, 0x00, // Compression (none)
            (pixel_data_size & 0xFF) as u8,
            ((pixel_data_size >> 8) & 0xFF) as u8,
            ((pixel_data_size >> 16) & 0xFF) as u8,
            ((pixel_data_size >> 24) & 0xFF) as u8,
            0x13, 0x0B, 0x00, 0x00, // Horizontal resolution (2835 pixels/meter)
            0x13, 0x0B, 0x00, 0x00, // Vertical resolution (2835 pixels/meter)
            0x00, 0x00, 0x00, 0x00, // Colors in color table (none)
            0x00, 0x00, 0x00, 0x00, // Important color count (all)
        ])?;

        // Pixel data
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let index = y * self.width + x;
                let pixel = self.buffer[index];
                let r = ((pixel >> 16) & 0xFF) as u8;
                let g = ((pixel >> 8) & 0xFF) as u8;
                let b = (pixel & 0xFF) as u8;
                file.write_all(&[b, g, r])?;
            }
            // Padding for 4-byte alignment
            for _ in 0..(row_size - 3 * self.width) {
                file.write_all(&[0x00])?;
            }
        }

        Ok(())
    }
    pub fn line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                self.point(x, y);
            }
            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}
