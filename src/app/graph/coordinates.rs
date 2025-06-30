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
        for root in hg.roots() {
            let tmp = nodes.grant(root, (next_x, init_y), hg);
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
    fn grant(&mut self, parent: &Oid, xy: (usize, usize), hg: &HistoryGraph) -> usize {
        self.insert(parent.clone(), xy);

        let mut next_x = xy.0;
        if let Some(children) = hg.children().get(parent) {
            for child in children {
                let tmp = self.grant(child, (next_x, xy.1+Self::gap()), hg);
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
