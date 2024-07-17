mod bmp;
mod color;
mod framebuffer;
mod line;

use bmp::BmpWriter;
use color::Color;
use framebuffer::Framebuffer;
use line::Line;
use nalgebra_glm::Vec3;

fn main() {
    let mut fb = Framebuffer::new(800, 600);

    draw_polygon1(&mut fb);

    fb.save_as_bmp("output.bmp").unwrap();
}

fn draw_polygon1(fb: &mut Framebuffer) {
    let vertices = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];

    let border_color = Color::new(255, 255, 255); // Blanco
    let fill_color = Color::new(255, 255, 0); // Amarillo

    fb.draw_polygon(&vertices, border_color.clone(), fill_color);

    // Dibujar contorno
    draw_polygon_border(fb, &vertices, border_color);
}

fn draw_polygon_border(fb: &mut Framebuffer, vertices: &[Vec3], border_color: Color) {
    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = vertices[(i + 1) % vertices.len()];
        fb.draw_line(
            start.x as i32,
            start.y as i32,
            end.x as i32,
            end.y as i32,
            border_color.clone(),
        );
    }
}
