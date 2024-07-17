use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::{BufWriter, Write};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24;

pub trait BmpWriter {
    fn save_as_bmp(&self, file_path: &str) -> std::io::Result<()>;
}

impl BmpWriter for Framebuffer {
    fn save_as_bmp(&self, file_path: &str) -> std::io::Result<()> {
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);

        self.write_bmp_header(&mut writer)?;
        self.write_pixel_data(&mut writer)?;

        Ok(())
    }
}

impl Framebuffer {
    fn write_bmp_header(&self, writer: &mut BufWriter<File>) -> std::io::Result<()> {
        let file_size = BMP_HEADER_SIZE + (self.width * self.height * 3) as usize;

        writer.write_all(&[
            b'B',
            b'M', // BMP Signature
            (file_size & 0xFF) as u8,
            ((file_size >> 8) & 0xFF) as u8,
            ((file_size >> 16) & 0xFF) as u8,
            ((file_size >> 24) & 0xFF) as u8,
            0,
            0,
            0,
            0, // Reserved
            BMP_PIXEL_OFFSET as u8,
            (BMP_PIXEL_OFFSET >> 8) as u8,
            (BMP_PIXEL_OFFSET >> 16) as u8,
            (BMP_PIXEL_OFFSET >> 24) as u8, // Data offset
            40,
            0,
            0,
            0, // Header size
            (self.width as u32 & 0xFF) as u8,
            ((self.width as u32 >> 8) & 0xFF) as u8,
            ((self.width as u32 >> 16) & 0xFF) as u8,
            ((self.width as u32 >> 24) & 0xFF) as u8,
            (self.height as u32 & 0xFF) as u8,
            ((self.height as u32 >> 8) & 0xFF) as u8,
            ((self.height as u32 >> 16) & 0xFF) as u8,
            ((self.height as u32 >> 24) & 0xFF) as u8,
            1,
            0, // Number of color planes
            BMP_BITS_PER_PIXEL as u8,
            0, // Bits per pixel
            0,
            0,
            0,
            0, // Compression
            0,
            0,
            0,
            0, // Image size (no compression)
            0,
            0,
            0,
            0, // Horizontal resolution (pixels per meter)
            0,
            0,
            0,
            0, // Vertical resolution (pixels per meter)
            0,
            0,
            0,
            0, // Number of colors
            0,
            0,
            0,
            0, // Important colors
        ])?;
        Ok(())
    }

    fn write_pixel_data(&self, writer: &mut BufWriter<File>) -> std::io::Result<()> {
        let padding = (4 - (self.width * 3) % 4) % 4;

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = &self.buffer[y * self.width + x];
                writer.write_all(&[pixel.b, pixel.g, pixel.r])?;
            }
            for _ in 0..padding {
                writer.write_all(&[0])?;
            }
        }
        Ok(())
    }
}
