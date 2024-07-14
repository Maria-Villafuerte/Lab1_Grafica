use crate::framebuffer::Framebuffer;
use std::io::{self, Write};

pub fn save_framebuffer_as_bmp(fb: &Framebuffer, filename: &str) -> io::Result<()> {
    let mut file = std::fs::File::create(filename)?;

    // BMP file header
    let file_header_size = 14;
    let info_header_size = 40;
    let pixel_data_offset = file_header_size + info_header_size;
    let row_size = (3 * fb.width + 3) & !3;
    let pixel_data_size = row_size * fb.height;
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
        (fb.width & 0xFF) as u8,
        ((fb.width >> 8) & 0xFF) as u8,
        ((fb.width >> 16) & 0xFF) as u8,
        ((fb.width >> 24) & 0xFF) as u8,
        (fb.height & 0xFF) as u8,
        ((fb.height >> 8) & 0xFF) as u8,
        ((fb.height >> 16) & 0xFF) as u8,
        ((fb.height >> 24) & 0xFF) as u8,
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
    for y in (0..fb.height).rev() {
        for x in 0..fb.width {
            let index = y * fb.width + x;
            let pixel = fb.buffer[index];
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            file.write_all(&[b, g, r])?;
        }
        // Padding for 4-byte alignment
        for _ in 0..(row_size - 3 * fb.width) {
            file.write_all(&[0x00])?;
        }
    }

    Ok(())
}
