use std::collections::HashMap;
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
