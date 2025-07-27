use std::collections::HashMap;
use rit::{
    commands::history::HistoryGraph,
    repository::Oid,
};

#[derive(Clone, Debug)]
pub struct Coordinates(HashMap<Oid, (usize, usize)>);
impl Coordinates {
    pub fn from(hg: &HistoryGraph) -> Self {
        let mut nodes = Coordinates(HashMap::new());
        let (mut next_x, init_y) = Coordinates::init();
        for (_, leaf) in hg.leaves() {
            let tmp = nodes.recursively_assign_coordinate(leaf, (next_x, init_y), hg);
            next_x = std::cmp::max(
                next_x + Self::gap(),
                tmp
            );
        }
        nodes
    }
    pub fn gap() -> usize {
        50
    }
    pub fn init() -> (usize, usize) {
        (50, 50)
    }
    fn recursively_assign_coordinate(&mut self, child: &Oid, xy: (usize, usize), hg: &HistoryGraph) -> usize {
        self.insert(child.clone(), xy);

        let mut next_x = xy.0;
        if let Some(parents) = hg.parents().get(child) {
            for parent in parents {
                let tmp = self.recursively_assign_coordinate(parent, (next_x, xy.1+Self::gap()), hg);
                next_x = std::cmp::max(
                    next_x + Self::gap(), 
                    tmp
                );
            }
        }

        next_x
    }
}

impl std::ops::Deref for Coordinates {
    type Target = HashMap<Oid, (usize, usize)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Coordinates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
