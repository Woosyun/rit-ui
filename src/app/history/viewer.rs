use std::collections::{ HashMap, BinaryHeap};
use rit::{
    commands::history::HistoryGraph,
    repository::Oid,
};
use leptos::prelude::*;

#[derive(Clone)]
pub struct HistoryViewer {
    nodes: HashMap<Oid, Node>,
}
impl HistoryViewer {
    pub fn from(hg: &HistoryGraph) -> Self {
        let mut bts_que = BinaryHeap::new();
        for (branch_name, branch) in hg.branches() {
            let (ctime, oid, parent_oid, msg) = hg.commits()
                .get(branch.leaf())
                .map(|commit| (commit.ctime(), branch.leaf(), commit.parents().get(0).unwrap(), commit.message()))
                .unwrap();
            let temp = BranchToSort::from(branch_name, ctime, oid, parent_oid, msg);
            bts_que.push(temp);
        }

        let mut hr = HistoryReader::new();
        let mut nodes = HashMap::new();
        while let Some(BranchToSort{name, oid, parent_oid, message, ..}) = bts_que.pop() {
            //create new node and store
            let new_node = hr.create_node(name, message);
            nodes.insert(oid.clone(), new_node);

            //if parent commit is same as branch -> root, finish
            if name != "main" && parent_oid == hg.branches().get(name).unwrap().root() {
                //root is on a other branch
                continue;
            }

            //get parent commit
            let (ctime, oid, parent_oid, msg) = hg.commits()
                .get(parent_oid)
                .map(|commit| (commit.ctime(), parent_oid, commit.parents().get(0).unwrap(), commit.message()))
                .unwrap();

            //add bts to queue
            bts_que.push(BranchToSort::from(name, ctime, oid, parent_oid, msg));
        }

        Self {
            nodes: HashMap::new(),
        }
    }
}

impl IntoRender for HistoryViewer {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
        view! {
            <h1>"hello world"</h1>
        }.into_any()
    }
}

#[derive(PartialEq, Eq)]
struct BranchToSort<'a> {
    name: &'a str,
    ctime: u64,
    oid: &'a Oid,
    parent_oid: &'a Oid,
    message: &'a str,
}
impl<'a> BranchToSort<'a> {
    fn from(name: &'a str, ctime: u64, oid: &'a Oid, parent_oid: &'a Oid, message: &'a str) -> Self {
        Self {
            name,
            ctime,
            oid,
            parent_oid,
            message,
        }
    }
}
impl<'a> PartialOrd for BranchToSort<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.ctime.cmp(&self.ctime))
    }
}
impl<'a> Ord for BranchToSort<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.ctime.cmp(&self.ctime)
    }
}

#[derive(Default)]
struct HistoryReader {
    xs: HashMap<String, u32>,
    x: u32,
    y: u32,
}
impl HistoryReader {
    fn new() -> Self {
        Self::default()
    }
    fn gap() -> u32 {
        50
    }
    fn create_node(&mut self, branch_name: &str, commit_message: &str) -> Node {
        let x = if let Some(x) = self.xs.get(branch_name) {
            x
        } else {
            self.x += Self::gap();
            self.xs.insert(branch_name.to_string(), self.x);
            &self.x
        };
        let y = self.y;
        self.y += Self::gap();

        Node::from(*x, y, commit_message)
    }
}

#[derive(Clone)]
pub struct Node {
    x: u32,
    y: u32,
    commit_message: String,
}
impl Node {
    fn from(x: u32, y: u32, commit_message: &str) -> Self {
        Self {
            x,y,
            commit_message: commit_message.to_string(),
        }
    }
}
