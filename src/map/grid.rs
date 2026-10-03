use bevy::prelude::*;

#[derive(Clone,Copy,PartialEq,Debug)]
pub enum CellType{
    Empty,
    Wall,
}

#[derive(Resource)]
pub struct Map{
    pub width : usize,
    pub height : usize,
    grid : Vec<CellType>,
}

impl Map{
    fn new(width: usize, height : usize) -> Self {
        Self{
            width,
            height,
            grid: vec![CellType::Wall; width * height]
        }
    }
    fn index(&self,x: i32, y:i32) -> usize {
        y as usize * self.width + x as usize
    }
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }
    pub fn set(&mut self, x: i32, y:i32, cell: CellType) -> bool{
        if !self.in_bounds(x,y) { return false }
        let index =  self.index(x, y);
        self.grid[ index ] = cell;
        true
    } 
    pub fn get(&self, x: i32, y: i32) -> Option<CellType> { 
        if !self.in_bounds(x,y) { return None }
        Some(self.grid[self.index(x, y)])
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
    
    #[test]
    fn get_set_test(){
        let mut map = Map::new(10,10);

        assert!(!map.set(10,10, CellType::Empty ));
        assert!(map.set(1,1, CellType::Empty ));
        assert_eq!(map.get(1,1).unwrap(), CellType::Empty);
        assert_eq!(map.get(10,10), None)

    }
}
