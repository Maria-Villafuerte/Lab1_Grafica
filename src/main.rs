mod color;
mod framebuffer;
mod polygon;

use color::Color;
use framebuffer::Framebuffer;
use polygon::Polygon;

fn main() {
    let black = Color::new(0, 0, 0);
    let white = Color::new(255, 255, 255);
    let blue = Color::new(0, 0, 255);
    let yellow = Color::new(255, 255, 0);


    let mut fb = Framebuffer::new(800, 600);

    // Establecer el color de fondo a blanco
    fb.set_background_color(white);
    fb.clear(); // Limpiar el framebuffer con el color de fondo

    // Establecer el color actual a negro para dibujar el triángulo
    fb.set_current_color(yellow);

    // Dibujar un triángulo negro
    let points = vec![
        (165, 380),
        (185, 360), 
        (180, 330), 
        (207, 345), 
        (233, 330), 
        (230, 360),
         (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    // fb.set_current_color(blue);
    // let points = vec![
    //     (321, 335),
    //     (288, 286),
    //     (339, 251), 
    //     (374, 302)
    // ];




    fb.draw_polygon(&points);

    fb.render_buffer("output.bmp").unwrap();
    println!("Framebuffer rendered to output.bmp");
}


    
    /* 
    COMO CREAR PUNTOS 
    
    fb.set_current_color(red);
    fb.point(10, 8);

    fb.set_current_color(green);
    fb.point(19, 8);

    fb.set_current_color(blue);
    fb.point(8, 5);   
     */

    /*
    COMO CREAR LÍNEAS 
    fb.line(x1, y1, x0, y1); // Línea inferior
    fb.line(x0, y1, x0, y0); // Línea izquierda
     */
