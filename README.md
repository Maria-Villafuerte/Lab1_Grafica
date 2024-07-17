# Proyecto de Dibujo de Polígonos

Este proyecto consiste en un programa en Rust que dibuja varios polígonos en un framebuffer y genera una imagen BMP como salida. El proyecto utiliza las siguientes tecnologías y bibliotecas:

- Rust
- nalgebra_glm
- std::fs para la manipulación de archivos

## Descripción del Proyecto

El proyecto tiene como objetivo dibujar cinco polígonos en un framebuffer y renderizarlos en un archivo BMP llamado `out.bmp`. Los polígonos se dibujan en diferentes ramas, y cada rama añade un polígono adicional al framebuffer.

### Polígonos

1. **Polígono 1**: (165, 380) (185, 360) (180, 330) (207, 345) (233, 330) (230, 360) (250, 380) (220, 385) (205, 410) (193, 383)
2. **Polígono 2**: (321, 335) (288, 286) (339, 251) (374, 302)
3. **Polígono 3**: (377, 249) (411, 197) (436, 249)
4. **Polígono 4**: (413, 177) (448, 159) (502, 88) (553, 53) (535, 36) (676, 37) (660, 52) (750, 145) (761, 179) (672, 192) (659, 214) (615, 214) (632, 230) (580, 230) (597, 215) (552, 214) (517, 144) (466, 180)
5. **Polígono 5**: (682, 175) (708, 120) (735, 148) (739, 170) - Este polígono es un agujero dentro del Polígono 4 y no debe ser pintado.

## Estructura de Ramas (Branches)

Cada polígono se ha dibujado en una rama separada, y las ramas se han fusionado en la rama principal de configuración llamada `config`. A continuación, se describe el manejo de las ramas:

- **`Poligon-1`**: Dibuja solo el Polígono 1 (amarillo con orilla blanca).
- **`Poligon-2`**: Dibuja el Polígono 1 y el Polígono 2 (azul con orilla blanca).
- **`Poligon-3`**: Dibuja los Polígonos 1, 2 y 3 (rojo con orilla blanca).
- **`Poligon-4`**: Dibuja los Polígonos 1, 2, 3 y 4, con el Polígono 5 como un agujero en el Polígono 4 (verde con orilla blanca).
- **`config`**: Rama principal que contiene los merges de las ramas `Poligon-1`, `Poligon-2`, `Poligon-3` y `Poligon-4`.

## Instrucciones para Ejecutar el Proyecto

1. Clonar el repositorio:
   ```sh
   git clone https://github.com/tu_usuario/tu_repositorio.git
   cd tu_repositorio
