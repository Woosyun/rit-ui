use std::collections::{HashSet, HashMap, BinaryHeap};
use rit::{
    commands::history::HistoryGraph,
    repository::{Oid, Commit},
};

#[derive(Clone, Debug)]
pub struct HistoryViewer<'a> {
    pub nodes: HashMap<Oid, Node>,
    pub edges: HashMap<&'a Oid, HashSet<&'a Oid>>,
}

//todo: because of workspace node, parent -> children workflow is needed
//1. Assign xs to nodes base on branch->root->commit->ctime. Remember roots?
//2. Assign ys to nodes base on commit->ctime

//todo: find workspace node

impl<'a> HistoryViewer<'a> {
    pub fn from(hg: &'a HistoryGraph) -> Self {
        let mut edges: HashMap<&'a Oid, HashSet<&'a Oid>> = HashMap::new();
        let mut nodes: HashMap<Oid, Node> = HashMap::new();

        let mut branches: HashMap<&Oid, &str> = HashMap::new();
        let mut roots: HashSet<&Oid> = HashSet::new();
        //set oid->branch
        for (branch_name, branch) in hg.branches() {
            let mut next_oid = Some(branch.leaf());
            while let Some(oid) = next_oid {
                if oid == branch.root() && branch_name != "main" {
                    roots.insert(oid);
                    break;
                }

                branches.insert(oid, branch_name);
                next_oid = hg.commits()
                    .get(oid).unwrap()
                    .parents().get(0);
            }
        }

        //make edges(parent -> children)
        for (child, commit) in hg.commits() {
            for parent in commit.parents() {
                edges.entry(parent)
                    .or_default()
                    .insert(child);
            }
        }

        let mut que: BinaryHeap<BranchToSort> = BinaryHeap::new();
        //init
        for root in roots {
            let commit = hg.commits()
                .get(root).unwrap();
            let bts = BranchToSort::from(root, commit.ctime());
            que.push(bts);
        }
        //assign x and y to node
        let mut coord = Coordinator::new();
        while let Some(BranchToSort{oid, ..}) = que.pop() {
            if nodes.get(oid).is_some() {
                continue;
            }

            //create node
            let branch_name = branches.get(oid).unwrap();
            let (x, y) = coord.assign(branch_name);
            let commit = hg.commits().get(oid).unwrap();
            let node = Node::from(x, y, oid, commit);
            nodes.insert(oid.clone(), node);

            if let Some(child_oids) = edges.get(oid) {
                for child_oid in child_oids {
                    let child_ctime = hg.commits()
                        .get(*child_oid).unwrap()
                        .ctime();

                    let new_bts = BranchToSort::from(child_oid, child_ctime);
                    que.push(new_bts);
                }
            }
        }

        Self {
            nodes,
            edges,
        }
    }
    pub fn nodes(&self) -> &HashMap<Oid, Node> {
        &self.nodes
    }
}
struct Coordinator {
    xs: HashMap<String, u32>, // branch -> x
    max_x: u32,
    y: u32,
}
impl Coordinator {
    fn new() -> Self {
        Self {
            xs: HashMap::new(),
            max_x: 0,
            y: 0,
        }
    }
    fn assign(&mut self, branch_name: &str) -> (u32, u32) {
        let x = self.xs
            .entry(branch_name.to_string())
            .or_insert_with(|| {
                self.max_x += 1;
                self.max_x
            })
            .clone();
        self.y += 1;

        (x, self.y)
    }
}

struct BranchToSort<'a> {
    oid: &'a Oid,
    ctime: u64,
}
impl<'a> BranchToSort<'a> {
    fn from(oid: &'a Oid, ctime: u64) -> Self {
        Self {
            oid,
            ctime,
        }
    }
}
//todo: unimplement commit->eq
impl<'a> PartialEq for BranchToSort<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.ctime == other.ctime
    }
}
impl<'a> Eq for BranchToSort<'a> {}
impl<'a> PartialOrd for BranchToSort<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.ctime.cmp(&other.ctime))
    }
}
impl<'a> Ord for BranchToSort<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ctime.cmp(&other.ctime)
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub x: u32,
    pub y: u32,
    pub oid: Oid,
    pub commit: Commit,
}
impl Node {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            //todo: oid and commit?
        }
    }
    fn from(x: u32, y: u32, oid: &Oid, commit: &Commit) -> Self {
        Self {
            x,y,
            oid: oid.clone(),
            commit: commit.clone(),
        }
    }
}
