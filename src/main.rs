use bevy::prelude::*;
use crate::map::{generate::generate, grid::*,connect::*};
use std::time::{SystemTime, UNIX_EPOCH};
mod config;
mod map;
fn main() {
        App::new()
            .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_systems(Startup, startup)
            .run();
}

fn startup(mut commands : Commands){
    let mut map = Map::new(config::MAP_SIZE.0, config::MAP_SIZE.1);
    let seed : u32 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u32;
    println!("{}", seed);
    generate(&mut map,seed, 0.04, 0.5);    
    let rooms = keep_rooms(&mut map);
    commands.insert_resource(map);
    commands.spawn((
            Camera2d::default(),
            Transform::from_xyz(0.,0.,0.),
    ));
}

