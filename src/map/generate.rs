use super::grid::*;
use noise::{Perlin, NoiseFn};
pub fn generate(map : &mut Map, seed: u32, scale: f64, threshold : f64) {
    let noise = Perlin::new(seed);
    let (width,height) = (map.width as i32, map.height as i32);
    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                map.set(x,y, CellType::Wall);
                continue
            } 
            if (noise.get([x as f64 * scale,y as f64 * scale]) + 1.) / 2. > threshold 
            { map.set(x,y, CellType::Empty); } else { map.set(x,y, CellType::Wall); }
        }
    }

    
}
