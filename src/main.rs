// Archivo main.rs
mod color;
mod framebuffer;
mod polygon;

use color::Color;
use framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

fn main() {
    let white = Color::new(255, 255, 255);
    let yellow = Color::new(255, 255, 0);
    let blue = Color::new(0, 0, 255);
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255, 0);

    let mut fb = Framebuffer::new(800, 600);

    // Establecer el color de fondo a negro
    fb.set_background_color(Color::new(0, 0, 0));
    fb.clear(); // Limpiar el framebuffer con el color de fondo

    // Polígono 1
    let polygon1 = vec![
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

    // Polígono 2
    let polygon2 = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0),
    ];

    // Polígono 3
    let polygon3 = vec![
        Vec3::new(377.0, 249.0, 0.0),
        Vec3::new(411.0, 197.0, 0.0),
        Vec3::new(436.0, 249.0, 0.0),
    ];

    // Polígono 4
    let polygon4 = vec![
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

    // Polígono 5 (agujero dentro del polígono 4)
    let polygon5 = vec![
        Vec3::new(682.0, 175.0, 0.0),
        Vec3::new(708.0, 120.0, 0.0),
        Vec3::new(735.0, 148.0, 0.0),
        Vec3::new(739.0, 170.0, 0.0),
    ];

    draw_polygon(&mut fb, &polygon1, &[], yellow, white);
    draw_polygon(&mut fb, &polygon2, &[], blue, white);
    draw_polygon(&mut fb, &polygon3, &[], red, white);
    draw_polygon(&mut fb, &polygon4, &[polygon5], green, white);

    fb.render_buffer("out1.bmp").unwrap();
    println!("Framebuffer rendered to out.bmp");
}




fn draw_polygon(fb: &mut Framebuffer, vertices: &[Vec3], holes: &[Vec<Vec3>], fill_color: Color, line_color: Color) {
    // Verificar si el número de vértices es al menos 3 (mínimo requerido para un polígono)
    if vertices.len() < 3 {
        println!("Error: Se necesitan al menos 3 vértices para dibujar un polígono.");
        return;
    }

    // Establecer el color de línea y dibujar el contorno del polígono
    fb.set_current_color(line_color);
    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = vertices[(i + 1) % vertices.len()];
        fb.line(start, end);
    }

    // Dibujar el contorno de los agujeros
    for hole in holes {
        for i in 0..hole.len() {
            let start = hole[i];
            let end = hole[(i + 1) % hole.len()];
            fb.line(start, end);
        }
    }

    // Rellenar el polígono utilizando el algoritmo de escaneo de línea
    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min) as isize;
    let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max) as isize;

    fb.set_current_color(fill_color);

    for y in min_y..=max_y {
        let mut nodes = vec![];
        let mut j = vertices.len() - 1;

        for i in 0..vertices.len() {
            let vi = &vertices[i];
            let vj = &vertices[j];
            if (vi.y as isize > y && vj.y as isize <= y) || (vj.y as isize > y && vi.y as isize <= y) {
                let node_x = (vi.x + (y as f32 - vi.y) / (vj.y - vi.y) * (vj.x - vi.x)) as isize;
                nodes.push(node_x);
            }
            j = i;
        }

        nodes.sort();

        for k in (0..nodes.len()).step_by(2) {
            if k + 1 < nodes.len() {
                for x in nodes[k]..=nodes[k + 1] {
                    if !is_point_in_holes(x, y, holes) {
                        fb.point(x, y);
                    }
                }
            }
        }
    }
}

fn is_point_in_holes(x: isize, y: isize, holes: &[Vec<Vec3>]) -> bool {
    for hole in holes {
        let mut inside = false;
        let mut j = hole.len() - 1;
        for i in 0..hole.len() {
            let vi = &hole[i];
            let vj = &hole[j];
            if (vi.y as isize > y) != (vj.y as isize > y) && x < (vj.x as isize - vi.x as isize) * (y - vi.y as isize) / (vj.y as isize - vi.y as isize) + vi.x as isize {
                inside = !inside;
            }
            j = i;
        }
        if inside {
            return true;
        }
    }
    false
}
