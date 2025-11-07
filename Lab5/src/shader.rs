// shaders.rs - Versión con colores

use nalgebra_glm::{Vec3, Vec4, Mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Transform position
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    let transformed = uniforms.model_matrix * position;

    // Perform perspective division
    let w = transformed.w;
    let transformed_position = Vec3::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w
    );

    // Transform normal
    let model_mat3 = Mat3::new(
        uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
        uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
        uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
    );
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());
    
    let transformed_normal = normal_matrix * vertex.normal;

    // Create a new Vertex with transformed attributes
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal,
    }
}

// NUEVO: Fragment shader para aplicar colores y efectos
pub fn fragment_shader(fragment: &Fragment, light_dir: &Vec3, normal: &Vec3) -> Color {
    // Iluminación difusa
    let normal = normal.normalize();
    let intensity = nalgebra_glm::dot(normal, light_dir).max(0.0);
    
    // Aplica la iluminación al color del fragmento
    fragment.color * intensity
}

// NUEVO: Función para calcular color basado en posición (efecto gradiente)
pub fn color_from_position(position: &Vec3) -> Color {
    // Gradiente basado en altura (eje Y)
    let t = (position.y + 1.0) * 0.5; // Normaliza entre 0 y 1
    
    // Gradiente de azul oscuro a azul claro
    let r = (50.0 + t * 100.0) as u8;
    let g = (100.0 + t * 100.0) as u8;
    let b = (200.0 + t * 55.0) as u8;
    
    Color::new(r, g, b)
}

// NUEVO: Colores metálicos con variación
pub fn metallic_shader(position: &Vec3, normal: &Vec3, light_dir: &Vec3) -> Color {
    let normal = normal.normalize();
    let intensity = nalgebra_glm::dot(normal, light_dir).max(0.0);
    
    // Color base metálico (plateado/gris)
    let base = Color::new(180, 190, 200);
    
    // Componente especular (brillo)
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let reflect_dir = 2.0 * nalgebra_glm::dot(normal, light_dir) * normal - light_dir;
    let specular = nalgebra_glm::dot(&reflect_dir, &view_dir).max(0.0).powf(32.0);
    
    // Combina difuso + especular
    let diffuse = base * intensity;
    let specular_color = Color::new(255, 255, 255) * specular * 0.5;
    
    diffuse + specular_color
}

// NUEVO: Shader de nave espacial (azul metálico con detalles)
pub fn spaceship_shader(position: &Vec3, normal: &Vec3, light_dir: &Vec3) -> Color {
    let normal = normal.normalize();
    let intensity = nalgebra_glm::dot(normal, light_dir).max(0.0);
    
    // Color base: azul metálico
    let base_color = Color::new(40, 80, 150);
    
    // Añade variación basada en la posición para simular paneles
    let panel_variation = ((position.x * 5.0).sin() * (position.y * 5.0).cos()).abs();
    let panel_color = if panel_variation > 0.7 {
        Color::new(60, 100, 170) // Paneles más claros
    } else {
        base_color
    };
    
    // Aplica iluminación
    let lit_color = panel_color * intensity;
    
    // Añade brillo especular en los bordes
    let rim_light = (1.0 - intensity).powf(3.0) * 0.3;
    let rim_color = Color::new(100, 150, 255) * rim_light;
    
    lit_color + rim_color
}

// NUEVO: Shader de nave de guerra (rojo/naranja)
pub fn warship_shader(position: &Vec3, normal: &Vec3, light_dir: &Vec3) -> Color {
    let normal = normal.normalize();
    let intensity = nalgebra_glm::dot(normal, light_dir).max(0.0);
    
    // Color base: rojo oscuro metálico
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

// NUEVO: Shader futurista (cyan/magenta)
pub fn futuristic_shader(position: &Vec3, normal: &Vec3, light_dir: &Vec3) -> Color {
    let normal = normal.normalize();
    let intensity = nalgebra_glm::dot(normal, light_dir).max(0.0);
    
    // Gradiente cyan a magenta
    let t = (position.y.sin() * 0.5 + 0.5).abs();
    
    let r = (t * 200.0 + 50.0) as u8;
    let g = (100.0) as u8;
    let b = ((1.0 - t) * 200.0 + 50.0) as u8;
    
    Color::new(r, g, b) * intensity
}