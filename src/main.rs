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
    draw_polygon2(&mut fb);
    draw_polygon3(&mut fb);
    draw_polygon4(&mut fb);

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

fn draw_polygon2(fb: &mut Framebuffer) {
    let vertices = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0),
    ];

    let border_color = Color::new(255, 255, 255); // Blanco
    let fill_color = Color::new(0, 0, 255); // Azul

    fb.draw_polygon(&vertices, border_color.clone(), fill_color);

    // Dibujar contorno
    draw_polygon_border(fb, &vertices, border_color);
}

fn draw_polygon3(fb: &mut Framebuffer) {
    let vertices = vec![
        Vec3::new(377.0, 249.0, 0.0),
        Vec3::new(411.0, 197.0, 0.0),
        Vec3::new(436.0, 249.0, 0.0),
    ];

    let border_color = Color::new(255, 255, 255); // Blanco
    let fill_color = Color::new(255, 0, 0); // Rojo

    fb.draw_polygon(&vertices, border_color.clone(), fill_color);

    // Dibujar contorno
    draw_polygon_border(fb, &vertices, border_color);
}

fn draw_polygon4(fb: &mut Framebuffer) {
    let outer_vertices = vec![
        Vec3::new(413.0, 177.0, 0.0),
        Vec3::new(448.0, 159.0, 0.0),
        Vec3::new(502.0, 88.0, 0.0),
        Vec3::new(553.0, 53.0, 0.0),
        Vec3::new(535.0, 36.0, 0.0),
        Vec3::new(676.0, 37.0, 0.0),
        Vec3::new(660.0, 52.0, 0.0),
        Vec3::new(750.0, 145.0, 0.0),
        Vec3::new(761.0, 179.0, 0.0),
        Vec3::new(672.0, 192.0, 0.0),
        Vec3::new(659.0, 214.0, 0.0),
        Vec3::new(615.0, 214.0, 0.0),
        Vec3::new(632.0, 230.0, 0.0),
        Vec3::new(580.0, 230.0, 0.0),
        Vec3::new(597.0, 215.0, 0.0),
        Vec3::new(552.0, 214.0, 0.0),
        Vec3::new(517.0, 144.0, 0.0),
        Vec3::new(466.0, 180.0, 0.0),
    ];

    let hole_vertices = vec![
        Vec3::new(682.0, 175.0, 0.0),
        Vec3::new(708.0, 120.0, 0.0),
        Vec3::new(735.0, 148.0, 0.0),
        Vec3::new(739.0, 170.0, 0.0),
    ];

    let border_color = Color::new(255, 255, 255); // Blanco
    let fill_color = Color::new(0, 255, 0); // Verde
    let background_color = fb.background_color.clone(); // Color de fondo

    fb.draw_polygon_with_hole(
        &outer_vertices,
        &hole_vertices,
        border_color.clone(),
        fill_color,
        background_color,
    );

    // Dibujar contorno
    draw_polygon_border(fb, &outer_vertices, border_color.clone());
    draw_polygon_border(fb, &hole_vertices, border_color);
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
