use macroquad::prelude::*;

// 1. LAS ESTRUCTURAS (Paredes de obstáculos y la Luz)
struct Wall {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[macroquad::main("AXIOM SHADOW CORE // DYNAMIC LIGHTS")]
async fn main() {
    // Colocamos unos bloques/columnas en el escenario para que tapen la luz
    let walls = vec![
        Wall { x: 150.0, y: 150.0, w: 100.0, h: 100.0 },
        Wall { x: 500.0, y: 100.0, w: 120.0, h: 80.0 },
        Wall { x: 250.0, y: 400.0, w: 80.0, h: 150.0 },
        Wall { x: 600.0, y: 350.0, w: 100.0, h: 100.0 },
    ];

    loop {
        // Fondo oscuro de búnker impenetrable
        clear_background(Color::new(0.01, 0.01, 0.02, 1.0));

        // La posición de la luz sigue al ratón del usuario de forma nativa
        let (light_x, light_y) = mouse_position();

        // 🔥 EL MOTOR DE RAYCASTING (Luz matemática de 360 grados)
        // Lanzamos "rayos de luz" invisibles en todas las direcciones para dibujar las sombras
        let num_rays = 360;
        for i in 0..num_rays {
            let angle = (i as f32).to_radians();
            let ray_dir_x = angle.cos();
            let ray_dir_y = angle.sin();

            let mut max_dist = 1200.0; // Alcance máximo del haz de luz

            // Comprobamos si el rayo choca con alguna pared para cortar la luz y crear la sombra
            for wall in &walls {
                // Algoritmo matemático ultra-rápido de intersección de rayos en cajas (AABB)
                let t1 = (wall.x - light_x) / ray_dir_x;
                let t2 = (wall.x + wall.w - light_x) / ray_dir_x;
                let t3 = (wall.y - light_y) / ray_dir_y;
                let t4 = (wall.y + wall.h - light_y) / ray_dir_y;

                let tmin = t1.min(t2).max(t3.min(t4));
                let tmax = t1.max(t2).min(t3.max(t4));

                if tmax >= tmin && tmin > 0.0 && tmin < max_dist {
                    max_dist = tmin; // El rayo choca, la luz se detiene aquí
                }
            }

            // Dibujamos el haz de luz debilitándose con la distancia (Efecto Linterna Neón)
            let end_x = light_x + ray_dir_x * max_dist;
            let end_y = light_y + ray_dir_y * max_dist;
            
            draw_line(
                light_x, 
                light_y, 
                end_x, 
                end_y, 
                1.5, 
                Color::new(1.0, 0.9, 0.4, (1.0 - (max_dist / 1200.0)).powf(2.0) * 0.15)
            );
        }

        // Dibujamos los obstáculos de piedra física en el mapa
        for wall in &walls {
            draw_rectangle(wall.x, wall.y, wall.w, wall.h, Color::new(0.1, 0.1, 0.15, 1.0));
            draw_rectangle_lines(wall.x, wall.y, wall.w, wall.h, 2.0, CYAN); // Bordes neón Axiom
        }

        // El núcleo brillante de la luz en el ratón
        draw_circle(light_x, light_y, 6.0, WHITE);
        draw_circle_lines(light_x, light_y, 12.0, 1.5, YELLOW);

        // Telemetría Soberana
        draw_rectangle(10.0, 10.0, 310.0, 80.0, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 30.0, 20.0, GREEN);
        draw_text("RENDER: 360° RAYCAST SHADOWS", 20.0, 50.0, 14.0, MAGENTA);
        draw_text("MOVE MOUSE TO CORE LIGHT", 20.0, 70.0, 14.0, YELLOW);

        next_frame().await
    }
}
