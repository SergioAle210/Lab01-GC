use crate::color::Color;
use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub trait Line {
    fn line(&mut self, start: Vec3, end: Vec3, color: Color);
    fn draw_polygon(&mut self, vertices: &[Vec3], border_color: Color, fill_color: Color);
    fn draw_polygon_with_hole(
        &mut self,
        outer_vertices: &[Vec3],
        hole_vertices: &[Vec3],
        border_color: Color,
        fill_color: Color,
        background_color: Color,
    );
    fn fill_polygon(&mut self, vertices: &[Vec3], fill_color: Color);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vec3, end: Vec3, color: Color) {
        let x1 = start.x as i32;
        let y1 = start.y as i32;
        let x2 = end.x as i32;
        let y2 = end.y as i32;

        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;

        self.set_current_color(color);

        loop {
            self.point(x, y);

            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_polygon(&mut self, vertices: &[Vec3], border_color: Color, fill_color: Color) {
        if vertices.len() < 2 {
            return;
        }

        // Dibuja las líneas del polígono en el color del borde
        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = vertices[(i + 1) % vertices.len()]; // Conecta el último vértice con el primero
            self.line(start, end, border_color.clone());
        }

        // Rellena el polígono con el color de relleno
        self.fill_polygon(vertices, fill_color);
    }

    fn draw_polygon_with_hole(
        &mut self,
        outer_vertices: &[Vec3],
        hole_vertices: &[Vec3],
        border_color: Color,
        fill_color: Color,
        background_color: Color,
    ) {
        if outer_vertices.len() < 2 {
            return;
        }

        // Dibuja las líneas del polígono exterior en el color del borde
        for i in 0..outer_vertices.len() {
            let start = outer_vertices[i];
            let end = outer_vertices[(i + 1) % outer_vertices.len()];
            self.line(start, end, border_color.clone());
        }

        // Rellena el polígono exterior con el color de relleno
        self.fill_polygon(outer_vertices, fill_color);

        // Dibuja las líneas del polígono del agujero en el color del borde
        for i in 0..hole_vertices.len() {
            let start = hole_vertices[i];
            let end = hole_vertices[(i + 1) % hole_vertices.len()];
            self.line(start, end, border_color.clone());
        }

        // Rellena el polígono del agujero con el color de fondo
        self.fill_polygon(hole_vertices, background_color);
    }

    fn fill_polygon(&mut self, vertices: &[Vec3], fill_color: Color) {
        if vertices.len() < 3 {
            return;
        }

        let mut min_y = vertices[0].y;
        let mut max_y = vertices[0].y;
        for vertex in vertices.iter().skip(1) {
            if vertex.y < min_y {
                min_y = vertex.y;
            }
            if vertex.y > max_y {
                max_y = vertex.y;
            }
        }

        self.set_current_color(fill_color);

        for y in min_y as i32..=max_y as i32 {
            let mut intersections = Vec::new();

            for i in 0..vertices.len() {
                let v1 = vertices[i];
                let v2 = vertices[(i + 1) % vertices.len()];

                let mut x1 = v1.x;
                let mut y1 = v1.y;
                let mut x2 = v2.x;
                let mut y2 = v2.y;

                if y1 > y2 {
                    std::mem::swap(&mut x1, &mut x2);
                    std::mem::swap(&mut y1, &mut y2);
                }

                if y >= y1 as i32 && y < y2 as i32 {
                    let x_intersect = x1 + (y as f32 - y1) * (x2 - x1) / (y2 - y1);
                    intersections.push(x_intersect);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x1 = intersections[i].round() as i32;
                    let x2 = intersections[i + 1].round() as i32;
                    for x in x1..=x2 {
                        self.point(x, y);
                    }
                }
            }
        }
    }
}
