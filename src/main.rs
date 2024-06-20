use crate::vector::Vector3;

mod vector;

use Vector3 as Color;
fn write_color(color: Color) {
    let normalized_r: f32 = color.x();
    let normalized_g : f32 = color.y();
    let normalized_b : f32 = color.z();

    let r: i32 = (255.99999 * normalized_r) as i32;
    let g: i32 = (255.999 * normalized_g) as i32;
    let b: i32 = (255.999 * normalized_b) as i32;

    println!("{} {} {}", r, g, b);
}

fn main() {
    const IMAGE_HEIGHT: i32 = 256;
    const IMAGE_WIDTH: i32 = 256;
    println!("P3");
    println!("{} {}", &IMAGE_HEIGHT, &IMAGE_WIDTH);
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Rendering.... {} Lines Remaining", &IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let color: Color = Color(i as f32 / (IMAGE_WIDTH - 1) as f32, j as f32 / (IMAGE_HEIGHT - 1) as f32, 0.0);
            write_color(color);
            // TODO: 4. RAYS!!!
        }
    }
    eprintln!("Rendering finished!");
}
