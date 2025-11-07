use nalgebra_glm::{Vec3, dot};
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::line::line;
use crate::color::Color;

pub fn _triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    
    // Draw the three sides of the triangle
    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v3, v1));
    
    fragments
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);
    
    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);
    
    let light_dir = Vec3::new(0.0, 0.0, -1.0);
    
    let triangle_area = edge_function(&a, &b, &c);
    
    // Iterate over each pixel in the bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            
            // Calculate barycentric coordinates
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);
            
            // Check if the point is inside the triangle
            if w1 >= 0.0 && w1 <= 1.0 &&
               w2 >= 0.0 && w2 <= 1.0 &&
               w3 >= 0.0 && w3 <= 1.0 {
                
                // Interpolate normal
                let normal = v1.transformed_normal * w1 + v2.transformed_normal * w2 + v3.transformed_normal * w3;
                let normal = normal.normalize();
                
                // Interpolate position for shader effects
                let world_pos = v1.position * w1 + v2.position * w2 + v3.position * w3;
                
                // Calculate lighting intensity
                let intensity = dot(&normal, &light_dir).max(0.0);
                
                // === ELIGE UNO DE ESTOS SHADERS ===
                
                // 1. Color uniforme simple
                // let base_color = Color::new(40, 80, 150); // Azul espacial
                
                // 2. Shader metálico con brillo
                // let base_color = metallic_color(&normal, &light_dir, intensity);
                
                // 3. Shader de nave espacial (azul con paneles)
                let base_color = spaceship_color(&world_pos, &normal, &light_dir, intensity);
                
                // 4. Shader de nave de guerra (rojo)
                // let base_color = warship_color(&world_pos, intensity);
                
                // 5. Shader futurista (cyan/magenta)
                // let base_color = futuristic_color(&world_pos, intensity);
                
                // Interpolate depth
                let depth = a.z * w1 + b.z * w2 + c.z * w3;
                
                fragments.push(Fragment::new(x as f32, y as f32, base_color, depth));
            }
        }
    }
    
    fragments
}

// === FUNCIONES DE SHADER ===

// Shader 1: Metálico simple
fn metallic_color(normal: &Vec3, light_dir: &Vec3, intensity: f32) -> Color {
    let base = Color::new(180, 190, 200); // Plateado
    
    // Brillo especular
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let reflect_dir = 2.0 * dot(normal, light_dir) * normal - light_dir;
    let specular = dot(&reflect_dir, &view_dir).max(0.0).powf(32.0);
    
    let diffuse = base * intensity;
    let specular_color = Color::new(255, 255, 255) * specular * 0.5;
    
    diffuse + specular_color
}

// Shader 2: Nave espacial con paneles
fn spaceship_color(position: &Vec3, _normal: &Vec3, _light_dir: &Vec3, intensity: f32) -> Color {
    // Color base: dorado metálico
    let base_color = Color::new(200, 170, 50);
    
    // Añade variación para simular paneles
    let panel_variation = ((position.x * 5.0).sin() * (position.y * 5.0).cos()).abs();
    let panel_color = if panel_variation > 0.7 {
        Color::new(220, 190, 70) // Paneles más claros/brillantes
    } else {
        base_color
    };
    
    // Aplica iluminación
    let lit_color = panel_color * intensity;
    
    // Sin rim lighting - solo devuelve el color iluminado
    lit_color
}

// Shader 3: Nave de guerra (rojo/naranja)
fn warship_color(position: &Vec3, intensity: f32) -> Color {
    let base_color = Color::new(150, 30, 30);
    
    // Detalles naranjas en ciertas partes
    let detail = ((position.z * 3.0).sin() * 0.5 + 0.5).abs();
    let color = if detail > 0.8 {
        Color::new(200, 80, 20) // Detalles naranjas
    } else {
        base_color
    };
    
    color * intensity
}

// Shader 4: Futurista (cyan/magenta)
fn futuristic_color(position: &Vec3, intensity: f32) -> Color {
    // Gradiente basado en posición
    let t = (position.y.sin() * 0.5 + 0.5).abs();
    
    let r = (t * 200.0 + 50.0) as u8;
    let g = 100;
    let b = ((1.0 - t) * 200.0 + 50.0) as u8;
    
    Color::new(r, g, b) * intensity
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;
    
    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;
    
    (w1, w2, w3)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}