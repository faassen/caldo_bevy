use bevy::prelude::*;
use std::collections::HashMap;

mod data;
use data::{Cell, Instr, Processor};

struct Position {
    x: u64,
    y: u64,
}

struct Name {
    name: String,
}

struct PositionMap {
    map: HashMap<(u64, u64), [Option<Entity>; 4]>,
}

impl PositionMap {
    pub fn new() -> PositionMap {
        PositionMap {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, entity: Entity, position: (u64, u64)) {
        let (x, y) = position;
        if y > 0 {
            let north = self.map.entry((x, y - 1)).or_insert([None; 4]);
            north[2] = Some(entity);
        }
        if x < u64::MAX {
            let east = self.map.entry((x + 1, y)).or_insert([None; 4]);
            east[3] = Some(entity);
        }
        if y < u64::MAX {
            let south = self.map.entry((x, y + 1)).or_insert([None; 4]);
            south[0] = Some(entity);
        }
        if x > 0 {
            let west = self.map.entry((x - 1, y)).or_insert([None; 4]);
            west[1] = Some(entity);
        }
    }

    fn remove(&mut self, position: (u64, u64)) {
        let (x, y) = position;
        if y > 0 {
            let north = self.map.entry((x, y - 1)).or_insert([None; 4]);
            north[2] = None;
        }
        if x < u64::MAX {
            let east = self.map.entry((x + 1, y)).or_insert([None; 4]);
            east[3] = None;
        }
        if y < u64::MAX {
            let south = self.map.entry((x, y + 1)).or_insert([None; 4]);
            south[0] = None;
        }
        if x > 0 {
            let west = self.map.entry((x - 1, y)).or_insert([None; 4]);
            west[1] = None;
        }
    }

    fn get_neighbor(&self, position: (u64, u64), direction: usize) -> Option<Entity> {
        self.get_neighbors(position)[direction]
    }

    fn get_neighbors(&self, position: (u64, u64)) -> &[Option<Entity>; 4] {
        self.map.get(&(position)).unwrap_or_else(|| &[None; 4])
    }
}

// we could detect whether position has changed
// if position changes, we update the neighbors components of these
// places

//      A
//      B C
//    E D

fn setup_entities(commands: &mut Commands) {
    commands.spawn((Position { x: 10, y: 10 }, Name { name: "A".into() }));
    commands.spawn((Position { x: 10, y: 11 }, Name { name: "B".into() }));
    commands.spawn((Position { x: 11, y: 11 }, Name { name: "C".into() }));
    commands.spawn((Position { x: 10, y: 12 }, Name { name: "D".into() }));
    commands.spawn((Position { x: 9, y: 12 }, Name { name: "E".into() }));
}

fn update_position_map(mut position_map: ResMut<PositionMap>, query: Query<(Entity, &Position)>) {
    for (entity, position) in query.iter() {
        position_map.add(entity, (position.x, position.y));
    }
}

fn print_neighbor_system(position_map: Res<PositionMap>, query: Query<(Entity, &Name, &Position)>) {
    println!("Print neighbor system");
    for (entity, name, position) in query.iter() {
        println!(
            "entity {:?}, name {} position: {:?} {:?}",
            entity, name.name, position.x, position.y
        );
        println!(
            "Neighbor entities {:?}",
            position_map.get_neighbors((position.x, position.y))
        )
    }
}

// to efficiently render part of a huge world we need a good
// space partitioning system
// can this be used to make neighborhood checks more efficient too?

// we can keep track of which things are in which partition by
// tracking position changes, but can see before the changes then?

#[bevy_main]
fn main() {
    App::build()
        .add_resource(PositionMap::new())
        .add_startup_system(setup_entities.system())
        .add_startup_stage_after(stage::STARTUP, "finalize_startup", SystemStage::parallel())
        .add_startup_system_to_stage("finalize_startup", update_position_map.system())
        .add_system(print_neighbor_system.system())
        .run();
}
