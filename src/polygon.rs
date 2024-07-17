use nalgebra_glm::Vec3;
use crate::framebuffer::Framebuffer;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[Vec3]);
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, points: &[Vec3]) {
        if points.len() < 2 {
            return; // No se puede dibujar un polígono con menos de 2 puntos
        }

        for i in 0..points.len() {
            let start = points[i];
            let end = points[(i + 1) % points.len()];
            self.line(start, end);
        }
    }
}
