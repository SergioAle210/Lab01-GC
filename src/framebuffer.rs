use crate::color::Color;

pub struct Framebuffer {
    pub buffer: Vec<Color>,      // Vector de píxeles en formato RGBA
    pub width: usize,            // Ancho del framebuffer
    pub height: usize,           // Alto del framebuffer
    pub background_color: Color, // Color de fondo
    pub current_color: Color,    // Color actual (foreground)
}

impl Framebuffer {
    // Constructor para crear un nuevo framebuffer con ancho y alto dados
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![Color::new(0, 0, 0); width * height]; // Inicializar con color negro
        Self {
            buffer,
            width,
            height,
            background_color: Color::new(0, 0, 0), // Color negro como predeterminado
            current_color: Color::new(255, 255, 255), // Color blanco como predeterminado
        }
    }

    // Función para limpiar el framebuffer con el color de fondo
    pub fn clear(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = self.background_color.clone();
        }
    }

    // Función para colocar un punto en las coordenadas (x, y) con el color actual
    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.buffer[y as usize * self.width + x as usize] = self.current_color.clone();
        }
    }

    // Función para obtener el color de un punto en las coordenadas (x, y)
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            Some(self.buffer[y as usize * self.width + x as usize].clone())
        } else {
            None
        }
    }

    // Métodos para settear los colores de fondo y actual
    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.background_color = color.into();
    }

    pub fn set_current_color(&mut self, color: impl Into<Color>) {
        self.current_color = color.into();
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = y as usize * self.width + x as usize;
            self.buffer[index] = color;
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x0 = x0;
        let mut y0 = y0;

        loop {
            self.set_pixel(x0, y0, color.clone());

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

// Implementación del trait `Into<Color>` para que `u32` pueda convertirse en `Color`
impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as i32;
        let g = ((hex >> 8) & 0xFF) as i32;
        let b = (hex & 0xFF) as i32;
        Color::new(r, g, b)
    }
}

// Pruebas unitarias
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framebuffer_new() {
        let fb = Framebuffer::new(800, 600);
        assert_eq!(fb.width, 800);
        assert_eq!(fb.height, 600);
        assert_eq!(fb.background_color, Color::new(0, 0, 0));
        assert_eq!(fb.current_color, Color::new(255, 255, 255));
        assert_eq!(fb.get_pixel(0, 0), Some(Color::new(0, 0, 0)));
    }

    #[test]
    fn test_framebuffer_clear() {
        let mut fb = Framebuffer::new(800, 600);
        fb.set_background_color(Color::new(255, 0, 0));
        fb.clear();
        assert_eq!(fb.get_pixel(100, 100), Some(Color::new(255, 0, 0)));
    }

    #[test]
    fn test_framebuffer_point() {
        let mut fb = Framebuffer::new(800, 600);
        fb.set_current_color(Color::new(0, 255, 0));
        fb.point(200, 300);
        assert_eq!(fb.get_pixel(200, 300), Some(Color::new(0, 255, 0)));
    }

    #[test]
    fn test_framebuffer_point_out_of_bounds() {
        let mut fb = Framebuffer::new(800, 600);
        fb.set_current_color(Color::new(0, 255, 0));
        fb.point(-100, 1000);
        assert_eq!(fb.get_pixel(-100, 1000), None);
    }
}
