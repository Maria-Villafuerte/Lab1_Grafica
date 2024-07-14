use crate::framebuffer::Framebuffer;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[(isize, isize)]);
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, points: &[(isize, isize)]) {
        if points.len() < 2 {
            return; // No se puede dibujar un polígono con menos de 2 puntos
        }

        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % points.len()];
            self.line(x1, y1, x2, y2);
        }
    }
}
