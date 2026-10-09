use std::collections::VecDeque;
use super::grid::*;
use crate::config::MINIMAL_ROOM_SIZE;

pub fn find_regions(map : &Map) -> Vec<Vec<(i32, i32)>> {
    let (width, height) = (map.width as i32, map.height as i32);
    let mut visited = vec![false; map.width * map.height];
    let mut res = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if visited[map.index(x, y)] { continue; }
            if let Some(cell) = map.get(x,y) {
                if cell == CellType::Empty {
                    res.push(collect_region(map, &mut visited, (x,y)));
                }      
            }
        }
    }
    res
}

fn collect_region(map : &Map, visited :&mut Vec<bool>, point: (i32,i32)) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    let mut quque = VecDeque::new();
    quque.push_back(point);
    visited[map.index(point.0, point.1)] = true;
    let neighbors = [(-1,0),(0,-1),(0,1),(1,0)];
    while let Some(current) = quque.pop_front() {
 
        for (dx,dy) in neighbors {
            if let Some(cell) = map.get(current.0 - dx, current.1 - dy){
                if visited[map.index(current.0 - dx,current.1 - dy)] { continue; } 
                if cell == CellType::Empty { 
                    quque.push_back((current.0 - dx, current.1 - dy));
                    visited[map.index(current.0 -dx, current.1 - dy)] = true;      
                }
            }
        }
        res.push(current);
    }
    res
}

fn keep_rooms(map :&mut Map) -> Vec<Vec<(i32,i32)>> {
    let mut regions = find_regions(map);
    let mut res = Vec::new();
    while let Some(region) = regions.pop() {
        if region.len() < MINIMAL_ROOM_SIZE as usize {
            for (x,y) in region {
                map.set(x,y, CellType::Wall);
            }
            continue;
        }
        res.push(region);
    }
    res
}
