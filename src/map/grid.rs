use bevy::prelude::*;

#[derive(Clone,Copy,PartialEq,Debug)]
enum Cell{
    Empty,
    Wall,
}

#[derive(Resource)]
struct Map{
    pub width : usize,
    pub height : usize,
    pub mut grid : Vec<Cell>,
}

impl Map{
    fn new(width: usize, height : usize) -> Self {
        Self{
            width,
            height,
            grid: vec![Cell::Wall, width * height]
        }
    }
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }
    
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn in_bounds_test(){
        let map = Map::new(10,10);
        
        assert!(map.in_bounds(0,0));
        assert!(map.in_bounds(9,9));
        assert!(!map.in_bounds(-1,0));
        assert!(!map.in_bounds(0,-1));
        assert!(!map.in_bounds(10,0));
        assert!(!map.in_bounds(0,10));
    }
}
