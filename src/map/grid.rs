use bevy::prelude::*;

#[derive(Clone,Copy,PartialEq,Debug)]
enum Cell{
    Empty,
    Wall,
}

#[derive(Resource)]
struct Map{
    width : usize,
    height : usize,
    grid : Vec<Cell>,
}

impl Map{
    fn new(width: usize, height : usize, grid: Vec<Cell>) -> Self {
        Self{
            width,
            height,
            grid,
        }
    }
}
