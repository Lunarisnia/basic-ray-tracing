mod vector;

fn main() {
    const IMAGE_HEIGHT: i32 = 256;
    const IMAGE_WIDTH: i32 = 256;
    println!("P3");
    println!("{} {}", &IMAGE_HEIGHT, &IMAGE_WIDTH);
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Rendering.... {} Lines Remaining", &IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let normalized_r: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let normalized_g: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let normalized_b: f32 = 0.0;

            let r: i32 = (255.999 * normalized_r) as i32;
            let g: i32 = (255.999 * normalized_g) as i32;
            let b: i32 = (255.999 * normalized_b) as i32;
            print!("\n{} {} {}", r, g, b)
            // TODO: 3.1
        }
    }
    eprintln!("Rendering finished!");
}
